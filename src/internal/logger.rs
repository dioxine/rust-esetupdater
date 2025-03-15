use log::{LevelFilter, info};
use syslog::{BasicLogger, Facility, Formatter3164};

pub fn push_into_syslog(string: String) {
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "rust-esetupdater".into(),
        pid: 0,
    };

    let logger = match syslog::unix(formatter) {
        Err(e) => {
            println!("impossible to connect to syslog: {:?}", e);
            return;
        }
        Ok(logger) => logger,
    };

    let _result = log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
        .map(|()| log::set_max_level(LevelFilter::Info));

    info!("{}", string);
}
