pub fn lin<T>(i: T, list: Vec<T>) -> Option<T> where 
    T: std::any::Any+PartialEq
    {
    for l in list.iter() {
        if i == *l {
            return Option::Some(i);
        }
        }
    None
}

use tokio_stream::{StreamExt, Iter};
pub async fn asylin<T, E>(i: T, mut list: Iter<E::IntoIter>) -> Option<T> 
    where 
    T: std::any::Any+PartialEq+PartialEq<<E as IntoIterator>::Item>,
    E:IntoIterator+PartialEq<<E as IntoIterator>::Item>{
    while let Some(l) = list.next().await{
        if &i == &l {
            return Option::Some(i);
        }
    }
    None
}