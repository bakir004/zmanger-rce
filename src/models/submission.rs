use serde::Deserialize;
use tokio::sync::oneshot;

use super::ExecutionResult;

#[derive(Deserialize, Clone)]
pub struct ISubmission {
    pub code: String,
    #[serde(default)]
    pub stdin: Option<String>,
    pub expected_output: Vec<String>,
    pub language_id: u8 
}

pub struct SubmissionJob {
    pub bash_script: String,
    pub response_tx: oneshot::Sender<ExecutionResult>,
}

#[derive(Deserialize, Clone)]
pub struct ISubmissionGroupElement {
    pub id: u64,
    pub submission: ISubmission,
}


