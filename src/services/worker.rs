use tokio::time::{sleep, Duration};
use crate::{models::ExecutionResult, services::run::run, SUBMISSION_QUEUE};

pub async fn worker_loop() {
    loop {
        let job_opt = {
            let mut queue = SUBMISSION_QUEUE.lock().unwrap();
            queue.pop_front()
        };

        if let Some(job) = job_opt {
            let result = run(job.bash_script).await;

            let _ = job.response_tx.send(result.unwrap_or_else(|e| ExecutionResult {
                compile_output: "".to_string(),
                stdout: "".to_string(),
                stderr: format!("Error: {:?}", e),
                time: 0,
                runtime_status: -1,
            }));
        } else {
            sleep(Duration::from_millis(100)).await;
        }
    }
}
