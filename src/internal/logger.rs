use simplelog::*;
use std::fs::File;
use time::{UtcDateTime, UtcOffset, macros::format_description};

pub fn get_time_now() -> String {
    let now = UtcDateTime::now();
    let local_utc_offset = UtcOffset::current_local_offset().unwrap();
    let format = format_description!("[day]-[month]-[year]-[hour]:[minute]:[second]_");
    let result_now = now.checked_to_offset(local_utc_offset).unwrap();
    result_now.format(&format).unwrap()
}

pub fn init() {
    // Logger settings
    // overriding default config to use local timezone offset
    let log_config = ConfigBuilder::new()
        .set_time_offset_to_local()
        .unwrap()
        .build();

    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        log_config,
        File::create(get_time_now() + "result.log").unwrap(),
    )])
    .unwrap();
}
