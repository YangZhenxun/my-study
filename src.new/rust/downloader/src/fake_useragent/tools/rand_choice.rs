use rand::prelude::*;
#[inline]
pub fn random_choice<T>(list: Vec<T>) -> T
where T: std::any::Any+Clone,{
    let mut rng = thread_rng();
    list[rng.gen_range(0..list.len())].clone()
}