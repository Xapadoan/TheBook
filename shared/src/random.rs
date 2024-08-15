pub trait Random {
    fn random() -> Self;
}

pub trait RandomDictionary<T> {
    fn random_item() -> T;
}
