use std::{env, process};

use server::{Config, run};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    if let Err(e) = run(&config).await {
        eprintln!("Application error:\n{e}");
        process::exit(1);
    }
}
