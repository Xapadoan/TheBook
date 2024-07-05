pub type Name = String;

pub trait HasName {
    fn name(&self) -> &Name;
}