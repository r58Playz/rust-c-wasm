use std::ffi::{c_char, CStr};

mod mallocator;

use wasm_bindgen::prelude::*;

extern "C" {
    fn cadd(x: i32, y: i32) -> i32;
    fn chello() -> *mut c_char;
    fn chello_free(ptr: *mut c_char);
}

#[wasm_bindgen]
pub fn add(left: i32, right: i32) -> i32 {
    unsafe { cadd(left, right) }
}

#[wasm_bindgen]
pub fn hello() -> String {
    let phello = unsafe { chello() };
    let c_msg = unsafe { CStr::from_ptr(phello) };
    let message = format!("{} and Rust!", c_msg.to_str().unwrap());
    unsafe { chello_free(phello) };
    message
}
