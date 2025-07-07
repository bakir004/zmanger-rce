use std::result::Result;
use tokio::sync::oneshot;

use crate::{models::{submission::SubmissionJob, IExecutionError, ExecutionResult}, SUBMISSION_QUEUE};

pub async fn add_to_queue(
    script: String,
) -> Result<ExecutionResult, IExecutionError> {
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
        Err(_) => Err(IExecutionError {
            status_code: 500,
            message: "Failed to receive response from worker".to_string(),
        }),
    }
}

