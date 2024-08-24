use std::fmt::Display;
use std::io;
use std::error::Error;
use std::num::ParseIntError;

use shared::name::Name;
use shared::warrior::Warrior;

pub fn select_warrior<'a>(warriors: &mut Vec<&'a Warrior>) -> Result<Option<&'a Warrior>, SelectWarriorError> {
    if warriors.len() < 1 {
        return Ok(None)
    }
    println!("Select a warrior:");
    let mut i = 0;
    while i < warriors.len() {
        println!("{}. {}", i + 1, warriors[i].name());
        i += 1;
    }
    println!("{}. Back", i + 1);
    let mut user_response = String::new();
    io::stdin().read_line(&mut user_response)?;
    let mut index: usize = user_response.trim().parse()?;
    index -= 1;
    if index > warriors.len() {
        return Ok(None)
    }
    let warrior = warriors.swap_remove(index);
    Ok(Some(warrior))
}

#[derive(Debug)]
pub struct SelectWarriorError {
    message: String,
}

impl SelectWarriorError {
    fn new(message: String) -> Self {
        Self { message: format!("Tournament Replay Build Error\n{message}") }
    }
}

impl Display for SelectWarriorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for SelectWarriorError {}

impl From<io::Error> for SelectWarriorError {
    fn from(value: io::Error) -> Self {
        Self::new(format!("io Error:\n{value}"))
    }
}

impl From<ParseIntError> for SelectWarriorError {
    fn from(value: ParseIntError) -> Self {
        Self::new(format!("Parse Int Error:\n{value}"))
    }
}
