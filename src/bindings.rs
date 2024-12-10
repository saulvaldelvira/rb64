use core::ffi::c_size_t;
use core::slice;
use std::ffi::c_ulonglong;
use std::os::raw::c_ulonglong;
use std::ptr;
use std::{ffi::{c_char, CStr}, mem};

use crate::{decode, encode};

fn ptr_2_box<T>(ptr: *mut T, len: usize) -> Box<[T]> {
    unsafe {
        let elems = slice::from_raw_parts_mut(ptr, len);
        Box::from_raw(elems)
    }
}

fn box_2_ptr<T>(mut data: Box<[T]>) -> *mut T {
    let elems = data.as_mut_ptr();
    mem::forget(data);
    elems
}

#[no_mangle]
pub extern "C"
fn base64_decode(ptr: *const c_char, len: *mut c_ulonglong) -> *mut u8 {
    let cstr = unsafe { CStr::from_ptr(ptr) };
    if let Ok(s) = cstr.to_str() {
        if let Ok(d) = decode(s) {
            unsafe { *len = d.len() as c_ulonglong };
            return box_2_ptr(d)
        }
    }
    ptr::null_mut()
}

#[no_mangle]
pub extern "C"
fn base64_buffer_free(res: Buffer) {
    let Buffer { elems, elems_size } = res;
    if elems.is_null() { return }

    let vec = ptr_2_box(elems, elems_size);
    mem::drop( vec );
}

#[no_mangle]
pub extern "C"
fn base64_encode(ptr: *mut u8, len: usize) -> Buffer {
    let vec = ptr_2_box(ptr, len);
    let mut result = encode(&vec);
    result.push('\0');

    let mut result = result.into_boxed_str();
    let buf = result.as_mut_ptr();
    mem::forget(result);

    EncodeResult {
        buf,
        len
   }
}

