use axum::{ 
    extract::Json, http::StatusCode, routing::post, Router
};
mod language_config;
use language_config::prepare_bash_script;
use language_config::get_language_config;
mod globals;
use globals::{DELIMITER, TIMEOUT};
mod result_formatter;
use models::throw;
use result_formatter::create_result;
mod models;
use models::{ErrorResponse, Submission, SubmissionResult};

use std::collections::VecDeque;
use std::sync::Mutex;
use lazy_static::lazy_static;

use std::process::{
    Command,
    Stdio
};
use std::io::Write;
use std::result::Result;
use tokio::sync::oneshot;

struct SubmissionJob {
    submission: Submission,
    response_tx: oneshot::Sender<SubmissionResult>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", post(run_code))
        .route("/queue", post(add_to_queue));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind TCP listener");

    let num_workers = num_cpus::get();
    for _ in 0..num_workers {
        tokio::spawn(worker_loop());
    }
    println!("Listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

lazy_static! {
    static ref SUBMISSION_QUEUE: Mutex<VecDeque<SubmissionJob>> = Mutex::new(VecDeque::new());
}

async fn add_to_queue(
    Json(payload): Json<Submission>,
) -> Result<Json<SubmissionResult>, (StatusCode, Json<ErrorResponse>)> {
    let sub = Submission {
        code: payload.code,
        stdin: payload.stdin,
        expected_output: payload.expected_output,
        language_id: payload.language_id,
    };

    let (tx, rx) = oneshot::channel();

    let job = SubmissionJob {
        submission: sub,
        response_tx: tx,
    };

    {
        let mut queue = SUBMISSION_QUEUE.lock().unwrap();
        queue.push_back(job);
        println!("Added job to queue, current queue size: {}", queue.len());
    }

    match rx.await {
        Ok(result) => Ok(Json(result)),
        Err(_) => throw(StatusCode::INTERNAL_SERVER_ERROR, "Worker dropped response channel"),
    }
}

use tokio::time::{sleep, Duration};

async fn worker_loop() {
    loop {
        let job_opt = {
            let mut queue = SUBMISSION_QUEUE.lock().unwrap();
            queue.pop_front()
        };

        if let Some(job) = job_opt {
            let result = run_code2(job.submission).await;

            let _ = job.response_tx.send(result.unwrap_or_else(|e| SubmissionResult {
                compile_output: "".to_string(),
                stdout: "".to_string(),
                stderr: format!("Error: {:?}", e),
                time: 0,
                runtime_status: -1,
                submission_status: -1,
                description: "Failed to process submission".to_string(),
            }));
        } else {
            sleep(Duration::from_millis(100)).await;
        }
    }
}

async fn run_code2(submission: Submission) -> Result<SubmissionResult, (StatusCode, ErrorResponse)> {
    let Submission {
        code,
        stdin,
        expected_output,
        language_id
    } = submission;

    let language_config = get_language_config(language_id).unwrap();

    let code = code.replace('\'', "'\\''");
    let stdin = stdin.replace('\n', "\n").replace('\'', "'\\''");
    let bash_script = prepare_bash_script(&code, &stdin, TIMEOUT, &language_config);
    let podman_arguments = [
        "run", 
        "--rm", 
        "-i", 
        "--cpus=1",
        "--security-opt", 
        "label=disable", 
        "--cap-add=SYS_PTRACE", 
        "--memory=512m", 
        "gcc:latest", 
        "bash"
    ];

    let mut child = Command::new("podman")
        .args(podman_arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start Podman container");

    let stdin = child.stdin.as_mut().unwrap();
    stdin.write_all(bash_script.as_bytes()).unwrap();

    let output = child.wait_with_output().expect("Failed to run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    // let stderr = String::from_utf8_lossy(&output.stderr);

    let results: Vec<&str> = stdout.split(&format!("{}\n", DELIMITER)).collect();

    let compile_output = results.get(0).unwrap_or(&"").to_string();
    let result_stdout = results.get(1).unwrap_or(&"").to_string();
    let result_stderr = results.get(2).unwrap_or(&"").to_string();
    let runtime_status = results.get(3).unwrap_or(&"0").trim().parse::<i32>().unwrap_or(0);
    let elapsed_us = results.get(4).unwrap_or(&"0").trim().parse::<u64>().unwrap_or(0);

    // if !stderr.is_empty() {
    //     return Err((
    //         StatusCode::INTERNAL_SERVER_ERROR,
    //         Json(ErrorResponse {
    //             message: format!("{} - Ensure Podman is installed and running.", stderr),
    //         }),
    //     ));
    // }

    let result = create_result(compile_output, result_stdout, result_stderr, elapsed_us, runtime_status, expected_output);
    Ok(result)
}

async fn run_code(Json(payload): Json<Submission>) -> Result<Json<SubmissionResult>, (StatusCode, Json<ErrorResponse>)> {
    let Submission {
        code,
        stdin,
        expected_output,
        language_id
    } = payload;

    if code.is_empty() {
        return throw(StatusCode::BAD_REQUEST, "Code cannot be empty");
    }
    if expected_output.is_empty() {
        return throw(StatusCode::BAD_REQUEST, "Expected output cannot be empty");
    }

    let language_config = get_language_config(language_id).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                message: e.to_string(),
            }),
        )
    })?;

    let code = code.replace('\'', "'\\''");
    let stdin = stdin.replace('\n', "\n").replace('\'', "'\\''");
    let bash_script = prepare_bash_script(&code, &stdin, TIMEOUT, &language_config);
    let podman_arguments = [
        "run", 
        "--rm", 
        "-i", 
        "--cpus=1",
        "--security-opt", 
        "label=disable", 
        "--cap-add=SYS_PTRACE", 
        "--memory=512m", 
        "gcc:latest", 
        "bash"
    ];

    let mut child = Command::new("podman")
        .args(podman_arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                message: format!("{} - Ensure Podman is installed and running.", e),
            }),
        ))?;

    let stdin = child.stdin.as_mut().unwrap();
    stdin.write_all(bash_script.as_bytes()).unwrap();

    let output = child.wait_with_output().expect("Failed to run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let results: Vec<&str> = stdout.split(&format!("{}\n", DELIMITER)).collect();

    let compile_output = results.get(0).unwrap_or(&"").to_string();
    let result_stdout = results.get(1).unwrap_or(&"").to_string();
    let result_stderr = results.get(2).unwrap_or(&"").to_string();
    let runtime_status = results.get(3).unwrap_or(&"0").trim().parse::<i32>().unwrap_or(0);
    let elapsed_us = results.get(4).unwrap_or(&"0").trim().parse::<u64>().unwrap_or(0);

    if !stderr.is_empty() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                message: format!("{} - Ensure Podman is installed and running.", stderr),
            }),
        ));
    }

    let result = create_result(compile_output, result_stdout, result_stderr, elapsed_us, runtime_status, expected_output);
    Ok(Json(result))
}
