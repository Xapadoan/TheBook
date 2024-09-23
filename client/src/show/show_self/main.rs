pub trait ShowSelf {
    fn show_self(&self) -> String;
}

impl<T: ShowSelf> ShowSelf for Option<T> {
    fn show_self(&self) -> String {
        match self {
            Some(item) => item.show_self(),
            None => "None".to_string(),
        }
    }
}
