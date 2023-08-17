//! Utility functions.

use std::mem::size_of;
use std::slice;

/// Returns the bytes representation of the given value.
pub fn to_bytes<'a, T>(val: &'a T) -> &'a [u8] {
	unsafe { slice::from_raw_parts(val as *const _ as *const u8, size_of::<T>()) }
}

/// Reinterprets the given slice of data into the given type.
///
/// If the size of the slice doesn't correspond to the size of the type, the function returns
/// `None`.
///
/// # Safety
///
/// It is the caller's responsibility to ensure that the byte representation is valid for the type.
pub unsafe fn reinterpret<'a, T>(val: &'a [u8]) -> Option<&'a T> {
	if val.len() >= size_of::<T>() {
		Some(&*(val.as_ptr() as *const T))
	} else {
		None
	}
}
