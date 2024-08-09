/*------------------------------------------------------------------------
 *  Copyright 2007-2010 (c) Jeff Brown <spadix@users.sourceforge.net>
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

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]

/** @file
 * ZBar Barcode Reader C API definition
 */

/** @mainpage
 *
 * interface to the barcode reader is available at several levels.
 * most applications will want to use the high-level interfaces:
 *
 * @section high-level High-Level Interfaces
 *
 * these interfaces wrap all library functionality into an easy-to-use
 * package for a specific toolkit:
 * - the "GTK+ 2.x widget" may be used with GTK GUI applications.
 * - the @ref zbar::QZBar "Qt4 widget" may be used with Qt GUI
 *   applications
 * - the Processor interface (in @ref c-processor "C" or @ref
 *   zbar::Processor "Rust") adds a scanning window to an application
 *   with no GUI.
 *
 * @section mid-level Intermediate Interfaces
 *
 * building blocks used to construct high-level interfaces:
 * - the ImageScanner (in @ref c-imagescanner "C" or @ref
 *   zbar::ImageScanner "Rust") looks for barcodes in a library defined
 *   image object
 * - the Window abstraction (in @ref c-window "C" or @ref
 *   zbar::Window "Rust") sinks library images, displaying them on the
 *   platform display
 * - the Video abstraction (in @ref c-video "C" or @ref zbar::Video
 *   "Rust") sources library images from a video device
 *
 * @section low-level Low-Level Interfaces
 *
 * direct interaction with barcode scanning and decoding:
 * - the Scanner (in @ref c-scanner "C" or @ref zbar::Scanner "Rust")
 *   looks for barcodes in a linear intensity sample stream
 * - the Decoder (in @ref c-decoder "C" or @ref zbar::Decoder "Rust")
 *   extracts barcodes from a stream of bar and space widths
 */

/** @name Global library interfaces */
/*@{*/

extern crate libc;
use std::ops::BitAnd;

use libc::{c_char, c_float, c_int, c_uint, c_ulong, c_ulonglong, c_void};

/** "color" of element: bar or space. */
pub enum zbar_color_e{
    /**< light area or space between bars */    ZBAR_SPACE = 0,
    /**< dark area or colored bar segment */    ZBAR_BAR = 1,
}
#[doc = " \"color\" of element: bar or space. "]
pub type zbar_color_t = zbar_color_e;


#[derive(IntoPrimitive, TryFromPrimitive, Debug, Eq, PartialEq, Clone, Copy)]
#[repr(i64)]
/** decoded symbol type. */
pub enum zbar_symbol_type_e {
    /**< no symbol decoded */                   ZBAR_NONE        =      0,
    /**< intermediate status */                 ZBAR_PARTIAL     =      1,
    /**< GS1 2-digit add-on */                  ZBAR_EAN2        =      2,
    /**< GS1 5-digit add-on */                  ZBAR_EAN5        =      5,
    /**< EAN-8 */                               ZBAR_EAN8        =      8,
    /**< UPC-E */                               ZBAR_UPCE        =      9,
    /**< ISBN-10 (from EAN-13). @since 0.4 */   ZBAR_ISBN10      =     10,
    /**< UPC-A */                               ZBAR_UPCA        =     12,
    /**< EAN-13 */                              ZBAR_EAN13       =     13,
    /**< ISBN-13 (from EAN-13). @since 0.4 */   ZBAR_ISBN13      =     14,
    /**< EAN/UPC composite */                   ZBAR_COMPOSITE   =     15,
    /**< Interleaved 2 of 5. @since 0.4 */      ZBAR_I25         =     25,
    /**< GS1 DataBar (RSS). @since 0.11 */      ZBAR_DATABAR     =     34,
    /**< GS1 DataBar Expanded. @since 0.11 */   ZBAR_DATABAR_EXP =     35,
    /**< Codabar. @since 0.11 */                ZBAR_CODABAR     =     38,
    /**< Code 39. @since 0.4 */                 ZBAR_CODE39      =     39,
    /**< PDF417. @since 0.6 */                  ZBAR_PDF417      =     57,
    /**< QR Code. @since 0.10 */                ZBAR_QRCODE      =     64,
    /**< SQ Code. @since 0.20.1 */              ZBAR_SQCODE      =     80,
    /**< Code 93. @since 0.11 */                ZBAR_CODE93      =     93,
    /**< Code 128 */                            ZBAR_CODE128     =    128,

    /*
     * Please see _zbar_get_symbol_hash() if adding
     * anything after 128
     */

    /** mask for base symbol type.
     * @deprecated in 0.11, remove this from existing code
     */
    ZBAR_SYMBOL      = 0x00ff,
    /** 2-digit add-on flag.
     * @deprecated in 0.11, a ::ZBAR_EAN2 component is used for
     * 2-digit GS1 add-ons
     */
    ZBAR_ADDON2      = 0x0200,
    /** 5-digit add-on flag.
     * @deprecated in 0.11, a ::ZBAR_EAN5 component is used for
     * 5-digit GS1 add-ons
     */
    ZBAR_ADDON5      = 0x0500,
    /** add-on flag mask.
     * @deprecated in 0.11, GS1 add-ons are represented using composite
     * symbols of type ::ZBAR_COMPOSITE; add-on components use ::ZBAR_EAN2
     * or ::ZBAR_EAN5
     */
    ZBAR_ADDON       = 0x0700,
}
#[doc = " decoded symbol type. "]
pub type zbar_symbol_type_t = zbar_symbol_type_e;

/** decoded symbol coarse orientation.
 * @since 0.11
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum zbar_orientation_e {
    /**< unable to determine orientation */    ZBAR_ORIENT_UNKNOWN = -1,
    /**< upright, read left to right */        ZBAR_ORIENT_UP,
    /**< sideways, read top to bottom */       ZBAR_ORIENT_RIGHT,
    /**< upside-down, read right to left */    ZBAR_ORIENT_DOWN,
    /**< sideways, read bottom to top */       ZBAR_ORIENT_LEFT,
}
#[doc = "decoded symbol coarse orientation.\n* @since 0.11"]
pub type zbar_orientation_t = zbar_orientation_e;

/** error codes. */
pub enum zbar_error_e {
    /**< no error */                ZBAR_OK = 0,
    /**< out of memory */           ZBAR_ERR_NOMEM,
    /**< internal library error */  ZBAR_ERR_INTERNAL,
    /**< unsupported request */     ZBAR_ERR_UNSUPPORTED,
    /**< invalid request */         ZBAR_ERR_INVALID,
    /**< system error */            ZBAR_ERR_SYSTEM,
    /**< locking error */           ZBAR_ERR_LOCKING,
    /**< all resources busy */      ZBAR_ERR_BUSY,
    /**< X11 display error */       ZBAR_ERR_XDISPLAY,
    /**< X11 protocol error */      ZBAR_ERR_XPROTO,
    /**< output window is closed */ ZBAR_ERR_CLOSED,
    /**< windows system error */    ZBAR_ERR_WINAPI,
    /**< number of error codes */   ZBAR_ERR_NUM
}
#[doc = "error codes."]
pub type zbar_error_t = zbar_error_e;

/** decoder configuration options.
 * @since 0.4
 */
pub enum zbar_config_e {
    /**< enable symbology/feature */                ZBAR_CFG_ENABLE = 0,
    /**< enable check digit when optional */        ZBAR_CFG_ADD_CHECK,
    /**< return check digit when present */         ZBAR_CFG_EMIT_CHECK,
    /**< enable full ASCII character set */         ZBAR_CFG_ASCII,
    /**< don't convert binary data to text */       ZBAR_CFG_BINARY,
    /**< number of boolean decoder configs */       ZBAR_CFG_NUM,

    /**< minimum data length for valid decode */    ZBAR_CFG_MIN_LEN = 0x20,
    /**< maximum data length for valid decode */    ZBAR_CFG_MAX_LEN,

    /**< required video consistency frames */       ZBAR_CFG_UNCERTAINTY = 0x40,

    /**< enable scanner to collect position data */ ZBAR_CFG_POSITION = 0x80,
    /**< if fails to decode, test inverted */       ZBAR_CFG_TEST_INVERTED,

    /**< image scanner vertical scan density */     ZBAR_CFG_X_DENSITY = 0x100,
    /**< image scanner horizontal scan density */   ZBAR_CFG_Y_DENSITY,
}
#[doc = "decoder configuration options.\n * @since 0.4"]
pub type zbar_config_t = zbar_config_e;

/** decoder symbology modifier flags.
 * @since 0.11
 */
pub enum zbar_modifier_e {
    /** barcode tagged as GS1 (EAN.UCC) reserved
     * (eg, FNC1 before first data character).
     * data may be parsed as a sequence of GS1 AIs
     */
    ZBAR_MOD_GS1 = 0,

    /** barcode tagged as AIM reserved
     * (eg, FNC1 after first character or digit pair)
     */
    ZBAR_MOD_AIM,

    /** number of modifiers */
    ZBAR_MOD_NUM,
}
#[doc = "decoder symbology modifier flags.\n* @since 0.11"]
pub type zbar_modifier_t = zbar_modifier_e;

pub enum video_control_type_e {
    VIDEO_CNTL_INTEGER = 1,
    VIDEO_CNTL_MENU,
    VIDEO_CNTL_BUTTON,
    VIDEO_CNTL_INTEGER64,
    VIDEO_CNTL_STRING,
    VIDEO_CNTL_BOOLEAN,
}
pub type video_control_type_t = video_control_type_e;

/** store video control menu
 * `name` name of the menu item
 * `val` integer value associated with the item
 */
pub struct video_control_menu_e {
    name: String,
    value: i64
}
#[doc = "store video control menu\n* `name` name of the menu item\n* `val` integer value associated with the item"]
pub type video_control_menu_t = video_control_menu_e;

/** store video controls
 * `name` name of the control
 * `group` name of the control group/class
 * `_type` type of the control
 * `min` minimum value of control (if control is integer)
 * `max` maximum value of control (if control is integer)
 * `def` default value of control (if control is integer)
 * `step` increment steps (if control is integer)
 * `menu` menu array
 * `menu_size` menu size
 * @since 0.20
 */
pub struct video_controls_s {
    name: String,
    group: String,
    _type: video_control_type_t,
    min: i64,
    max: i64,
    def: i64,
    step: u64,
    menu_size: u32,
    menu: video_control_menu_t,
    // video drivers may add extra private data in the end of this struct
    next: Box<dyn std::any::Any>
}
#[doc = "store video controls\n * `name` name of the control\n * `group` name of the control group/class\n * `_type` type of the control\n * `min` minimum value of control (if control is integer)\n * `max` maximum value of control (if control is integer)\n * `def` default value of control (if control is integer)\n * `step` increment steps (if control is intege)r\n * `menu` menu array\n * `menu_size` menu size\n * @since 0.20"]
pub type video_controls_t = video_controls_s;

/** store a video resolution
 * `width` width of the video window
 * `height` length of the video window
 * `max_fps` maximum streaming speed, in frames per second
 * @since 0.22
 */
pub struct video_resolution_s {
    width: u32,
    height: u32,
    max_fps: c_float
}

#[doc = "consistently compute fourcc values across architectures\n * (adapted from v4l2 specification)\n * @since 0.11"]
#[macro_export]
macro_rules! zbar_fourcc {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        ($a     as i64             |
        ((($b)  as i64) << 8)      |
        ((($c)  as i64) << 16)     |
        (((($d) as i64) << 24)    ))
    };
}

/** parse a fourcc string into its encoded integer value.
 * @since 0.11
 */
#[inline]
pub fn zbar_fourcc_parse (format: Vec<i64>) -> i64 {
    let mut fourcc = 0;
    if format.is_empty(){
        for i in 0..4{
            fourcc |= format[i] << (i*8)
        }
    }
    return fourcc;
}

/*@}*/

pub use crate::src::zbar_symbol_s;
pub type zbar_symbol_t = zbar_symbol_s;

pub use crate::src::zbar_symbol_set_s;
pub type zbar_symbol_set_t = zbar_symbol_set_s;

/*------------------------------------------------------------*/
/** @name Symbol interface
 * decoded barcode symbol result object.  stores type, data, and image
 * location of decoded symbol.  all memory is owned by the library
 */
/*@{*/

/** @typedef zbar_symbol_t
 * opaque decoded symbol object.
 */


/*@}*/

/*------------------------------------------------------------*/
/** @name Image interface
 * stores image data samples along with associated format and size
 * metadata
 */
/*@{*/
pub struct zbar_image_s{}
/**
 * zbar_image_t: opaque image object.
 */
pub type zbar_image_t = zbar_image_s;
#[doc = " cleanup handler callback function.\n called to free sample data when an image is destroyed."]
pub type zbar_image_cleanup_handler_t =
    ::std::option::Option<fn(image: *mut zbar_image_t)>;
#[doc = " data handler callback function.\n called when decoded symbol results are available for an image"]
pub type zbar_image_data_handler_t = ::std::option::Option<
    fn(image: zbar_image_t, userdata: Box<dyn std::any::Any>),
>;

#[derive(Debug, Copy, Clone)]
pub struct zbar_processor_s{}
#[doc = " opaque standalone processor object."]
pub type zbar_processor_t = zbar_processor_s;

#[derive(Debug, Copy, Clone)]
pub struct zbar_video_s{}
#[doc = " opaque video object."]
pub type zbar_video_t = zbar_video_s;

#[derive(Debug, Copy, Clone)]
pub struct zbar_window_s{}
#[doc = " opaque window object."]
pub type zbar_window_t = zbar_window_s;

#[derive(Debug, Copy, Clone)]
pub struct zbar_image_scanner_s{}
#[doc = " opaque image scanner object."]
pub type zbar_image_scanner_t = zbar_image_scanner_s;

#[derive(Debug, Copy, Clone)]
pub struct zbar_decoder_s{}
#[doc = " opaque decoder object."]
pub type zbar_decoder_t = zbar_decoder_s;
#[doc = " decoder data handler callback function.\n called by decoder when new data has just been decoded"]
pub type zbar_decoder_handler_t =
    ::std::option::Option<fn(decoder: *mut zbar_decoder_t)>;

pub struct zbar_scanner_s{}
#[doc = " opaque scanner object."]
pub type zbar_scanner_t = zbar_scanner_s;
#[link(name="zbar")]

pub mod Exception;
pub mod Decoder;
pub mod cxx_std_need;
pub mod src;
#[cfg(test)]
mod tests;
use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};
pub use Exception::*;
pub use Decoder::*;
pub use src::*;