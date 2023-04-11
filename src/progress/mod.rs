
// A Poor man's progress logger
// Invoking the constructor will print the start message (and set the total count)
// increment() will print a dot every 10 times it is called, and a message every 100 times
// done() will print the final message
pub mod progress {
    use std::io::{self, Write};

    pub struct ProgressLogger {
        count: u32,
        total: usize,
        message: String,
    }

    impl ProgressLogger {
        pub fn new(message: String, total: usize) -> ProgressLogger {
            println!("... Start {} ({})", message, total);
            ProgressLogger { count: 0, total, message }
        }

        pub fn increment(&mut self) {
            self.count += 1;

            if self.count % 100 == 0 {
                println!("... {} ({}/{})", self.message, self.count, self.total);
            } else if self.count % 10 == 0 {
                print!(".");
                // flush stdout so we can see progress
                io::stdout().flush().ok().expect("Could not flush stdout");
        }
        }
        pub fn done(&mut self) {
            println!("... Done {} ({})", self.message, self.count);
        }
    }
}
