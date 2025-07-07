use crate::{globals::DELIMITER, services::language::LanguageConfig};

pub fn prepare_bash_script(
    code: &str,
    stdin: &str,
    language_config: &LanguageConfig,
    timeout: u8,
) -> String {
    format!(
        r#"echo '{code}' > code.{extension}
        echo '{stdin}' > stdin.txt
        {compiler} {flags} code.{extension} -o program 2> compile_errors.txt
        compile_status=$?
        if [ $compile_status -eq 0 ]; then
          start_time=$(date +%s%N)
          timeout {timeout}s ./program < stdin.txt > output.txt 2> runtime_errors.txt
          runtime_status=$?
          end_time=$(date +%s%N)
          elapsed_ns=$((end_time - start_time))
          elapsed_us=$((elapsed_ns / 1000))
        else
          runtime_status=127
          touch output.txt
          touch runtime_errors.txt
          elapsed_ns=0
        fi
        cat compile_errors.txt
        echo "{delimiter}"
        cat output.txt
        echo "{delimiter}"
        cat runtime_errors.txt
        echo "{delimiter}"
        echo $runtime_status
        echo "{delimiter}"
        echo $elapsed_us
        "#,
        extension = language_config.extension,
        compiler = language_config.compiler,
        flags = language_config.flags.join(" "),
        timeout = timeout,
        code = code,
        stdin = stdin,
        delimiter = DELIMITER 
    )
}
