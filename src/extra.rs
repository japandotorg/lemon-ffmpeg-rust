/*! extra stuff */
pub mod defs;

use std::ffi::{CString, c_void};
use std::os::raw::{c_char, c_int};
use libc::{size_t, c_float};

/*
 * #[link(name = "cbits")]
 * extern "C" {
 *    
 * }
 */