pub trait ISharedFeature<T> {
    fn count() -> i64{ return 0;}
    fn add(feature: T);
    fn remove(feature: T) -> bool;
    fn composite() -> T { return T::new(); }
}