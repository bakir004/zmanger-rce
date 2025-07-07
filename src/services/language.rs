pub struct LanguageConfig {
    pub extension: &'static str,
    pub compiler: &'static str,
    pub flags: &'static [&'static str],
}

pub fn get_language_config(language_id: u8) -> Result<LanguageConfig, std::io::Error> {
    match language_id {
        1 => Ok(LanguageConfig {
            extension: "cpp",
            compiler: "g++",
            flags: &["-fsanitize=address"],
        }),
        2 => Ok(LanguageConfig {
            extension: "c",
            compiler: "gcc",
            flags: &["-fsanitize=address"],
        }),
        _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Unsupported language ID")),
    }
}
