use std::sync::Mutex;

use indicatif::MultiProgress;
use lazy_static::lazy_static;

lazy_static! {
    static ref MULTIPROGRESS: Mutex<MultiProgress> = {
        let multiprogress = MultiProgress::new();
        Mutex::new(multiprogress)
    };
}

pub fn multiprogress() -> &'static Mutex<MultiProgress> {
    &MULTIPROGRESS
}

#[macro_export]
macro_rules! progress_println {

    ($($arg:tt)*) => {
        {
            let multiprogress = crate::progress::multiprogress::multiprogress()
                .lock();

            if let Ok(multiprogress) = multiprogress {
                if let Err(_) = multiprogress.println(format!($($arg)*)) {
                    eprintln!("Failed to log test progress.");
                }
            } else {
                eprintln!("Failed to log test progress.");
            }
        }
    }
}
