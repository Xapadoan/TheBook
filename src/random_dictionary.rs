pub trait RandomDictionary<T> {
    fn get_random_item(&self) -> T;
}
