use crate::*;
use zbar_symbol_type_e::*;
use zbar_config_e::*;

#[macro_export]
macro_rules! NUM_SYMS {() => {20}}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct point_s{
    x: i64,
    y: i64
}

pub type point_t = point_s;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct zbar_symbol_set_s{
    refcnt: refcnt_t,
    nsyms: i64,
    head: Option<Box<zbar_symbol_t>>,
    tail: Box<zbar_symbol_t>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct zbar_symbol_s {
    pub _type: zbar_symbol_type_t,
    pub configs: u64,
    pub modifiers: u64,
    pub data_alloc: Option<u64>,
    pub datalen: u64,
    pub data: Option<String>,

    pub pts_alloc: u64,
    pub npts: u64,
    pub pts: Option<point_t>,
    pub orient: zbar_orientation_t,

    pub refcnt: refcnt_t,
    pub next: Option<Box<zbar_symbol_t>>,
    pub syms: Option<zbar_symbol_set_t>,
    pub time: u64,
    pub cache_count: i64,
    pub quality: i64
}

pub fn zbar_get_symbol_name(sym: zbar_symbol_type_t) -> String
{
    match zbar_symbol_type_t::try_from(sym as i64 & ZBAR_SYMBOL as i64){
        Ok(ZBAR_EAN2) => return "EAN-2".to_string(),
        Ok(ZBAR_EAN5) => return "EAN-5".to_string(),
        Ok(ZBAR_EAN8) => return "EAN-8".to_string(),
        Ok(ZBAR_UPCE) => return "UPC-E".to_string(),
        Ok(ZBAR_ISBN10) => return "ISBN-10".to_string(),
        Ok(ZBAR_UPCA) => return "UPC-A".to_string(),
        Ok(ZBAR_EAN13) => return "EAN-13".to_string(),
        Ok(ZBAR_ISBN13) => return "ISBN-13".to_string(),
        Ok(ZBAR_COMPOSITE) => return "COMPOSITE".to_string(),
        Ok(ZBAR_I25) => return "I2/5".to_string(),
        Ok(ZBAR_DATABAR) => return "DataBar".to_string(),
        Ok(ZBAR_DATABAR_EXP) => return "DataBar-Exp".to_string(),
        Ok(ZBAR_CODABAR) => return "Codabar".to_string(),
        Ok(ZBAR_CODE39) => return "CODE-39".to_string(),
        Ok(ZBAR_CODE93) => return "CODE-93".to_string(),
        Ok(ZBAR_CODE128) => return "CODE-128".to_string(),
        Ok(ZBAR_PDF417) => return "PDF417".to_string(),
        Ok(ZBAR_QRCODE) => return "QR-Code".to_string(),
        Ok(ZBAR_SQCODE) =>return "SQ-Code".to_string(),
        _ => return "UNKNOWN".to_string()
    }
}

pub fn zbar_get_addon_name(_sym: zbar_symbol_type_t) -> String
{
    return "".to_string()
}

pub fn zbar_get_config_name(cfg: zbar_config_t) -> String
{
    match cfg{
        ZBAR_CFG_ENABLE => return "ENABLE".to_string(),
        ZBAR_CFG_ADD_CHECK => return "ADD_CHECK".to_string(),
        ZBAR_CFG_EMIT_CHECK => return "EMIT_CHECK".to_string(),
        ZBAR_CFG_ASCII => return "ASCII".to_string(),
        ZBAR_CFG_BINARY => return "BINARY".to_string(),
        ZBAR_CFG_MIN_LEN => return "MIN_LEN".to_string(),
        ZBAR_CFG_MAX_LEN => return "MAX_LEN".to_string(),
        ZBAR_CFG_UNCERTAINTY => return "UNCERTAINTY".to_string(),
        ZBAR_CFG_POSITION => return "POSITION".to_string(),
        ZBAR_CFG_X_DENSITY => return "X_DENSITY".to_string(),
        ZBAR_CFG_Y_DENSITY => return "Y_DENSITY".to_string(),
        _ => return "".to_string()
    }
}

pub fn zbar_get_modifer_name(moder: zbar_modifier_t) -> String 
{
    match moder{
        zbar_modifier_e::ZBAR_MOD_GS1 => return "GS1".to_string(),
        zbar_modifier_e::ZBAR_MOD_AIM => return "AIM".to_string(),
        _ => return "".to_string()
    }
}

pub fn zbar_get_orientation_name(orient: zbar_orientation_t) -> String
{
    match orient {
        zbar_orientation_e::ZBAR_ORIENT_UP => return "UP".to_string(),
        zbar_orientation_e::ZBAR_ORIENT_RIGHT => return "RIGHT".to_string(),
        zbar_orientation_e::ZBAR_ORIENT_DOWN => return "DOWN".to_string(),
        zbar_orientation_e::ZBAR_ORIENT_LEFT => return "LEFT".to_string(),
        _ => return "UNKNOWN".to_string()
    }
}

pub fn _init_hash() -> Vec<i32>{
    let mut hash = vec![-1];
    let mut was_initialized = false;
    if was_initialized {
        return hash
    }
    /* Keep in sync with the C99 implementation */
    hash[ZBAR_SQCODE as usize] = 1; hash[ZBAR_CODE128 as usize] = 2; hash[ZBAR_EAN13 as usize] = 3;
    hash[ZBAR_UPCA as usize] = 4; hash[ZBAR_EAN8 as usize] = 5; hash[ZBAR_UPCE as usize] = 6;
    hash[ZBAR_ISBN13 as usize] = 7; hash[ZBAR_ISBN10 as usize] = 8; hash[ZBAR_CODE39 as usize] = 9;
    hash[ZBAR_I25 as usize] = 10; hash[ZBAR_PDF417 as usize] = 11; hash[ZBAR_QRCODE as usize] = 12;
    hash[ZBAR_DATABAR as usize] = 13; hash[ZBAR_DATABAR_EXP as usize] = 14;
    hash[ZBAR_CODE93 as usize] = 15; hash[ZBAR_EAN2 as usize] = 16; hash[ZBAR_EAN5 as usize] = 17;
    hash[ZBAR_COMPOSITE as usize] = 18; hash[ZBAR_CODABAR as usize] = 19;

    was_initialized = true;
    return hash;
}

pub fn _zbar_get_symbol_hash(sym: zbar_symbol_type_t) -> i32
{
    let hash = _init_hash();
    assert!((sym as i64) >= (ZBAR_PARTIAL as i64) && (sym as i64) <= (ZBAR_CODE128 as i64));
    let h = hash[sym as usize];
    assert!(h >= 0 && h < NUM_SYMS!());
    return h;
}

pub fn _zbar_symbol_free(mut sym: zbar_symbol_t)
{
    if let Some(syms) = sym.syms {
        zbar_symbol_set_ref(syms, -1);
        sym.syms = None
    }
    if let Some(pts) = sym.pts {
        drop(pts);
    }
    if let Some(ref data) = sym.data {
        if sym.data_alloc != None{
            drop(data)
        }
    }
    drop(sym);
}

#[inline]
pub fn _zbar_symbol_refcnt(sym: zbar_symbol_t, delta: i32){
    if _zbar_refcnt(sym.refcnt, delta) == 0 && delta <= 0{
        _zbar_symbol_free(sym);
    }
}

#[inline]
pub fn _zbar_symbol_set_free(mut syms: zbar_symbol_set_t)
{
    let mut sym = syms.head;
    let mut next = None;
    loop{
        match sym{
            Some(ref mut sym) => {
                next = sym.next.clone();
                sym.next = None;
                let mut csym = sym.clone();
                _zbar_symbol_refcnt(*csym, -1);
                match next{
                    Some(next) => csym = next,
                    None => break
                }
            },
            None => break
        }
    }
    syms.head = None;
    drop(syms);
}

pub fn zbar_symbol_set_ref(syms: zbar_symbol_set_t, delta: i32)
{
    if _zbar_refcnt(syms.refcnt, delta) == 0 && delta <= 0{
        _zbar_symbol_set_free(syms);
    }
}