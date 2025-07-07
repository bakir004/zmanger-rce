use crate::models::SubmissionStatus;

pub fn get_submission_status(
    stdout: &String,
    stderr: &String,
    runtime_status: i32,
    expected_output: Vec<String> 
) -> SubmissionStatus{
    let result_description = if runtime_status == 124 {
        SubmissionStatus::TimeLimitExceeded
    } else if runtime_status == 127 {
        SubmissionStatus::CompilationError
    } else if runtime_status == 1 && stderr.contains("out of memory") {
        SubmissionStatus::MemoryLimitExceeded
    } else if runtime_status == 1 && stderr.contains("LeakSanitizer") {
        SubmissionStatus::MemoryLeakDetected
    } else if runtime_status == 1 {
        SubmissionStatus::MemoryError
    } else if runtime_status == 0 && expected_output.contains(&stdout){
        SubmissionStatus::Accepted
    } else if runtime_status == 0 {
        SubmissionStatus::WrongAnswer
    } else {
        SubmissionStatus::RuntimeError
    };

    result_description
}
