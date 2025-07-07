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
