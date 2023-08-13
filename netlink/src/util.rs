//! Utility functions.

use std::mem::size_of;
use std::slice;

/// Returns the bytes representation of the given value.
pub fn to_bytes<'a, T>(val: &'a T) -> &'a [u8] {
	unsafe { slice::from_raw_parts(val as *const _ as *const u8, size_of::<T>()) }
}
