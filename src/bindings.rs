use core::{
    ffi::{c_char, c_ulonglong, CStr},
    mem, ptr, slice,
};

use crate::{decode, encode, prelude::Box};

fn ptr_2_box<T>(ptr: *mut T, len: usize) -> Box<[T]> {
    unsafe {
        let elems = slice::from_raw_parts_mut(ptr, len);
        Box::from_raw(elems)
    }
}

#[repr(C)]
pub struct Buffer {
    ptr: *mut u8,
    len: usize,
}

#[no_mangle]
pub extern "C" fn base64_decode(ptr: *const c_char, len: *mut c_ulonglong) -> Buffer {
    let cstr = unsafe { CStr::from_ptr(ptr) };
    if let Ok(s) = cstr.to_str() {
        if let Ok(mut d) = decode(s) {
            unsafe { *len = d.len() as c_ulonglong };
            let len = d.len();
            let ptr = d.as_mut_ptr();
            mem::forget(d);
            return Buffer { ptr, len };
        }
    }
    Buffer {
        ptr: ptr::null_mut(),
        len: 0,
    }
}

#[no_mangle]
pub extern "C" fn base64_buffer_free(res: Buffer) {
    let Buffer { ptr, len } = res;
    if ptr.is_null() {
        return;
    }

    let vec = ptr_2_box(ptr, len);
    mem::drop(vec);
}

#[no_mangle]
pub extern "C" fn base64_encode(ptr: *mut u8, len: usize) -> Buffer {
    let vec = ptr_2_box(ptr, len);
    let mut result = encode(&vec);
    result.push('\0');

    let len = result.len();
    let mut result = result.into_boxed_str();
    let ptr = result.as_mut_ptr();
    mem::forget(result);

    Buffer { ptr, len }
}
