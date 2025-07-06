use serde::Deserialize;
use serde::Serialize;
use axum::{
    http::StatusCode,
    extract::Json,
};
use std::result::Result;

#[derive(Deserialize, Debug, Clone)]
pub struct Submission {
    pub code: String,
    pub stdin: String,
    pub expected_output: Vec<String>,
    pub language_id: u8 
}

#[derive(Serialize)]
pub struct SubmissionResult {
    pub compile_output: String,
    pub stdout: String,
    pub stderr: String,
    pub time: u64,
    pub runtime_status: i32,
    pub submission_status: i32,
    pub description: String
}

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    pub message: String,
}

pub fn throw(code: StatusCode, message: &str) -> Result<Json<SubmissionResult>, (StatusCode, Json<ErrorResponse>)> {
    Err((
        code,
        Json(ErrorResponse {
            message: message.to_string(),
        }),
    ))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SubmissionStatus {
    Accepted,
    WrongAnswer,
    CompilationError,
    TimeLimitExceeded,
    MemoryLimitExceeded,
    MemoryLeakDetected,
    MemoryError,
    RuntimeError,
    // InternalError,
}

impl SubmissionStatus {
    pub fn name(&self) -> &'static str {
        match self {
            SubmissionStatus::Accepted => "Accepted",
            SubmissionStatus::WrongAnswer => "Wrong Answer",
            SubmissionStatus::TimeLimitExceeded => "Time Limit Exceeded",
            SubmissionStatus::MemoryLimitExceeded => "Memory Limit Exceeded",
            SubmissionStatus::CompilationError => "Compilation Error",
            SubmissionStatus::RuntimeError => "Runtime Error",
            SubmissionStatus::MemoryError => "Memory Error",
            // SubmissionStatus::InternalError => "Internal Server Error",
            SubmissionStatus::MemoryLeakDetected => "Memory Leak Detected",
        }
    }
}

