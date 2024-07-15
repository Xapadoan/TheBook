use std::io;
use std::error::Error;

use crate::{name::HasName, warrior::Warrior};

pub fn select_warrior<'a>(warriors: &mut Vec<&'a mut Warrior>) -> Result<Option<&'a mut Warrior>, Box<dyn Error>> {
    if warriors.len() < 1 {
        return Ok(None)
    }
    println!("Select a warrior:");
    let mut i = 0;
    let mut user_response = String::new();
    while i < warriors.len() {
        println!("{}. {}", i + 1, warriors[i].name());
        i += 1;
    }
    println!("{}. Back", i + 1);
    user_response.clear();
    io::stdin().read_line(&mut user_response)?;
    let mut index: usize = user_response.trim().parse()?;
    index -= 1;
    if index > warriors.len() {
        return Ok(None)
    }
    let warrior = warriors.swap_remove(index);
    Ok(Some(warrior))
}