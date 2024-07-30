use libc::c_uint;
use std::{ops, ptr};
use crate::*;
use std::ffi::CStr;

#[derive(Debug, Clone, Eq, PartialEq)]
enum _All{
    Handler(*mut Handler),
    Non(crate::cxx_std_need::Null)
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum _Decoder{
    Decoder(Decoder),
    Non(crate::cxx_std_need::Null)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Decoder{
    _decoder: *mut zbar_decoder_t,
    _handler: _All
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Handler {}
impl Handler {
    pub fn decode_callback(&self, decoder: &Decoder){}
    pub fn new()  -> Handler {
        Handler{}
    }
}

impl Drop for Decoder {
    fn drop(&mut self) {
        unsafe {
            crate::zbar_decoder_destroy(self._decoder)
        }
    }
}

pub trait config_all {
    fn set_config  (&self,
                        symbology: zbar_symbol_type_t,
                        config: zbar_config_t,
                        value: c_int) -> c_int;
}

impl Decoder {
    pub fn new() -> Decoder {
        unsafe{
            let handler = crate::cxx_std_need::Null{};
            let decoder = zbar_decoder_create();
            Decoder { _decoder: decoder, _handler: _All::Non(handler)}
        }
    }
    pub fn reset(&self){
        unsafe {
            zbar_decoder_reset(self._decoder)
        }
    }
    pub fn new_scan(&self){
        unsafe {
            zbar_decoder_new_scan(self._decoder)
        }
    }
    pub fn decode_width(&self, width: c_uint) -> zbar_symbol_type_t{
        unsafe{
            return zbar_decode_width(self._decoder, width)
        }
    }
    pub fn get_color(&self) -> zbar_color_t{
        unsafe {
            return zbar_decoder_get_color(self._decoder)
        }
    }
    pub fn get_type(&self) -> zbar_symbol_type_t{
        unsafe {
            return zbar_decoder_get_type(self._decoder)
        }
    }
    pub fn get_symbol_name(&self) -> *const c_char {
        unsafe{
            return zbar_get_addon_name(zbar_decoder_get_type(self._decoder));
        }
    }
    pub fn get_addon_name(&self) -> *const c_char {
        unsafe {
            return zbar_get_addon_name(zbar_decoder_get_type(self._decoder));
        }
    }
    pub fn get_data_chars(&self) -> *const c_char {
        unsafe {
            return zbar_decoder_get_data(self._decoder)
        }
    }
    pub fn get_data_string(&self)  -> Result<String, std::str::Utf8Error>{
        unsafe {
            let _return = CStr::from_ptr(zbar_decoder_get_data(self._decoder)).to_str()?.to_string() +
                CStr::from_ptr(ptr::from_ref(&zbar_decoder_get_data_length(self._decoder)) as *const i8).to_str()?;
            Ok(_return)
        }
    }
    pub fn get_data(&self) -> Result<String, std::str::Utf8Error>{
        let _return = self.get_data_string()?;
        Ok(_return)
    }
    pub fn get_data_length(&self) -> c_int{
        unsafe{
            return zbar_decoder_get_direction(self._decoder);
        }
    }
    pub fn get_direction(&self) -> c_int {
        unsafe{
            return zbar_decoder_get_direction(self._decoder);
        }
    }
    pub fn set_handler(&self, handler: &Handler){
        unsafe{
            let _handler = &handler;
            zbar_decoder_set_handler(self._decoder, Some(Decoder::_cb));
            zbar_decoder_set_userdata(self._decoder, ptr::from_ref(self) as *mut c_void);
        }
    }
    extern fn _cb(cdcode: *mut zbar_decoder_t){
        unsafe{
            let mut usage = Decoder::new();
            let dcode = zbar_decoder_get_userdata(cdcode);
            let _from = ptr::from_mut(&mut usage);
            match dcode as *mut Decoder {
                _from => {
                    let dcode = (*(dcode as *mut Decoder)).clone();
                    match dcode._handler {
                        _All::Handler(hand) => {
                            let hand = (*hand).clone();
                            hand.decode_callback(&dcode);
                        },
                        _All::Non(crate::cxx_std_need::Null{}) => {}
                    }
                }
            }
        }
    }
}

impl ops::Shl<c_uint> for Decoder {
    type Output = Decoder;

    fn shl(self, rhs: c_uint) -> Decoder{
        unsafe{
            zbar_decode_width(self._decoder, rhs);
            return self;
        }
    }
}

impl config_all for Decoder{
    fn set_config  (&self,
        symbology: zbar_symbol_type_t,
        config: zbar_config_t,
        value: c_int) -> c_int
    {
        unsafe{
            return zbar_decoder_set_config(self._decoder, symbology, config, value)
        }
    }
}
