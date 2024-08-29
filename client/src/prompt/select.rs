use std::io;

use super::{prompt, PromptError};

pub fn select_with_keys<'a, T, F: Fn(&'a T) -> String>(
    message: &str,
    options: &[&'a T],
    display: F,
) -> Result<Option<&'a T>, PromptError> {
    if options.len() < 1 {
        return Ok(None)
    }
    println!("{message}");
    let mut i = 0;
    while i < options.len() {
        println!("{}. {}", i + 1, display(options[i]));
        i += 1;
    }
    println!("{}. Back", i + 1);
    let mut user_response = String::new();
    io::stdin().read_line(&mut user_response)?;
    let mut index: usize = user_response.trim().parse()?;
    index -= 1;
    if index >= options.len() {
        return Ok(None)
    }
    Ok(Some(options[index]))
}

pub fn select_with_arrows<'a, T, F: Fn(&'a T) -> String>(
    message: &str,
    options: &[&'a T],
    display: F,
) -> Result<Option<&'a T>, PromptError> {
    if options.len() < 1 {
        return Ok(None)
    }
    let mut i = 0;
    loop {
        println!("{message}");
        println!("{}", display(options[i]));
        let res = prompt("Previous (4) / Select(O) / Next(6)")?;
        if res == "O" {
            return Ok(Some(options[i]));
        } else if res == "4" {
            i = if i == 0 { options.len() - 1 } else { i - 1 };
        } else if res == "6" {
            i = if i == options.len() - 1 { 0 } else { i + 1 };
        }
    }
}