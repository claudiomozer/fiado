use log::{Record, Level, Metadata, LevelFilter};


struct SimpleLogger;
static LOGGER: SimpleLogger = SimpleLogger;


impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub fn init() {
    let result = log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info));

    if let Err(e) = result {
        panic!("{}", e);
    }
}