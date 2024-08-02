use std::io::Write;
use std::time::Instant;
use thiserror::Error;
use std::path::Path;
use colored::Colorize;

#[derive(Error, Debug)]
pub enum LoggerError {
    #[error("Can't get full path of application")]
    FullPath,
    #[error("File name can't be obtained")]
    FileName,
    #[error("Name is not valid UTF-8")]
    NameStr,
}

pub fn init() -> Result<(), LoggerError> {
    let start_time = Instant::now();
    let fullpath = std::env::args()
        .next()
        .ok_or(LoggerError::FullPath)?;
    let appname = Path::new(&fullpath)
        .file_name()
        .ok_or(LoggerError::FileName)?
        .to_str()
        .ok_or(LoggerError::NameStr)?
        .to_owned()
        .cyan();

    env_logger::Builder::from_env("LOGLEVEL")
        .format(move |buf, record| {
            let time = Instant::now()
                .duration_since(start_time)
                .as_secs();
            let level = {
                let lvl_str = record.level().to_string();
                match record.level() {
                    log::Level::Error => lvl_str.red(),
                    log::Level::Warn => lvl_str.yellow(),
                    log::Level::Info => lvl_str.green(),
                    log::Level::Debug => lvl_str.blue(),
                    log::Level::Trace => lvl_str.purple(),
                }
            };

            writeln!(
                buf,
                "[{}] {}: {}: {}",
                time,
                appname,
                level,
                record.args()
            )
        })
        .init();

    Ok(())
}
