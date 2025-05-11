use anyhow::Context;
use log::LevelFilter;
use log4rs::{
    Config, Handle,
    append::console::ConsoleAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
};

pub fn configure_logs(min_level: LevelFilter) -> Result<Handle, anyhow::Error> {
    let console = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} [{l}] {m}{n}",
        )))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("console", Box::new(console)))
        .build(Root::builder().appender("console").build(min_level))
        .unwrap();

    log4rs::init_config(config).with_context(|| "failed to init logger")
}
