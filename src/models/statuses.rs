#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SubmissionStatus {
    Accepted = 0,
    CoreAccepted = 1,
    WrongAnswer = 2,
    CompilationError = 3,
    TimeLimitExceeded = 4,
    MemoryLimitExceeded = 5,
    MemoryLeakDetected = 6,
    MemoryError = 7,
    RuntimeError = 8,
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
            SubmissionStatus::CoreAccepted => "Core Accepted",
            SubmissionStatus::MemoryLeakDetected => "Memory Leak Detected",
        }
    }
}
