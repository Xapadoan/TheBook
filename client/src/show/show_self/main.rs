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

pub trait ShowSelfExtended {
    fn show_self_extended(&self) -> String;
}

impl<T: ShowSelfExtended> ShowSelfExtended for Option<T> {
    fn show_self_extended(&self) -> String {
        match self {
            Some(item) => item.show_self_extended(),
            None => "None".to_string(),
        }
    }
}
