use std::io::Write;
use thiserror::Error;
use std::path::Path;
use colored::Colorize;
#[cfg(not(feature = "chrono"))] use std::time::Instant;
#[cfg(feature = "chrono")] use chrono::{Utc, Duration, format::{Item, StrftimeItems}};

#[derive(Error, Debug)]
pub enum LoggerError {
    #[error("Can't get full path of application")]
    FullPath,
    #[error("File name can't be obtained")]
    FileName,
    #[error("Name is not valid UTF-8")]
    NameStr,
    #[error("Time zone env variable not set")]
    TimeZoneEnv,
    #[error("Can't parse time zone variable")]
    TimeZoneParse,
}

#[cfg(feature = "chrono")]
lazy_static::lazy_static! {
    static ref FORMAT: Vec<Item<'static>> = StrftimeItems::new("%Y-%m-%d %H:%M:%S").parse().unwrap();
}

pub fn init() -> Result<(), LoggerError> {
    #[cfg(not(feature = "chrono"))]
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

    let offset: i64 = std::env::var("TZ").ok()
        .ok_or(LoggerError::TimeZoneEnv)?
        .parse().ok()
        .ok_or(LoggerError::TimeZoneParse)?;

    env_logger::Builder::from_env("LOGLEVEL")
        .format(move |buf, record| {
            #[cfg(not(feature = "chrono"))]
            let time = Instant::now()
                .duration_since(start_time)
                .as_secs();

            #[cfg(feature = "chrono")]
            let time = (Utc::now() + Duration::hours(offset))
                .format_with_items(FORMAT.iter());

            let level = match record.level() {
                log::Level::Error => "ERROR".red(),
                log::Level::Warn => "WARN".yellow(),
                log::Level::Info => "INFO".green(),
                log::Level::Debug => "DEBUG".bright_blue(),
                log::Level::Trace => "TRACE".purple(),
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
