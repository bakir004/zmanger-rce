use serde::Serialize;

#[derive(Serialize)]
pub struct IExecutionResult {
    pub compile_output: String,
    pub stdout: String,
    pub stderr: String,
    pub time: u64,
    pub runtime_status: i32,
    pub submission_status: u8,
    pub description: String,
}

#[derive(Serialize)]
pub struct IExecutionError {
    pub status_code: u16,
    pub message: String,
}

#[derive(Serialize)]
pub struct ExecutionResult {
    pub compile_output: String,
    pub stdout: String,
    pub stderr: String,
    pub time: u64,
    pub runtime_status: i32,
}

#[derive(Serialize, Debug)]
pub struct ExecutionError {
    pub message: String,
}

#[derive(Serialize)]
pub struct IExecutionGroupResult {
    pub results: Vec<IExecutionGroupResultElement>,
}

#[derive(Serialize)]
pub struct IExecutionGroupResultElement {
    pub id: u64,
    pub result: IExecutionResult,
}

