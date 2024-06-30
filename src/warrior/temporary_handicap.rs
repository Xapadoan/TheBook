pub mod parries_miss;
pub mod assaults_miss;

pub trait TemporaryHandicap {
    fn decrement_turns_count(&mut self);
    fn turns_left(&self) -> u8;
    fn reason(&self) -> &String;
}