use std::io::Write;
use std::time::Instant;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoggerError {
}

pub fn init() -> Result<(), LoggerError> {
    let start_time = Instant::now();
    let appname = std::env::args().next().unwrap_or_else(|| "app".into());
    env_logger::Builder::from_env("LOGLEVEL")
        .format(move |buf, record| {
            let time = Instant::now()
                .duration_since(start_time)
                .as_secs();
            writeln!(
                buf,
                "[{}] {}: {}: {}",
                time,
                appname,
                record.level(),
                record.args()
            )
        })
        .init();
    Ok(())
}
