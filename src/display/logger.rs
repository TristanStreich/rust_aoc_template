use colored::Colorize;
use log::Log;

use std::sync::atomic::{
    AtomicUsize,
    Ordering::Relaxed,
};

const EMOJI: [&str; 10] = ["🎄", "🎁", "⭐️", "🦌", "⛄️", "🎄", "🎁", "⭐️", "🍪", "🥛"];

static EMOJI_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct YuleLogger;

impl YuleLogger {
    pub fn init() {
        log::set_logger(&Self).unwrap();
        log::set_max_level(log::LevelFilter::Trace);
    }
}

impl Log for YuleLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let filename = record.file().unwrap();
        let line = record.line().unwrap();
        let time_stamp = time_stamp().bright_red();
        let args = record.args();

        let location = format!("[{filename}:{line}]").green();

        let emoji = {
            let emoji_index = EMOJI_COUNTER.fetch_add(1, Relaxed) % EMOJI.len();
            EMOJI[emoji_index]
        };

        println!("{emoji} {time_stamp} {location} {args}")
    }
    fn flush(&self) {}
}

macro_rules! log {
    ($($tt:tt)*) => {
        ::log::info!($($tt)*)
    };
}
pub(crate) use log;

#[allow(unused)]
macro_rules! debug {
    ($e:expr) => {{
        crate::display::logger::log!("{} = {:#?}", std::stringify!($e), $e);
        $e
    }};
}
#[allow(unused)]
pub(crate) use debug;

// returns current time as ISO string
fn time_stamp() -> String {
    let now = std::time::SystemTime::now();
    let dt: chrono::DateTime<chrono::Utc> = now.into();
    format!("{}", dt.format("%H:%M:%S%.3f"))
}
