pub type Name = String;

pub trait HasName {
    fn name<'a>(&'a self) -> &'a Name;
}