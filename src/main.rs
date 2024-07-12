use std::{env, process};

use the_book::{self, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    if let Err(e) = the_book::run(&config) {
        eprintln!("Application error:\n{e}");
        process::exit(1);
    }
}
