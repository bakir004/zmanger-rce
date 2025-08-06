use std::result::Result;
use axum::{http::StatusCode, response::Response, response::IntoResponse};
use tokio::sync::oneshot;

use crate::{models::{submission::SubmissionJob, ExecutionResult}, SUBMISSION_QUEUE};

pub async fn add_to_queue(
    script: String,
) -> Result<ExecutionResult, Response> {
    let (tx, rx) = oneshot::channel();

    let job = SubmissionJob {
        bash_script: script, 
        response_tx: tx,
    };

    {
        let mut queue = SUBMISSION_QUEUE.lock().unwrap();
        queue.push_back(job);
    }

    match rx.await {
        Ok(result) => Ok(result),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to receive execution result").into_response()),
    }
}

