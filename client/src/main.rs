use std::process;

use client::run;

pub fn main() {
    if let Err(e) = run() {
        eprintln!("Client error:\n{e}");
        process::exit(1);
    }
}
