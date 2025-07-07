pub const DELIMITER: &str = "===DELIMITER===";
pub const TIMEOUT_IN_SECONDS: u8 = 5;
pub const PODMAN_ARGUMENTS: [&str; 10] = [
    "run", 
    "--rm", 
    "-i", 
    "--cpus=1",
    "--security-opt", 
    "label=disable", 
    "--cap-add=SYS_PTRACE", 
    "--memory=512m", 
    "docker.io/library/gcc:latest", 
    "/bin/bash"
];

