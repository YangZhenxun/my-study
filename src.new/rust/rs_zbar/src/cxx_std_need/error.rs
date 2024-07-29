extern crate libc;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct bad_alloc{
    _Message: *const libc::c_char
}
impl std::error::Error for bad_alloc{}
impl std::fmt::Display for bad_alloc{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        if self._Message != std::ptr::null(){
            write!(f, "{:?}", self._Message)
        } else {
            write!(f, "bad allocation")
        }
    }
}
pub trait bad_alloc_mes_new {
    fn new(message: *const libc::c_char) -> bad_alloc;
}
impl bad_alloc_mes_new for bad_alloc{
    fn new(message: *const libc::c_char) -> bad_alloc {
        bad_alloc { _Message: message }
    }
}

pub trait bad_alloc_def_new {
    fn new() -> bad_alloc;
}
impl bad_alloc_def_new for bad_alloc{
    fn new() -> bad_alloc {
        bad_alloc{ _Message: std::ptr::null()}
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Null{}
