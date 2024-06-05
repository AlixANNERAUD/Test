#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::ffi::{c_char, c_int, c_uint, c_ulong, c_void, CStr};

#[repr(C)]
struct request_t {
    // message id
    mid : c_uint,

    // url of the request
    url : *mut c_char,

    // action of the request, can be PUT/GET/POST/DELETE
    action : c_int,

    // payload format, currently only support attr_container_t type
    fmt : c_int,

    // payload of the request, currently only support attr_container_t type
    payload : *mut c_void,

    // length in bytes of the payload
    payload_len : c_int,

    // sender of the request
    sender : c_ulong,
}

#[link(wasm_import_module = "host")]
extern "C" {
    pub fn test(a: i32, b: i32) -> i32;
    pub fn Get_test() -> u64;

    pub fn Test_s_add(a: u64, b: i32, d: i32) -> i32;

    pub fn pack_response(response: *mut request_t, size: *mut c_int) -> *mut c_char;
}

#[export_name = "gcd"]
pub fn gcd(mut a: i32, mut b: i32) -> i32 {
    unsafe { test(1, 2) };

    let t = unsafe { Get_test() };

    println!("Get_test : {}", t);

    let mut r = request_t {
        mid: 1,
        url: std::ptr::null_mut(),
        action: 1,
        fmt: 1,
        payload: std::ptr::null_mut(),
        payload_len: 0,
        sender: 0,
    };

    let mut i = 0;

    unsafe {
        pack_response(&mut r as *mut request_t, &i as *const _ as *mut c_int);
    }

    println!("Test_s_add : {}", unsafe { Test_s_add(t, 2, 3) });

    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    println!("Arguments : {}", std::env::args().len());

    for i in std::env::args() {
        println!("{}", i);
    }

    // println!("Traire_la_vache : {}", unsafe { Traire_la_vache(1, 2) });
    // println!("Caresser_le_capitaine : {}", unsafe { Caresser_le_capitaine(1, 2) });

    println!("Hello, world from WASM !");
}
