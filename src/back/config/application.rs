use config::{Config, ConfigError, File};
use flexi_logger::{AdaptiveFormat, Logger};

pub fn logger_initializer(level: String) -> bool {
    let logger = Logger::try_with_str(level)
        .unwrap()
        .adaptive_format_for_stderr(AdaptiveFormat::Detailed)
        .use_utc()
        .start();

    logger.is_err()
}

pub fn load_file_settings(path: &str) -> Result<Config, ConfigError> {
    Config::builder()
        .add_source(vec![File::with_name(path)])
        .build()
}
