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
 * - the "GTK+ 2.x widget" may be used with GTK GUI applications.  a
 *   Python wrapper is included for PyGtk
 * - the @ref zbar::QZBar "Qt4 widget" may be used with Qt GUI
 *   applications
 * - the Processor interface (in @ref c-processor "C" or @ref
 *   zbar::Processor "C++") adds a scanning window to an application
 *   with no GUI.
 *
 * @section mid-level Intermediate Interfaces
 *
 * building blocks used to construct high-level interfaces:
 * - the ImageScanner (in @ref c-imagescanner "C" or @ref
 *   zbar::ImageScanner "C++") looks for barcodes in a library defined
 *   image object
 * - the Window abstraction (in @ref c-window "C" or @ref
 *   zbar::Window "C++") sinks library images, displaying them on the
 *   platform display
 * - the Video abstraction (in @ref c-video "C" or @ref zbar::Video
 *   "C++") sources library images from a video device
 *
 * @section low-level Low-Level Interfaces
 *
 * direct interaction with barcode scanning and decoding:
 * - the Scanner (in @ref c-scanner "C" or @ref zbar::Scanner "C++")
 *   looks for barcodes in a linear intensity sample stream
 * - the Decoder (in @ref c-decoder "C" or @ref zbar::Decoder "C++")
 *   extracts barcodes from a stream of bar and space widths
 */

/** @name Global library interfaces */
/*@{*/

extern crate libc;
use libc::{c_char, c_float, c_int, c_uint, c_ulong, c_ulonglong, c_void};

/** "color" of element: bar or space. */
#[repr(C)]
pub enum zbar_color_e{
    /**< light area or space between bars */    ZBAR_SPACE = 0,
    /**< dark area or colored bar segment */    ZBAR_BAR = 1,
}
#[doc = " \"color\" of element: bar or space. "]
pub type zbar_color_t = zbar_color_e;


/** decoded symbol type. */
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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
#[repr(C)]
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

#[repr(C)]
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
#[repr(C)]
pub struct video_control_menu_e {
    name: *mut c_char,
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
#[repr(C)]
pub struct video_controls_s {
    name: *mut c_char,
    group: *mut c_char,
    _type: video_control_type_t,
    min: i64,
    max: i64,
    def: i64,
    step: u64,
    menu_size: u32,
    menu: video_control_menu_t,
    // video drivers may add extra private data in the end of this struct
    next: *mut c_void
}
#[doc = "store video controls\n * `name` name of the control\n * `group` name of the control group/class\n * `_type` type of the control\n * `min` minimum value of control (if control is integer)\n * `max` maximum value of control (if control is integer)\n * `def` default value of control (if control is integer)\n * `step` increment steps (if control is intege)r\n * `menu` menu array\n * `menu_size` menu size\n * @since 0.20"]
pub type video_controls_t = video_controls_s;

/** store a video resolution
 * `width` width of the video window
 * `height` length of the video window
 * `max_fps` maximum streaming speed, in frames per second
 * @since 0.22
 */
#[repr(C)]
pub struct video_resolution_s {
    width: u32,
    height: u32,
    max_fps: c_float
}

#[link(name="zbar")]
extern {
    #[doc = "* retrieve runtime library version information.\n * `major` set to the running major version (unless NULL)\n* `minor` set to the running minor version (unless NULL)\n* returns 0\n"]
    pub fn zbar_version(major:*mut c_ulonglong, minor: *mut c_ulonglong, patch: *mut c_ulonglong) -> c_int;
}

#[link(name="zbar")]
extern {
    #[doc = " set global library debug level.\n`verbosity` desired debug level.  higher values create more spew"]
    pub fn zbar_set_verbosity(verbosity: c_int);
}
#[link(name="zbar")]
extern {
    #[doc = " increase global library debug level.\n eg, for -vvvv"]
    pub fn zbar_increase_verbosity();
}
#[link(name="zbar")]
extern {
    #[doc = " retrieve string name for symbol encoding.\n `sym` symbol type encoding\n @returns the static string name for the specified symbol type,\n or \"UNKNOWN\" if the encoding is not recognized"]
    pub fn zbar_get_symbol_name(sym: zbar_symbol_type_t) -> *const c_char;
}
#[link(name="zbar")]
extern {
    #[doc = " retrieve string name for addon encoding.\n `sym` symbol type encoding\n @returns static string name for any addon, or the empty string\n if no addons were decoded\n @deprecated in 0.11"]
    pub fn zbar_get_addon_name(sym: zbar_symbol_type_t) -> *const c_char;
}
#[link(name="zbar")]
extern {
    #[doc = " retrieve string name for configuration setting.\n `config` setting to name\n @returns static string name for config,\n or the empty string if value is not a known config"]
    pub fn zbar_get_config_name(config: zbar_config_t) -> *const c_char;
}
#[link(name="zbar")]
extern {
    #[doc = " retrieve string name for modifier.\n `modifier` flag to name\n @returns static string name for modifier,\n or the empty string if the value is not a known flag"]
    pub fn zbar_get_modifier_name(modifier: zbar_modifier_t) -> *const c_char;
}
#[link(name="zbar")]
extern {
    #[doc = " retrieve string name for orientation.\n `orientation` orientation encoding\n @returns the static string name for the specified orientation,\n or \"UNKNOWN\" if the orientation is not recognized\n @since 0.11"]
    pub fn zbar_get_orientation_name(
        orientation: zbar_orientation_t,
    ) -> *const c_char;
}
#[link(name="zbar")]
extern {
    #[doc = " parse a configuration string of the form \"[symbology.]config[=value]\".\n the config must match one of the recognized names.\n the symbology, if present, must match one of the recognized names.\n if symbology is unspecified, it will be set to 0.\n if value is unspecified it will be set to 1.\n returns 0 if the config is parsed successfully, 1 otherwise\n @since 0.4"]
    pub fn zbar_parse_config(
        config_string: *const c_char,
        symbology: *mut zbar_symbol_type_t,
        config: *mut zbar_config_t,
        value: *mut c_int,
    ) -> c_int;
}

#[doc = "consistently compute fourcc values across architectures\n * (adapted from v4l2 specification)\n * @since 0.11"]
#[macro_export]
macro_rules! zbar_fourcc {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        ($a as ::std::os::raw::c_ulong             |
        ((($b) as ::std::os::raw::c_ulong) << 8)   |
        ((($c) as ::std::os::raw::c_ulong) << 16)  |
        (((($d) as ::std::os::raw::c_ulong) << 24)  ))
    };
}

/** parse a fourcc string into its encoded integer value.
 * @since 0.11
 */
#[inline]
pub fn zbar_fourcc_parse (format: *const c_char) -> c_ulong {
    let mut fourcc: c_ulong = 0;
    if (format)!=std::ptr::null(){
        for i in 0..4{
            if format.wrapping_add(i)!=std::ptr::null(){
                fourcc = (format.wrapping_add(i) as c_ulong) << (i*8);
            }
        }
    }
    return fourcc;
}

#[link(name="zbar")]
extern {
    #[doc = " @internal type unsafe error API (don't use)"]
    pub fn _zbar_error_spew(
        object: *const c_void,
        verbosity: c_int,
    ) -> c_int;
}
#[link(name="zbar")]
extern {
    pub fn _zbar_error_string(
        object: *const c_void,
        verbosity: c_int,
    ) -> *const c_char;
}
#[link(name="zbar")]
extern {
    pub fn _zbar_get_error_code(object: *const c_void) -> zbar_error_t;
}

/*@}*/

#[repr(C)]
pub struct zbar_symbol_s{
    _unused: [u8; 0]
}
pub type zbar_symbol_t = zbar_symbol_s;

#[repr(C)]
pub struct zbar_symbol_set_s{
    _unused: [u8; 0]
}
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

#[link(name="zbar")]
extern {
    #[doc = "symbol reference count manipulation.\n * increment the reference count when you store a new reference to the\n * symbol.  decrement when the reference is no longer used.  do not\n * refer to the symbol once the count is decremented and the\n * containing image has been recycled or destroyed.\n * note the containing image holds a reference to the symbol, so you\n * only need to use this if you keep a symbol after the image has been\n * destroyed or reused.\n * @since 0.9"]
    pub fn zbar_symbol_ref(synbol: *const zbar_symbol_t, refs: c_int) -> c_void;
}
#[link(name="zbar")]
extern {
    #[doc = "retrieve type of decoded symbol.\n * returns the symbol type"]
    pub fn zbar_symbol_get_type(symbol: *const zbar_symbol_t) -> zbar_symbol_type_t;
}
#[link(name="zbar")]
extern {
    #[doc = " retrieve symbology boolean config settings.\n @returns a bitmask indicating which configs were set for the detected\n symbology during decoding.\n @since 0.11"]
    pub fn zbar_symbol_get_configs(symbol: *const zbar_symbol_t) -> c_uint;
}
#[link(name="zbar")]
extern {
    #[doc = " retrieve symbology modifier flag settings.\n @returns a bitmask indicating which characteristics were detected\n during decoding.\n @since 0.11"]
    pub fn zbar_symbol_get_modifiers(symbol: *const zbar_symbol_t) -> c_uint;
}
#[link(name="zbar")]
extern {
    #[doc = " retrieve data decoded from symbol.\n @returns the data string"]
    pub fn zbar_symbol_get_data(symbol: *const zbar_symbol_t) -> *const c_char;
}
#[link(name="zbar")]
extern {
    #[doc = " retrieve length of binary data.\n @returns the length of the decoded data"]
    pub fn zbar_symbol_get_data_length(symbol: *const zbar_symbol_t) -> c_uint;
}
#[link(name="zbar")]
extern {
    #[doc = " retrieve a symbol confidence metric.\n @returns an unscaled, relative quantity: larger values are better\n than smaller values, where \"large\" and \"small\" are application\n dependent.\n @note expect the exact definition of this quantity to change as the\n metric is refined.  currently, only the ordered relationship\n between two values is defined and will remain stable in the future\n @since 0.9"]
    pub fn zbar_symbol_get_quality(symbol: *const zbar_symbol_t) -> c_int;
}
#[link(name="zbar")]
extern {
    #[doc = " retrieve current cache count.  when the cache is enabled for the\n image_scanner this provides inter-frame reliability and redundancy\n information for video streams.\n @returns < 0 if symbol is still uncertain.\n @returns 0 if symbol is newly verified.\n @returns > 0 for duplicate symbols"]
    pub fn zbar_symbol_get_count(symbol: *const zbar_symbol_t) -> c_int;
}
#[link(name="zbar")]
extern {
    #[doc = " retrieve the number of points in the location polygon.  the\n location polygon defines the image area that the symbol was\n extracted from.\n @returns the number of points in the location polygon\n @note this is currently not a polygon, but the scan locations\n where the symbol was decoded"]
    pub fn zbar_symbol_get_loc_size(symbol: *const zbar_symbol_t) -> c_uint;
}
#[link(name="zbar")]
extern {
    #[doc = " retrieve location polygon x-coordinates.\n points are specified by 0-based index.\n @returns the x-coordinate for a point in the location polygon.\n @returns -1 if index is out of range"]
    pub fn zbar_symbol_get_loc_x(
        symbol: *const zbar_symbol_t,
        index: c_uint,
    ) -> c_int;
}
#[link(name="zbar")]
extern {
    #[doc = " retrieve location polygon y-coordinates.\n points are specified by 0-based index.\n @returns the y-coordinate for a point in the location polygon.\n @returns -1 if index is out of range"]
    pub fn zbar_symbol_get_loc_y(
        symbol: *const zbar_symbol_t,
        index: c_uint,
    ) -> c_int;
}

//extern {pub struct zbar_orientation_t;}

#[link(name="zbar")]
extern "C" {
    #[doc = " retrieve general orientation of decoded symbol.\n @returns a coarse, axis-aligned indication of symbol orientation or\n ::ZBAR_ORIENT_UNKNOWN if unknown\n @since 0.11"]
    pub fn zbar_symbol_get_orientation(symbol: *const zbar_symbol_t) -> zbar_orientation_t;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " iterate the set to which this symbol belongs (there can be only one).\n @returns the next symbol in the set, or\n @returns NULL when no more results are available"]
    pub fn zbar_symbol_next(symbol: *const zbar_symbol_t) -> *const zbar_symbol_t;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " retrieve components of a composite result.\n @returns the symbol set containing the components\n @returns NULL if the symbol is already a physical symbol\n @since 0.10"]
    pub fn zbar_symbol_get_components(symbol: *const zbar_symbol_t) -> *const zbar_symbol_set_t;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " iterate components of a composite result.\n @returns the first physical component symbol of a composite result\n @returns NULL if the symbol is already a physical symbol\n @since 0.10"]
    pub fn zbar_symbol_first_component(symbol: *const zbar_symbol_t) -> *const zbar_symbol_t;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " print XML symbol element representation to user result buffer.\n @see http://zbar.sourceforge.net/2008/barcode.xsd for the schema.\n @param symbol is the symbol to print\n @param buffer is the inout result pointer, it will be reallocated\n with a larger size if necessary.\n @param buflen is inout length of the result buffer.\n @returns the buffer pointer\n @since 0.6"]
    pub fn zbar_symbol_xml(
        symbol: *const zbar_symbol_t,
        buffer: *mut *mut c_char,
        buflen: *mut c_uint,
    ) -> *mut c_char;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " reference count manipulation.\n increment the reference count when you store a new reference.\n decrement when the reference is no longer used.  do not refer to\n the object any longer once references have been released.\n @since 0.10"]
    pub fn zbar_symbol_set_ref(symbols: *const zbar_symbol_set_t, refs: c_int);
}
#[link(name="zbar")]
extern "C" {
    #[doc = " retrieve set size.\n @returns the number of symbols in the set.\n @since 0.10"]
    pub fn zbar_symbol_set_get_size(symbols: *const zbar_symbol_set_t) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " set iterator.\n @returns the first decoded symbol result in a set\n @returns NULL if the set is empty\n @since 0.10"]
    pub fn zbar_symbol_set_first_symbol(symbols: *const zbar_symbol_set_t) -> *const zbar_symbol_t;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " raw result iterator.\n @returns the first decoded symbol result in a set, *before* filtering\n @returns NULL if the set is empty\n @since 0.11"]
    pub fn zbar_symbol_set_first_unfiltered(
        symbols: *const zbar_symbol_set_t,
    ) -> *const zbar_symbol_t;
}

/*@}*/

/*------------------------------------------------------------*/
/** @name Image interface
 * stores image data samples along with associated format and size
 * metadata
 */
/*@{*/
#[repr(C)]
pub struct zbar_image_s{
    _unused: [u8; 0]
}
/**
 * zbar_image_t: opaque image object.
 */
pub type zbar_image_t = zbar_image_s;
#[doc = " cleanup handler callback function.\n called to free sample data when an image is destroyed."]
pub type zbar_image_cleanup_handler_t =
    ::std::option::Option<unsafe extern "C" fn(image: *mut zbar_image_t)>;
#[doc = " data handler callback function.\n called when decoded symbol results are available for an image"]
pub type zbar_image_data_handler_t = ::std::option::Option<
    unsafe extern "C" fn(image: *mut zbar_image_t, userdata: *const c_void),
>;
#[link(name="zbar")]
extern "C" {
    #[doc = " new image constructor.\n @returns a new image object with uninitialized data and format.\n this image should be destroyed (using zbar_image_destroy()) as\n soon as the application is finished with it"]
    pub fn zbar_image_create() -> *mut zbar_image_t;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " image destructor.  all images created by or returned to the\n application should be destroyed using this function.  when an image\n is destroyed, the associated data cleanup handler will be invoked\n if available\n @note make no assumptions about the image or the data buffer.\n they may not be destroyed/cleaned immediately if the library\n is still using them.  if necessary, use the cleanup handler hook\n to keep track of image data buffers"]
    pub fn zbar_image_destroy(image: *mut zbar_image_t);
}
#[link(name="zbar")]
extern "C" {
    #[doc = " image reference count manipulation.\n increment the reference count when you store a new reference to the\n image.  decrement when the reference is no longer used.  do not\n refer to the image any longer once the count is decremented.\n zbar_image_ref(image, -1) is the same as zbar_image_destroy(image)\n @since 0.5"]
    pub fn zbar_image_ref(image: *mut zbar_image_t, refs: c_int);
}
#[link(name="zbar")]
extern "C" {
    #[doc = " image format conversion.  refer to the documentation for supported\n image formats\n @returns a @em new image with the sample data from the original image\n converted to the requested format.  the original image is\n unaffected.\n @note the converted image size may be rounded (up) due to format\n constraints"]
    pub fn zbar_image_convert(
        image: *const zbar_image_t,
        format: c_ulong,
    ) -> *mut zbar_image_t;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " image format conversion with crop/pad.\n if the requested size is larger than the image, the last row/column\n are duplicated to cover the difference.  if the requested size is\n smaller than the image, the extra rows/columns are dropped from the\n right/bottom.\n @returns a @em new image with the sample data from the original\n image converted to the requested format and size.\n @note the image is @em not scaled\n @see zbar_image_convert()\n @since 0.4"]
    pub fn zbar_image_convert_resize(
        image: *const zbar_image_t,
        format: c_ulong,
        width: c_uint,
        height: c_uint,
    ) -> *mut zbar_image_t;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " retrieve the image format.\n @returns the fourcc describing the format of the image sample data"]
    pub fn zbar_image_get_format(image: *const zbar_image_t) -> c_ulong;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " retrieve a \"sequence\" (page/frame) number associated with this image.\n @since 0.6"]
    pub fn zbar_image_get_sequence(image: *const zbar_image_t) -> c_uint;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " retrieve the width of the image.\n @returns the width in sample columns"]
    pub fn zbar_image_get_width(image: *const zbar_image_t) -> c_uint;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " retrieve the height of the image.\n @returns the height in sample rows"]
    pub fn zbar_image_get_height(image: *const zbar_image_t) -> c_uint;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " retrieve both dimensions of the image.\n fills in the width and height in samples"]
    pub fn zbar_image_get_size(
        image: *const zbar_image_t,
        width: *mut c_uint,
        height: *mut c_uint,
    );
}
#[link(name="zbar")]
extern "C" {
    #[doc = " retrieve the crop rectangle.\n fills in the image coordinates of the upper left corner and size\n of an axis-aligned rectangular area of the image that will be scanned.\n defaults to the full image\n @since 0.11"]
    pub fn zbar_image_get_crop(
        image: *const zbar_image_t,
        x: *mut c_uint,
        y: *mut c_uint,
        width: *mut c_uint,
        height: *mut c_uint,
    );
}
#[link(name="zbar")]
extern "C" {
    #[doc = " return the image sample data.  the returned data buffer is only\n valid until zbar_image_destroy() is called"]
    pub fn zbar_image_get_data(image: *const zbar_image_t) -> *const c_void;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " return the size of image data.\n @since 0.6"]
    pub fn zbar_image_get_data_length(img: *const zbar_image_t) -> c_ulong;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " retrieve the decoded results.\n @returns the (possibly empty) set of decoded symbols\n @returns NULL if the image has not been scanned\n @since 0.10"]
    pub fn zbar_image_get_symbols(image: *const zbar_image_t) -> *const zbar_symbol_set_t;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " associate the specified symbol set with the image, replacing any\n existing results.  use NULL to release the current results from the\n image.\n @see zbar_image_scanner_recycle_image()\n @since 0.10"]
    pub fn zbar_image_set_symbols(image: *mut zbar_image_t, symbols: *const zbar_symbol_set_t);
}
#[link(name="zbar")]
extern "C" {
    #[doc = " image_scanner decode result iterator.\n @returns the first decoded symbol result for an image\n or NULL if no results are available"]
    pub fn zbar_image_first_symbol(image: *const zbar_image_t) -> *const zbar_symbol_t;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " specify the fourcc image format code for image sample data.\n refer to the documentation for supported formats.\n @note this does not convert the data!\n (see zbar_image_convert() for that)"]
    pub fn zbar_image_set_format(image: *mut zbar_image_t, format: c_ulong);
}
#[link(name="zbar")]
extern "C" {
    #[doc = " associate a \"sequence\" (page/frame) number with this image.\n @since 0.6"]
    pub fn zbar_image_set_sequence(image: *mut zbar_image_t, sequence_num: c_uint);
}
#[link(name="zbar")]
extern "C" {
    #[doc = " specify the pixel size of the image.\n @note this also resets the crop rectangle to the full image\n (0, 0, width, height)\n @note this does not affect the data!"]
    pub fn zbar_image_set_size(
        image: *mut zbar_image_t,
        width: c_uint,
        height: c_uint,
    );
}
#[link(name="zbar")]
extern "C" {
    #[doc = " specify a rectangular region of the image to scan.\n the rectangle will be clipped to the image boundaries.\n defaults to the full image specified by zbar_image_set_size()"]
    pub fn zbar_image_set_crop(
        image: *mut zbar_image_t,
        x: c_uint,
        y: c_uint,
        width: c_uint,
        height: c_uint,
    );
}
#[link(name="zbar")]
extern "C" {
    #[doc = " specify image sample data.  when image data is no longer needed by\n the library the specific data cleanup handler will be called\n (unless NULL)\n @note application image data will not be modified by the library"]
    pub fn zbar_image_set_data(
        image: *mut zbar_image_t,
        data: *const c_void,
        data_byte_length: c_ulong,
        cleanup_hndlr: zbar_image_cleanup_handler_t,
    );
}
#[link(name="zbar")]
extern "C" {
    #[doc = " built-in cleanup handler.\n passes the image data buffer to free()"]
    pub fn zbar_image_free_data(image: *mut zbar_image_t);
}
#[link(name="zbar")]
extern "C" {
    #[doc = " associate user specified data value with an image.\n @since 0.5"]
    pub fn zbar_image_set_userdata(image: *mut zbar_image_t, userdata: *mut c_void);
}
#[link(name="zbar")]
extern "C" {
    #[doc = " return user specified data value associated with the image.\n @since 0.5"]
    pub fn zbar_image_get_userdata(image: *const zbar_image_t) -> *mut c_void;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " dump raw image data to a file for debug.\n the data will be prefixed with a 16 byte header consisting of:\n   - 4 bytes uint = 0x676d697a (\"zimg\")\n   - 4 bytes format fourcc\n   - 2 bytes width\n   - 2 bytes height\n   - 4 bytes size of following image data in bytes\n this header can be dumped w/eg:\n @verbatim\nod -Ax -tx1z -N16 -w4 [file]\n@endverbatim\n for some formats the image can be displayed/converted using\n ImageMagick, eg:\n @verbatim\ndisplay -size 640x480+16 [-depth ?] [-sampling-factor ?x?] \\\n{GRAY,RGB,UYVY,YUV}:[file]\n@endverbatim\n\n @param image the image object to dump\n @param filebase base filename, appended with \".XXXX.zimg\" where\n XXXX is the format fourcc\n @returns 0 on success or a system error code on failure"]
    pub fn zbar_image_write(
        image: *const zbar_image_t,
        filebase: *const c_char,
    ) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " read back an image in the format written by zbar_image_write()\n @note TBD"]
    pub fn zbar_image_read(filename: *mut c_char) -> *mut zbar_image_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zbar_processor_s{
    _unused: [u8; 0]
}
#[doc = " opaque standalone processor object."]
pub type zbar_processor_t = zbar_processor_s;
#[link(name="zbar")]
extern "C" {
    #[doc = " constructor.\n if threaded is set and threading is available the processor\n will spawn threads where appropriate to avoid blocking and\n improve responsiveness"]
    pub fn zbar_processor_create(threaded: c_int) -> *mut zbar_processor_t;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " destructor.  cleans up all resources associated with the processor"]
    pub fn zbar_processor_destroy(processor: *mut zbar_processor_t);
}
#[link(name="zbar")]
extern "C" {
    #[doc = " (re)initialization.\n opens a video input device and/or prepares to display output"]
    pub fn zbar_processor_init(
        processor: *mut zbar_processor_t,
        video_device: *const c_char,
        enable_display: c_int,
    ) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " request a preferred size for the video image from the device.\n the request may be adjusted or completely ignored by the driver.\n @note must be called before zbar_processor_init()\n @since 0.6"]
    pub fn zbar_processor_request_size(
        processor: *mut zbar_processor_t,
        width: c_uint,
        height: c_uint,
    ) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " request a preferred video driver interface version for\n debug/testing.\n @note must be called before zbar_processor_init()\n @since 0.6"]
    pub fn zbar_processor_request_interface(
        processor: *mut zbar_processor_t,
        version: c_int,
    ) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " request a preferred video I/O mode for debug/testing.  You will\n get errors if the driver does not support the specified mode.\n @verbatim\n0 = auto-detect\n1 = force I/O using read()\n2 = force memory mapped I/O using mmap()\n3 = force USERPTR I/O (v4l2 only)\n@endverbatim\n @note must be called before zbar_processor_init()\n @since 0.7"]
    pub fn zbar_processor_request_iomode(
        video: *mut zbar_processor_t,
        iomode: c_int,
    ) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " force specific input and output formats for debug/testing.\n @note must be called before zbar_processor_init()"]
    pub fn zbar_processor_force_format(
        processor: *mut zbar_processor_t,
        input_format: c_ulong,
        output_format: c_ulong,
    ) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " setup result handler callback.\n the specified function will be called by the processor whenever\n new results are available from the video stream or a static image.\n pass a NULL value to disable callbacks.\n @param processor the object on which to set the handler.\n @param handler the function to call when new results are available.\n @param userdata is set as with zbar_processor_set_userdata().\n @returns the previously registered handler"]
    pub fn zbar_processor_set_data_handler(
        processor: *mut zbar_processor_t,
        handler: zbar_image_data_handler_t,
        userdata: *const c_void,
    ) -> zbar_image_data_handler_t;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " associate user specified data value with the processor.\n @since 0.6"]
    pub fn zbar_processor_set_userdata(
        processor: *mut zbar_processor_t,
        userdata: *mut c_void,
    );
}
#[link(name="zbar")]
extern "C" {
    #[doc = " return user specified data value associated with the processor.\n @since 0.6"]
    pub fn zbar_processor_get_userdata(
        processor: *const zbar_processor_t,
    ) -> *mut c_void;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " set config for indicated symbology (0 for all) to specified value.\n @returns 0 for success, non-0 for failure (config does not apply to\n specified symbology, or value out of range)\n @see zbar_decoder_set_config()\n @since 0.4"]
    pub fn zbar_processor_set_config(
        processor: *mut zbar_processor_t,
        symbology: zbar_symbol_type_t,
        config: zbar_config_t,
        value: c_int,
    ) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " set video control value\n @returns 0 for success, non-0 for failure\n @since 0.20\n @see zbar_video_set_control()"]
    pub fn zbar_processor_set_control(
        processor: *mut zbar_processor_t,
        control_name: *const c_char,
        value: c_int,
    ) -> c_int;
}#[link(name="zbar")]
extern "C" {
    #[doc = " get video control value\n @returns 0 for success, non-0 for failure\n @since 0.20\n @see zbar_video_get_control()"]
    pub fn zbar_processor_get_control(
        processor: *mut zbar_processor_t,
        control_name: *const c_char,
        value: *mut c_int,
    ) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " retrieve the current state of the output window.\n @returns 1 if the output window is currently displayed, 0 if not.\n @returns -1 if an error occurs"]
    pub fn zbar_processor_is_visible(processor: *mut zbar_processor_t) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " show or hide the display window owned by the library.\n the size will be adjusted to the input size"]
    pub fn zbar_processor_set_visible(
        processor: *mut zbar_processor_t,
        visible: c_int,
    ) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " control the processor in free running video mode.\n only works if video input is initialized. if threading is in use,\n scanning will occur in the background, otherwise this is only\n useful wrapping calls to zbar_processor_user_wait(). if the\n library output window is visible, video display will be enabled."]
    pub fn zbar_processor_set_active(
        processor: *mut zbar_processor_t,
        active: c_int,
    ) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " retrieve decode results for last scanned image/frame.\n @returns the symbol set result container or NULL if no results are\n available\n @note the returned symbol set has its reference count incremented;\n ensure that the count is decremented after use\n @since 0.10"]
    pub fn zbar_processor_get_results(
        processor: *const zbar_processor_t,
    ) -> *const zbar_symbol_set_t;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " wait for input to the display window from the user\n (via mouse or keyboard).\n @returns >0 when input is received, 0 if timeout ms expired\n with no input or -1 in case of an error"]
    pub fn zbar_processor_user_wait(
        processor: *mut zbar_processor_t,
        timeout: c_int,
    ) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " process from the video stream until a result is available,\n or the timeout (in milliseconds) expires.\n specify a timeout of -1 to scan indefinitely\n (zbar_processor_set_active() may still be used to abort the scan\n from another thread).\n if the library window is visible, video display will be enabled.\n @note that multiple results may still be returned (despite the\n name).\n @returns >0 if symbols were successfully decoded,\n 0 if no symbols were found (ie, the timeout expired)\n or -1 if an error occurs"]
    pub fn zbar_process_one(
        processor: *mut zbar_processor_t,
        timeout: c_int,
    ) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " process the provided image for barcodes.\n if the library window is visible, the image will be displayed.\n @returns >0 if symbols were successfully decoded,\n 0 if no symbols were found or -1 if an error occurs"]
    pub fn zbar_process_image(
        processor: *mut zbar_processor_t,
        image: *mut zbar_image_t,
    ) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " enable dbus IPC API.\n @returns 0 successful"]
    pub fn zbar_processor_request_dbus(
        proc_: *mut zbar_processor_t,
        req_dbus_enabled: c_int,
    ) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zbar_video_s{
    _unused: [u8; 0]
}
#[doc = " opaque video object."]
pub type zbar_video_t = zbar_video_s;
#[link(name="zbar")]
extern "C" {
    #[doc = " constructor."]
    pub fn zbar_video_create() -> *mut zbar_video_t;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " destructor."]
    pub fn zbar_video_destroy(video: *mut zbar_video_t);
}
#[link(name="zbar")]
extern "C" {
    #[doc = " open and probe a video device.\n the device specified by platform specific unique name\n (v4l device node path in *nix eg \"/dev/video\",\n  DirectShow DevicePath property in windows).\n @returns 0 if successful or -1 if an error occurs"]
    pub fn zbar_video_open(
        video: *mut zbar_video_t,
        device: *const c_char,
    ) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " retrieve file descriptor associated with open *nix video device\n useful for using select()/poll() to tell when new images are\n available (NB v4l2 only!!).\n @returns the file descriptor or -1 if the video device is not open\n or the driver only supports v4l1"]
    pub fn zbar_video_get_fd(video: *const zbar_video_t) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " request a preferred size for the video image from the device.\n the request may be adjusted or completely ignored by the driver.\n @returns 0 if successful or -1 if the video device is already\n initialized\n @since 0.6"]
    pub fn zbar_video_request_size(
        video: *mut zbar_video_t,
        width: c_uint,
        height: c_uint,
    ) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " request a preferred driver interface version for debug/testing.\n @note must be called before zbar_video_open()\n @since 0.6"]
    pub fn zbar_video_request_interface(
        video: *mut zbar_video_t,
        version: c_int,
    ) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " request a preferred I/O mode for debug/testing.  You will get\n errors if the driver does not support the specified mode.\n @verbatim\n0 = auto-detect\n1 = force I/O using read()\n2 = force memory mapped I/O using mmap()\n3 = force USERPTR I/O (v4l2 only)\n@endverbatim\n @note must be called before zbar_video_open()\n @since 0.7"]
    pub fn zbar_video_request_iomode(
        video: *mut zbar_video_t,
        iomode: c_int,
    ) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " retrieve current output image width.\n @returns the width or 0 if the video device is not open"]
    pub fn zbar_video_get_width(video: *const zbar_video_t) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " retrieve current output image height.\n @returns the height or 0 if the video device is not open"]
    pub fn zbar_video_get_height(video: *const zbar_video_t) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " initialize video using a specific format for debug.\n use zbar_negotiate_format() to automatically select and initialize\n the best available format"]
    pub fn zbar_video_init(
        video: *mut zbar_video_t,
        format: c_ulong,
    ) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " start/stop video capture.\n all buffered images are retired when capture is disabled.\n @returns 0 if successful or -1 if an error occurs"]
    pub fn zbar_video_enable(
        video: *mut zbar_video_t,
        enable: c_int,
    ) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " retrieve next captured image.  blocks until an image is available.\n @returns NULL if video is not enabled or an error occurs"]
    pub fn zbar_video_next_image(video: *mut zbar_video_t) -> *mut zbar_image_t;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " set video control value (integer).\n @returns 0 for success, non-0 for failure\n @since 0.20\n @see zbar_processor_set_control()"]
    pub fn zbar_video_set_control(
        video: *mut zbar_video_t,
        control_name: *const c_char,
        value: c_int,
    ) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " get video control value (integer).\n @returns 0 for success, non-0 for failure\n @since 0.20\n @see zbar_processor_get_control()"]
    pub fn zbar_video_get_control(
        video: *mut zbar_video_t,
        control_name: *const c_char,
        value: *mut c_int,
    ) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " get available controls from video source\n @returns 0 for success, non-0 for failure\n @since 0.20"]
    pub fn zbar_video_get_controls(
        video: *const zbar_video_t,
        index: c_int,
    ) -> *mut video_controls_s;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " get available video resolutions from video source\n @returns 0 for success, non-0 for failure\n @since 0.22"]
    pub fn zbar_video_get_resolutions(
        vdo: *const zbar_video_t,
        index: c_int,
    ) -> *mut video_resolution_s;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zbar_window_s{
    _unused: [u8; 0]
}
#[doc = " opaque window object."]
pub type zbar_window_t = zbar_window_s;
#[link(name="zbar")]
extern "C" {
    #[doc = " constructor."]
    pub fn zbar_window_create() -> *mut zbar_window_t;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " destructor."]
    pub fn zbar_window_destroy(window: *mut zbar_window_t);
}
#[link(name="zbar")]
extern "C" {
    #[doc = " associate reader with an existing platform window.\n This can be any \"Drawable\" for X Windows or a \"HWND\" for windows.\n input images will be scaled into the output window.\n pass NULL to detach from the resource, further input will be\n ignored"]
    pub fn zbar_window_attach(
        window: *mut zbar_window_t,
        x11_display_w32_hwnd: *mut c_void,
        x11_drawable: c_ulong,
    ) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " control content level of the reader overlay.\n the overlay displays graphical data for informational or debug\n purposes.  higher values increase the level of annotation (possibly\n decreasing performance). @verbatim\n0 = disable overlay\n1 = outline decoded symbols (default)\n2 = also track and display input frame rate\n@endverbatim"]
    pub fn zbar_window_set_overlay(window: *mut zbar_window_t, level: c_int);
}
#[link(name="zbar")]
extern "C" {
    #[doc = " retrieve current content level of reader overlay.\n @see zbar_window_set_overlay()\n @since 0.10"]
    pub fn zbar_window_get_overlay(window: *const zbar_window_t) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " draw a new image into the output window."]
    pub fn zbar_window_draw(
        window: *mut zbar_window_t,
        image: *mut zbar_image_t,
    ) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " redraw the last image (exposure handler)."]
    pub fn zbar_window_redraw(window: *mut zbar_window_t) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " resize the image window (reconfigure handler).\n this does @em not update the contents of the window\n @since 0.3, changed in 0.4 to not redraw window"]
    pub fn zbar_window_resize(
        window: *mut zbar_window_t,
        width: c_uint,
        height: c_uint,
    ) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " select a compatible format between video input and output window.\n the selection algorithm attempts to use a format shared by\n video input and window output which is also most useful for\n barcode scanning.  if a format conversion is necessary, it will\n heuristically attempt to minimize the cost of the conversion"]
    pub fn zbar_negotiate_format(
        video: *mut zbar_video_t,
        window: *mut zbar_window_t,
    ) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zbar_image_scanner_s{
    _unused: [u8; 0]
}
#[doc = " opaque image scanner object."]
pub type zbar_image_scanner_t = zbar_image_scanner_s;
#[link(name="zbar")]
extern "C" {
    #[doc = " constructor."]
    pub fn zbar_image_scanner_create() -> *mut zbar_image_scanner_t;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " destructor."]
    pub fn zbar_image_scanner_destroy(scanner: *mut zbar_image_scanner_t);
}
#[link(name="zbar")]
extern "C" {
    #[doc = " setup result handler callback.\n the specified function will be called by the scanner whenever\n new results are available from a decoded image.\n pass a NULL value to disable callbacks.\n @returns the previously registered handler"]
    pub fn zbar_image_scanner_set_data_handler(
        scanner: *mut zbar_image_scanner_t,
        handler: zbar_image_data_handler_t,
        userdata: *const c_void,
    ) -> zbar_image_data_handler_t;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " request sending decoded codes via D-Bus\n @see zbar_processor_parse_config()\n @since 0.21"]
    pub fn zbar_image_scanner_request_dbus(
        scanner: *mut zbar_image_scanner_t,
        req_dbus_enabled: c_int,
    ) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " set config for indicated symbology (0 for all) to specified value.\n @returns 0 for success, non-0 for failure (config does not apply to\n specified symbology, or value out of range)\n @see zbar_decoder_set_config()\n @since 0.4"]
    pub fn zbar_image_scanner_set_config(
        scanner: *mut zbar_image_scanner_t,
        symbology: zbar_symbol_type_t,
        config: zbar_config_t,
        value: c_int,
    ) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " get config for indicated symbology\n @returns 0 for success, non-0 for failure (config does not apply to\n specified symbology, or value out of range). On success, *value is filled.\n @since 0.22"]
    pub fn zbar_image_scanner_get_config(
        scanner: *mut zbar_image_scanner_t,
        symbology: zbar_symbol_type_t,
        config: zbar_config_t,
        value: *mut c_int,
    ) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " enable or disable the inter-image result cache (default disabled).\n mostly useful for scanning video frames, the cache filters\n duplicate results from consecutive images, while adding some\n consistency checking and hysteresis to the results.\n this interface also clears the cache"]
    pub fn zbar_image_scanner_enable_cache(
        scanner: *mut zbar_image_scanner_t,
        enable: c_int,
    );
}
#[link(name="zbar")]
extern "C" {
    #[doc = " remove any previously decoded results from the image scanner and the\n specified image.  somewhat more efficient version of\n zbar_image_set_symbols(image, NULL) which may retain memory for\n subsequent decodes\n @since 0.10"]
    pub fn zbar_image_scanner_recycle_image(
        scanner: *mut zbar_image_scanner_t,
        image: *mut zbar_image_t,
    );
}
#[link(name="zbar")]
extern "C" {
    #[doc = " retrieve decode results for last scanned image.\n @returns the symbol set result container or NULL if no results are\n available\n @note the symbol set does not have its reference count adjusted;\n ensure that the count is incremented if the results may be kept\n after the next image is scanned\n @since 0.10"]
    pub fn zbar_image_scanner_get_results(
        scanner: *const zbar_image_scanner_t,
    ) -> *const zbar_symbol_set_t;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " scan for symbols in provided image.  The image format must be\n \"Y800\" or \"GRAY\".\n @returns >0 if symbols were successfully decoded from the image,\n 0 if no symbols were found or -1 if an error occurs\n @see zbar_image_convert()\n @since 0.9 - changed to only accept grayscale images"]
    pub fn zbar_scan_image(
        scanner: *mut zbar_image_scanner_t,
        image: *mut zbar_image_t,
    ) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zbar_decoder_s{
    _unused: [u8; 0]
}
#[doc = " opaque decoder object."]
pub type zbar_decoder_t = zbar_decoder_s;
#[doc = " decoder data handler callback function.\n called by decoder when new data has just been decoded"]
pub type zbar_decoder_handler_t =
    ::std::option::Option<unsafe extern "C" fn(decoder: *mut zbar_decoder_t)>;
#[link(name="zbar")]
extern "C" {
    #[doc = " constructor."]
    pub fn zbar_decoder_create() -> *mut zbar_decoder_t;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " destructor."]
    pub fn zbar_decoder_destroy(decoder: *mut zbar_decoder_t);
}
#[link(name="zbar")]
extern "C" {
    #[doc = " set config for indicated symbology (0 for all) to specified value.\n @returns 0 for success, non-0 for failure (config does not apply to\n specified symbology, or value out of range)\n @since 0.4"]
    pub fn zbar_decoder_set_config(
        decoder: *mut zbar_decoder_t,
        symbology: zbar_symbol_type_t,
        config: zbar_config_t,
        value: c_int,
    ) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " get config for indicated symbology\n @returns 0 for success, non-0 for failure (config does not apply to\n specified symbology, or value out of range). On success, *value is filled.\n @since 0.22"]
    pub fn zbar_decoder_get_config(
        decoder: *mut zbar_decoder_t,
        symbology: zbar_symbol_type_t,
        config: zbar_config_t,
        value: *mut c_int,
    ) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " retrieve symbology boolean config settings.\n @returns a bitmask indicating which configs are currently set for the\n specified symbology.\n @since 0.11"]
    pub fn zbar_decoder_get_configs(
        decoder: *const zbar_decoder_t,
        symbology: zbar_symbol_type_t,
    ) -> c_uint;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " clear all decoder state.\n any partial symbols are flushed"]
    pub fn zbar_decoder_reset(decoder: *mut zbar_decoder_t);
}
#[link(name="zbar")]
extern "C" {
    #[doc = " mark start of a new scan pass.\n clears any intra-symbol state and resets color to ::ZBAR_SPACE.\n any partially decoded symbol state is retained"]
    pub fn zbar_decoder_new_scan(decoder: *mut zbar_decoder_t);
}
#[link(name="zbar")]
extern "C" {
    #[doc = " process next bar/space width from input stream.\n the width is in arbitrary relative units.  first value of a scan\n is ::ZBAR_SPACE width, alternating from there.\n @returns appropriate symbol type if width completes\n decode of a symbol (data is available for retrieval)\n @returns ::ZBAR_PARTIAL as a hint if part of a symbol was decoded\n @returns ::ZBAR_NONE (0) if no new symbol data is available"]
    pub fn zbar_decode_width(
        decoder: *mut zbar_decoder_t,
        width: c_uint,
    ) -> zbar_symbol_type_t;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " retrieve color of @em next element passed to\n zbar_decode_width()."]
    pub fn zbar_decoder_get_color(decoder: *const zbar_decoder_t) -> zbar_color_t;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " retrieve last decoded data.\n @returns the data string or NULL if no new data available.\n the returned data buffer is owned by library, contents are only\n valid between non-0 return from zbar_decode_width and next library\n call"]
    pub fn zbar_decoder_get_data(decoder: *const zbar_decoder_t) -> *const c_char;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " retrieve length of binary data.\n @returns the length of the decoded data or 0 if no new data\n available."]
    pub fn zbar_decoder_get_data_length(decoder: *const zbar_decoder_t) -> c_uint;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " retrieve last decoded symbol type.\n @returns the type or ::ZBAR_NONE if no new data available"]
    pub fn zbar_decoder_get_type(decoder: *const zbar_decoder_t) -> zbar_symbol_type_t;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " retrieve modifier flags for the last decoded symbol.\n @returns a bitmask indicating which characteristics were detected\n during decoding.\n @since 0.11"]
    pub fn zbar_decoder_get_modifiers(decoder: *const zbar_decoder_t) -> c_uint;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " retrieve last decode direction.\n @returns 1 for forward and -1 for reverse\n @returns 0 if the decode direction is unknown or does not apply\n @since 0.11"]
    pub fn zbar_decoder_get_direction(decoder: *const zbar_decoder_t) -> c_int;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " setup data handler callback.\n the registered function will be called by the decoder\n just before zbar_decode_width() returns a non-zero value.\n pass a NULL value to disable callbacks.\n @returns the previously registered handler"]
    pub fn zbar_decoder_set_handler(
        decoder: *mut zbar_decoder_t,
        handler: zbar_decoder_handler_t,
    ) -> zbar_decoder_handler_t;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " associate user specified data value with the decoder."]
    pub fn zbar_decoder_set_userdata(
        decoder: *mut zbar_decoder_t,
        userdata: *mut c_void,
    );
}
#[link(name="zbar")]
extern "C" {
    #[doc = " return user specified data value associated with the decoder."]
    pub fn zbar_decoder_get_userdata(decoder: *const zbar_decoder_t)
        -> *mut c_void;
}
#[repr(C)]
pub struct zbar_scanner_s{
    _unused: [u8; 0]
}
#[doc = " opaque scanner object."]
pub type zbar_scanner_t = zbar_scanner_s;
#[link(name="zbar")]
extern "C" {
    #[doc = " constructor.\n if decoder is non-NULL it will be attached to scanner\n and called automatically at each new edge\n current color is initialized to ::ZBAR_SPACE\n (so an initial BAR->SPACE transition may be discarded)"]
    pub fn zbar_scanner_create(decoder: *mut zbar_decoder_t) -> *mut zbar_scanner_t;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " destructor."]
    pub fn zbar_scanner_destroy(scanner: *mut zbar_scanner_t);
}
#[link(name="zbar")]
extern "C" {
    #[doc = " clear all scanner state.\n also resets an associated decoder"]
    pub fn zbar_scanner_reset(scanner: *mut zbar_scanner_t) -> zbar_symbol_type_t;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " mark start of a new scan pass. resets color to ::ZBAR_SPACE.\n also updates an associated decoder.\n @returns any decode results flushed from the pipeline\n @note when not using callback handlers, the return value should\n be checked the same as zbar_scan_y()\n @note call zbar_scanner_flush() at least twice before calling this\n method to ensure no decode results are lost"]
    pub fn zbar_scanner_new_scan(scanner: *mut zbar_scanner_t) -> zbar_symbol_type_t;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " flush scanner processing pipeline.\n forces current scanner position to be a scan boundary.\n call multiple times (max 3) to completely flush decoder.\n @returns any decode/scan results flushed from the pipeline\n @note when not using callback handlers, the return value should\n be checked the same as zbar_scan_y()\n @since 0.9"]
    pub fn zbar_scanner_flush(scanner: *mut zbar_scanner_t) -> zbar_symbol_type_t;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " process next sample intensity value.\n intensity (y) is in arbitrary relative units.\n @returns result of zbar_decode_width() if a decoder is attached,\n otherwise @returns (::ZBAR_PARTIAL) when new edge is detected\n or 0 (::ZBAR_NONE) if no new edge is detected"]
    pub fn zbar_scan_y(
        scanner: *mut zbar_scanner_t,
        y: c_int,
    ) -> zbar_symbol_type_t;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " retrieve last scanned width."]
    pub fn zbar_scanner_get_width(scanner: *const zbar_scanner_t) -> c_uint;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " retrieve sample position of last edge.\n @since 0.10"]
    pub fn zbar_scanner_get_edge(
        scn: *const zbar_scanner_t,
        offset: c_uint,
        prec: c_int,
    ) -> c_uint;
}
#[link(name="zbar")]
extern "C" {
    #[doc = " retrieve last scanned color."]
    pub fn zbar_scanner_get_color(scanner: *const zbar_scanner_t) -> zbar_color_t;
}

pub mod Exception;
pub mod Decoder;
pub mod cxx_std_need;
#[cfg(test)]
mod tests;
pub use Exception::*;
pub use Decoder::*;
