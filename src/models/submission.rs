use serde::Deserialize;
use tokio::sync::oneshot;

use super::ExecutionResult;

#[derive(Deserialize, Clone)]
pub struct ISubmission {
    pub code: String,
    pub stdin: String,
    pub expected_output: Vec<String>,
    pub language_id: u8 
}

pub struct SubmissionJob {
    pub bash_script: String,
    pub response_tx: oneshot::Sender<ExecutionResult>,
}

