use log::{LevelFilter, SetLoggerError};
use log4rs::{append::console::ConsoleAppender, config::{Appender, Root}, encode::pattern::PatternEncoder, Config, Handle};


pub fn configure_logs(min_level: LevelFilter) -> Result<Handle, SetLoggerError> {
    let console = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} [{l}] {m}{n}",
        )))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("console", Box::new(console)))
        .build(Root::builder().appender("console").build(min_level))
        .unwrap();

    log4rs::init_config(config)
}
