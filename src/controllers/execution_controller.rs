use axum::response::{Response, IntoResponse};
use axum::{ extract::Json, http::StatusCode };
use crate::globals::TIMEOUT_IN_SECONDS;
use crate::models::execution::{IExecutionGroupResult, IExecutionGroupResultElement};
use crate::models::submission::ISubmissionGroupElement;
use crate::models::{ISubmission, IExecutionResult};
use crate::services::language::get_language_config;
use crate::services::bash::prepare_bash_script;
use crate::utils::status::get_submission_status;
use crate::services::queue::add_to_queue;

pub async fn submit_code(
    Json(payload): Json<ISubmission>,
) -> Result<Json<IExecutionResult>, Response> {
    let submission = ISubmission {
        code: payload.code,
        stdin: payload.stdin,
        expected_output: payload.expected_output,
        language_id: payload.language_id,
    };

    if submission.code.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Submission code cannot be empty").into_response());
    }

    if submission.expected_output.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "There must be at least one expected output, even if it is empty").into_response());
    }

    let language_config = get_language_config(submission.language_id)
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()).into_response())?;

    let stdin = submission.stdin.unwrap_or("".to_string());

    let escaped_code = submission.code.replace('\'', r#"'"'"'"#);
    let bash_script = prepare_bash_script(
        &escaped_code,
        &stdin,
        &language_config,
        TIMEOUT_IN_SECONDS,
    );

    let execution_result = add_to_queue(bash_script).await.map_err(|e| e)?;

    let submission_status = get_submission_status(
        &execution_result.stdout,
        &execution_result.stderr,
        execution_result.runtime_status,
        submission.expected_output
    );

    let response = IExecutionResult {
        compile_output: execution_result.compile_output,
        stdout: execution_result.stdout,
        stderr: execution_result.stderr,
        time: execution_result.time,
        runtime_status: execution_result.runtime_status,
        submission_status: submission_status as u8,
        description: submission_status.name().to_string(),
    };

    Ok(Json(response))
}

pub async fn submit_batch(
    Json(payload): Json<Vec<ISubmissionGroupElement>>,
) -> Result<Json<IExecutionGroupResult>, Response> {

    let submission_elements = payload;

    if submission_elements.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "There must be at least one submission in the batch").into_response());
    }
    for submission_element in &submission_elements {
        let submission = submission_element.submission.clone();
        if submission.code.is_empty() {
            return Err((StatusCode::BAD_REQUEST, "Submission code cannot be empty").into_response());
        }
        if submission.expected_output.is_empty() {
            return Err((StatusCode::BAD_REQUEST, "There must be at least one expected output, even if it is empty").into_response());
        }
        get_language_config(submission.language_id)
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()).into_response())?;
    }

    let mut response = IExecutionGroupResult {
        results: Vec::new(),
    };


    for submission_element in &submission_elements {
        let submission = submission_element.submission.clone();
        let language_config = get_language_config(submission.language_id)
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()).into_response())?;

        let stdin = submission.stdin.unwrap_or("".to_string());

        let escaped_code = submission.code.replace('\'', r#"'"'"'"#);
        let bash_script = prepare_bash_script(
            &escaped_code,
            &stdin,
            &language_config,
            TIMEOUT_IN_SECONDS,
        );

        let execution_result = add_to_queue(bash_script).await.map_err(|e| e)?;

        let submission_status = get_submission_status(
            &execution_result.stdout,
            &execution_result.stderr,
            execution_result.runtime_status,
            submission.expected_output
        );

        let execution_result_object = IExecutionResult {
            compile_output: execution_result.compile_output,
            stdout: execution_result.stdout,
            stderr: execution_result.stderr,
            time: execution_result.time,
            runtime_status: execution_result.runtime_status,
            submission_status: submission_status as u8,
            description: submission_status.name().to_string(),
        };

        let id = submission_element.id;

        let execution_group_result_element = IExecutionGroupResultElement {
            id,
            result: execution_result_object,
        };

        response.results.push(execution_group_result_element);
    }

    Ok(Json(response))
}
