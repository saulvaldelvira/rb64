use core::slice;
use std::{ffi::{c_char, CStr}, mem};

use crate::{decode, encode};

#[repr(C)]
pub enum DecodeResult {
    Ok {
        elems: *mut u8,
        elems_size: usize,
    },
    Error
}

fn ptr_2_vec<T>(ptr: *mut T, len: usize) -> Vec<T> {
    let elems = unsafe {
        let elems = slice::from_raw_parts_mut(ptr, len);
        Box::from_raw(elems)
    };
    elems.into_vec()
}

fn vec_2_ptr<T>(vec: Vec<T>) -> *mut T {
    let mut data = vec.into_boxed_slice();
    let elems = data.as_mut_ptr();
    mem::forget(data);
    elems
}

#[no_mangle]
pub extern "C"
fn base64_decode(ptr: *const c_char) -> DecodeResult {
    let cstr = unsafe { CStr::from_ptr(ptr) };
    if let Ok(s) = cstr.to_str() {
        if let Ok(d) = decode(s) {
            let len = d.len();
            return DecodeResult::Ok { elems: vec_2_ptr(d), elems_size: len };
        }
    }
    DecodeResult::Error
}

#[no_mangle]
pub extern "C"
fn base64_decode_result_free(res: DecodeResult) {
    if let DecodeResult::Ok { elems, elems_size } = res {
        let vec = ptr_2_vec(elems, elems_size);
        mem::drop( vec );
    }
}

#[repr(C)]
pub struct EncodeResult {
    buf: *mut u8,
    len: usize,
    cap: usize,
}

#[no_mangle]
pub extern "C"
fn base64_encode(ptr: *mut u8, len: usize) -> EncodeResult {
    let vec = ptr_2_vec(ptr, len);
    let mut result = encode(&vec);
    result.push('\0');
    result.shrink_to_fit();
    let len = result.len();
    let cap = result.capacity();
    let cstr = result.as_mut_ptr();
    EncodeResult {
        buf: cstr,
        len, cap
   }
}

#[no_mangle]
pub extern "C"
fn base64_encode_result_free(res: EncodeResult) {
    let s = unsafe {
        String::from_raw_parts(res.buf, res.len, res.cap)
    };
    mem::drop( s );
}
