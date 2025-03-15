use log::error;
use syslog::{Error, Facility};

pub fn push_into_syslog(string: String) -> Result<(), Error> {
    syslog::init(
        Facility::LOG_USER,
        log::LevelFilter::Debug,
        Some("My app name"),
    )?;
    error!("{string}");
    Ok(())
}
