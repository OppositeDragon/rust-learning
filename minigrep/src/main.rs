use std::process;

use minigrep::run;

fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
