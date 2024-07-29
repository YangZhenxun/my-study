extern crate libc;
use libc::c_void;
use std::{error::Error, fmt::Display};
use crate::cxx_std_need::bad_alloc_def_new;

use crate::{_zbar_error_string, _zbar_get_error_code};

pub trait Exception_who {
    fn new(obj: *const c_void, who: _Children) -> Exception;
}

pub trait Exception_default {
    fn new(obj: *const c_void) -> Exception;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum _Many{
    Exception(Exception),
    Bad_Alloc(crate::cxx_std_need::bad_alloc)
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum _Children {
    InternalE(InternalError),
    UE(UnsupportedError),
    InvaidE(InvaidError),
    SE(SystemError),
    LE(LockingError),
    BE(BusyError),
    DisplayE(XDisplayError),
    ProtoE(XProtoError),
    CE(ClosedError),
    FE(FormatError),
    End(nullerr)
}
impl Error for _Children {}
impl std::fmt::Display for _Children {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

/// base class for exceptions defined by this API.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Exception{
    _obj: *const c_void,
    who: _Children
}

impl Error for Exception {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if self.who != _Children::End(nullerr{}){
            None
        } else {
            Some(&self.who)
        }
    }
}

impl Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self._obj == std::ptr::null() {
            write!(f, "zbar library unspecified generic error")
        } else {
            unsafe{
                write!(f, "{:#?}", _zbar_error_string(self._obj, 0))
            }
        }
    }
}

impl Exception_who for Exception{
    fn new(obj: *const c_void, who:_Children) -> Exception {
        Exception{_obj: obj, who: who}
    }
}

impl Exception_default for Exception {
    fn new(obj: *const c_void) -> Exception {
        Exception{_obj: obj, who: _Children::End(nullerr{})}
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InternalError {
    obj: *const c_void
}
impl InternalError {
    pub fn new(obj: *const c_void) -> Exception {
        <Exception as Exception_who>::new(obj, _Children::InternalE(InternalError{obj}))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnsupportedError {
    obj: *const c_void
}
impl UnsupportedError {
    pub fn new(obj: *const c_void) -> Exception {
        <Exception as Exception_who>::new(obj, _Children::UE(UnsupportedError{obj}))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InvaidError {
    obj: *const c_void
}
impl InvaidError {
    pub fn new(obj: *const c_void) -> Exception {
        <Exception as Exception_who>::new(obj, _Children::InvaidE(InvaidError{obj}))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SystemError {
    obj: *const c_void
}
impl SystemError {
    pub fn new(obj: *const c_void) -> Exception {
        <Exception as Exception_who>::new(obj, _Children::SE(SystemError {obj}))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LockingError {
    obj: *const c_void
}
impl LockingError {
    pub fn new(obj: *const c_void) -> Exception {
        <Exception as Exception_who>::new(obj, _Children::LE(LockingError {obj}))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BusyError {
    obj: *const c_void
}
impl BusyError {
    pub fn new(obj: *const c_void) -> Exception {
        <Exception as Exception_who>::new(obj, _Children::BE(BusyError { obj }))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct XDisplayError {
    obj: *const c_void
}
impl XDisplayError {
    pub fn new(obj: *const c_void) -> Exception {
        <Exception as Exception_who>::new(obj, _Children::DisplayE(XDisplayError { obj }))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XProtoError {
    obj: *const c_void
}
impl XProtoError {
    pub fn new(obj: *const c_void) -> Exception {
        <Exception as Exception_who>::new(obj, _Children::ProtoE(XProtoError { obj }))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosedError {
    obj: *const c_void
}
impl ClosedError {
    pub fn new(obj: *const c_void) -> Exception {
        <Exception as Exception_who>::new(obj, _Children::CE(ClosedError { obj }))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct nullerr{}
impl Error for nullerr{}
impl Display for nullerr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error and no reason needed")
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FormatError {}
impl Error for FormatError{}
impl Display for FormatError{
    fn fmt(&self, f: &mut std::fmt::Formatter) ->std::fmt::Result {
        write!(f, "unsupported format")
    }
}

#[inline]
pub fn throw_exception(obj: *const c_void) -> Result<(), _Many>{
    unsafe {
        match _zbar_get_error_code(obj) {
            crate::zbar_error_e::ZBAR_ERR_NOMEM => Err(_Many::Bad_Alloc(crate::cxx_std_need::bad_alloc::new())),
            crate::zbar_error_e::ZBAR_ERR_INTERNAL => Err(_Many::Exception(InternalError::new(obj))),
            crate::zbar_error_e::ZBAR_ERR_UNSUPPORTED => Err(_Many::Exception(UnsupportedError::new(obj))),
            crate::zbar_error_e::ZBAR_ERR_INVALID => Err(_Many::Exception(InvaidError::new(obj))),
            crate::zbar_error_e::ZBAR_ERR_SYSTEM => Err(_Many::Exception(SystemError::new(obj))),
            crate::zbar_error_e::ZBAR_ERR_LOCKING => Err(_Many::Exception(LockingError::new(obj))),
            crate::zbar_error_e::ZBAR_ERR_BUSY => Err(_Many::Exception(BusyError::new(obj))),
            crate::zbar_error_e::ZBAR_ERR_XDISPLAY => Err(_Many::Exception(XDisplayError::new(obj))),
            crate::zbar_error_e::ZBAR_ERR_XPROTO => Err(_Many::Exception(XProtoError::new(obj))),
            crate::zbar_error_e::ZBAR_ERR_CLOSED => Err(_Many::Exception(ClosedError::new(obj))),
            _ => Err(_Many::Exception(<Exception as Exception_default>::new(obj))),
        }
    }
}
