/*------------------------------------------------------------------------
 *  Copyright 2008-2010 (c) Jeff Brown <spadix@users.sourceforge.net>
 *
 *  This file is part of the ZBar Bar Code Reader.
 *
 *  The ZBar Bar Code Reader is free software; you can redistribute it
 *  and/or modify it under the terms of the GNU Lesser Public License as
 *  published by the Free Software Foundation; either version 2.1 of
 *  the License, or (at your option) any later version.
 *
 *  The ZBar Bar Code Reader is distributed in the hope that it will be
 *  useful, but WITHOUT ANY WARRANTY; without even the implied warranty
 *  of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Lesser Public License for more details.
 *
 *  You should have received a copy of the GNU Lesser Public License
 *  along with the ZBar Bar Code Reader; if not, write to the Free
 *  Software Foundation, Inc., 51 Franklin St, Fifth Floor,
 *  Boston, MA  02110-1301  USA
 *
 *  http://sourceforge.net/projects/zbar
 *------------------------------------------------------------------------*/

use cxx_std_need::*;
use crate::*;
use zbar_symbol_type_e::*;
use zbar_config_e::*;

/* 
parse a configuration string of the form "[symbology.]config[=value]".
the config must match one of the recognized names.
the symbology, if present, must match one of the recognized names.
if symbology is unspecified, it will be set to 0.
if value is unspecified it will be set to 1.
returns 0 if the config is parsed successfully, 1 otherwise
@since 0.4
*/
#[doc = " parse a configuration string of the form \"[symbology.]config[=value]\".\n the config must match one of the recognized names.\n the symbology, if present, must match one of the recognized names.\n if symbology is unspecified, it will be set to 0.\n if value is unspecified it will be set to 1.\n returns 0 if the config is parsed successfully, 1 otherwise\n @since 0.4"]
pub fn zbar_parse_config(mut cfgstr: Option<&str>, 
    mut sym: &zbar_symbol_type_t,
    mut cfg: &zbar_config_t,
    mut val: i64) -> i64
{   
    let mut len = 0;
    let mut negate = 0;
    match cfgstr {
        Some(mut cfgstr) => {
            let mut cfgstr = CStrPtr::new(cfgstr.to_string(), 0);
            if let Some(dot) = strptrchr(cfgstr.clone(), '.'){
                len = (dot.clone() - cfgstr.clone().to_string()) as usize;
                if len == 0 || (len == 1 && strncmp(cfgstr.to_string().as_str(), "*", len) == 0){
                    sym = &ZBAR_NONE;
                } else if len < 2{
                    return 1;
                } else if strncmp(cfgstr.to_string().as_str(), "qrcode", len) == 0{
                    sym = &ZBAR_QRCODE;
                } else if strncmp(cfgstr.to_string().as_str(), "db", len) == 0 {
                    sym = &ZBAR_DATABAR;
                } else if len < 3 {
                    return 1;
                } else if strncmp(cfgstr.to_string().as_str(), "upca", len) == 0 {
                    sym = &ZBAR_UPCA;
                } else if strncmp(cfgstr.to_string().as_str(), "upce", len) == 0 {
                    sym = &ZBAR_UPCE;
                } else if strncmp(cfgstr.to_string().as_str(), "ean13", len) == 0 {
                    sym = &ZBAR_EAN13;
                } else if strncmp(cfgstr.to_string().as_str(), "ean8", len) == 0 {
                    sym = &ZBAR_EAN8;
                } else if strncmp(cfgstr.to_string().as_str(), "ean5", len) == 0 {
                    sym = &ZBAR_EAN5;
                } else if strncmp(cfgstr.to_string().as_str(), "ean2", len) == 0 {
                    sym = &ZBAR_EAN2;
                } else if strncmp(cfgstr.to_string().as_str(), "composite", len) == 0 {
                    sym = &ZBAR_COMPOSITE;
                } else if strncmp(cfgstr.to_string().as_str(), "i25", len) == 0 {
                    sym = &ZBAR_I25;
                } else if len < 4 {
                    return 1;
                } else if strncmp(cfgstr.to_string().as_str(), "scanner", len) == 0 {
                    sym = &ZBAR_PARTIAL; /* FIXME lame */
                } else if strncmp(cfgstr.to_string().as_str(), "isbn13", len) == 0 {
                    sym = &ZBAR_ISBN13;
                } else if strncmp(cfgstr.to_string().as_str(), "isbn10", len) == 0 {
                    sym = &ZBAR_ISBN10;
                } else if strncmp(cfgstr.to_string().as_str(), "db-exp", len) == 0 {
                    sym = &ZBAR_DATABAR_EXP;
                } else if strncmp(cfgstr.to_string().as_str(), "codabar", len) == 0 {
                    sym = &ZBAR_CODABAR;
                } else if len < 6 {
                    return 1;
                } else if strncmp(cfgstr.to_string().as_str(), "code93", len) == 0 {
                    sym = &ZBAR_CODE93;
                } else if strncmp(cfgstr.to_string().as_str(), "code39", len) == 0 {
                    sym = &ZBAR_CODE39;
                } else if strncmp(cfgstr.to_string().as_str(), "pdf417", len) == 0 {
                    sym = &ZBAR_PDF417;
                } else if len < 7 {
                    return 1;
                } else if strncmp(cfgstr.to_string().as_str(), "code128", len) == 0 {
                    sym = &ZBAR_CODE128;
                } else if strncmp(cfgstr.to_string().as_str(), "databar", len) == 0 {
                    sym = &ZBAR_DATABAR;
                } else if strncmp(cfgstr.to_string().as_str(), "databar-exp", len) == 0 {
                    sym = &ZBAR_DATABAR_EXP;
                } else {
                    return 1;
                }
                cfgstr = dot.clone() + 1;
            } else {
                sym = &zbar_symbol_type_e::ZBAR_NONE;
            }
            len = cfgstr.len();
            let eq = strptrchr(cfgstr.clone(), '=');
            match eq{
                Some(ref eq) => len = (eq.clone() - cfgstr.to_string()) as usize,
                None => val = 1 /* handle this here so we can override later */
            }
            negate = 0;
            if len > 3 && strncmp(cfgstr.to_string().as_str(), "no-", 3) == 0{
                negate = 1; 
                cfgstr += 3;
                len -= 3;
            }
            if len < 1{
                return 1;
            } else if strncmp(cfgstr.to_string().as_str(), "y-density", len) == 0{
                cfg = &ZBAR_CFG_Y_DENSITY;
            } else if strncmp(cfgstr.to_string().as_str(), "x-density", len) == 0{
                cfg = &ZBAR_CFG_X_DENSITY;
            } else if len < 2 {
                return 1;
            } else if strncmp(cfgstr.to_string().as_str(), "enable", len) == 0{
                cfg = &ZBAR_CFG_ENABLE;
            } else if len < 3{
                return 1;
            } else if strncmp(cfgstr.to_string().as_str(), "disable", len) == 0{
                cfg = &ZBAR_CFG_ENABLE;
                negate = !negate; /* no-disable ?!? */
            } else if strncmp(cfgstr.to_string().as_str(), "min-length", len) == 0{
                cfg = &ZBAR_CFG_MIN_LEN;
            } else if strncmp(cfgstr.to_string().as_str(), "max-length", len) == 0{
                cfg = &ZBAR_CFG_MAX_LEN;
            } else if strncmp(cfgstr.to_string().as_str(), "ascii", len) == 0{
                cfg = &ZBAR_CFG_ASCII;
            } else if strncmp(cfgstr.to_string().as_str(), "binary", len) == 0{
                cfg = &ZBAR_CFG_BINARY;
            } else if strncmp(cfgstr.to_string().as_str(), "add-check", len) == 0{
                cfg = &ZBAR_CFG_ADD_CHECK;
            } else if strncmp(cfgstr.to_string().as_str(), "emit-check", len) == 0{
                cfg = &ZBAR_CFG_EMIT_CHECK;
            } else if strncmp(cfgstr.to_string().as_str(), "uncertainty", len) == 0{
                cfg = &ZBAR_CFG_UNCERTAINTY;
            } else if strncmp(cfgstr.to_string().as_str(), "test-inverted", len) == 0{
                cfg = &ZBAR_CFG_TEST_INVERTED;
            } else if strncmp(cfgstr.to_string().as_str(), "position", len) == 0{
                cfg = &ZBAR_CFG_POSITION;
            } else {
                return 1;
            }
            match eq{
                Some(eq) => {
                    val = strtol((eq.clone() + 1).to_string().as_str(), None, 0) as i64;
                },
                None => ()
            }
            if negate == 1{
                val = !val
            }
            return 0;
        },
        None => return 1
    }
}