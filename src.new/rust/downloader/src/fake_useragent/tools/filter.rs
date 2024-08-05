use std::future::Future;

pub fn filter<T, E>(filter_fn: T, to_filter_list: Vec<E>) -> Vec<E>
    where T: FnOnce(&E) -> bool+Copy,
    E:std::any::Any+Clone,{
    let mut list: Vec<E> = vec![];
    for unused in to_filter_list.iter() {
        if filter_fn(&unused.clone()) == true {
            list.push(unused.clone())
        }
    }
    return list;
}

pub fn asyfilter<T, E>(filter_fn: T, to_filter_list: Vec<E>) -> Vec<E>
    where 
    T: Future<Output = bool>+Copy,
    E: std::any::Any+Clone,
    {
    
}
