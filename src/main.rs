use std::process;

use the_book;

fn main() {
    if let Err(e) = the_book::run() {
        eprintln!("Application error:\n{e}");
        process::exit(1);
    }
}
