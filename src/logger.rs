use log::{Log, LogRecord, LogLevel, LogMetadata};
use std::io::{Write, stderr};
use std::fs::OpenOptions;

pub struct SimpleLogger{
    pub logfile: String
}


impl Log for SimpleLogger {

    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Debug
    }

    fn log(&self, record: &LogRecord) {
        if &(*self.logfile)[..] == ""{
        if self.enabled(record.metadata()) {
            let _ = writeln!(
                &mut stderr(),
                "{} - {}",
                record.level(),
                record.args()
            );
        }
        }
        else{
        if let Ok(mut f) = OpenOptions::new().append(true).open(&(*self.logfile)[..]) {
            f.write_all(format!{"{} = {}\n", record.level(), record.args()}.as_bytes()).unwrap();
        }
    }
    }
}
