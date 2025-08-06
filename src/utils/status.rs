use crate::models::SubmissionStatus;

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

pub fn get_submission_status(
    stdout: &String,
    stderr: &String,
    runtime_status: i32,
    expected_outputs: Vec<String> 
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
    } else if expected_outputs
        .iter()
        .any(|expected| expected.trim() == stdout.trim())
    {
        SubmissionStatus::Accepted
    } else if expected_outputs
        .iter()
        .any(|expected| remove_whitespace(expected) == remove_whitespace(&stdout))
    {
        SubmissionStatus::CoreAccepted
    } else if runtime_status == 0 {
        SubmissionStatus::WrongAnswer
    } else {
        SubmissionStatus::RuntimeError
    };

    result_description
}
