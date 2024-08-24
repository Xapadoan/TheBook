use std::{env, process};

use server::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    if let Err(e) = run(&config) {
        eprintln!("Application error:\n{e}");
        process::exit(1);
    }
}
