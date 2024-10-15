use std::process;

use client::run;
use dotenv;

pub fn main() {
    dotenv::from_filename(".env.client").ok();
    if let Err(e) = run() {
        eprintln!("Client error:\n{e}");
        process::exit(1);
    }
}
