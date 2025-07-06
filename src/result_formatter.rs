use crate::SubmissionResult;
use crate::models::SubmissionStatus;

pub fn create_result(
    compile_output: String,
    stdout: String,
    stderr: String,
    time: u64,
    runtime_status: i32,
    expected_output: Vec<String>,
) -> SubmissionResult {
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

    SubmissionResult {
        compile_output,
        stdout,
        stderr,
        time,
        runtime_status,
        submission_status: result_description as i32,
        description: result_description.name().to_string(),
    }
}

// pub fn create_error_result(
//     stderr: String,
// ) -> SubmissionResult {
//     SubmissionResult {
//         compile_output: "".to_string(),
//         stdout: "".to_string(),
//         stderr,
//         time: 0,
//         runtime_status: 420,
//         submission_status: SubmissionStatus::InternalError as i32,
//         description: SubmissionStatus::InternalError.name().to_string(),
//     }
// }
