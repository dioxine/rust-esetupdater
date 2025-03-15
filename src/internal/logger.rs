use log::{error,debug};
use syslog::{Error, Facility};

pub fn push_into_syslog(string: String) -> Result<(), Error> {
    syslog::init(
        Facility::LOG_USER,
        log::LevelFilter::Debug,
        Some("rust-esetupdater"),
    )?;
    debug!("this is a debug {}", "message");
    error!("this is an error!");
    error!("{string}");
    Ok(())
}
