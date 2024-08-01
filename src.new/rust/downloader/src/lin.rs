pub fn lin<T>(i: T, list: Vec<T>) -> Option<T> where T: std::any::Any+PartialEq{
    for l in list.iter() {
        if &i == l {
            return Option::Some(&i);
        }
    }
    return None;
}