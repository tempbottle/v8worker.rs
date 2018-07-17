#![allow(improper_ctypes)]
extern crate bytes;
extern crate libc;

use libc::size_t;
use std::os::raw::{c_char, c_int, c_void};

use bytes::Bytes;

#[repr(C)]
pub struct worker {
    _unused: [u8; 0],
}

use worker::Worker as wrk;

#[link(name = "binding")]
extern "C" {
    pub fn v8_init();
    pub fn get_version() -> *const c_char;
    pub fn worker_new() -> *mut worker;
    pub fn worker_set_rust_callback(w: *mut worker, p: *mut fn(Bytes) -> Box<Bytes>);
    pub fn worker_set_rust_object(w: *mut worker, p: *mut wrk);
    pub fn worker_load(w: *mut worker, name_s: *const c_char, source_s: *const c_char) -> c_int;
    pub fn worker_send_bytes(w: *mut worker, data: *mut c_void, len: size_t) -> c_int;
    pub fn worker_dispose(w: *mut worker);
    pub fn worker_terminate_execution(w: *mut worker);

    pub fn worker_get_last_exception(w: *mut worker) -> *const c_char;

    pub fn rs2js_worker_run_task(w: *mut worker, data: *mut c_void, len: size_t);
}

#[repr(C)]
#[derive(Debug, Copy)]
pub struct buf_s {
    pub data: *mut ::std::os::raw::c_void,
    pub len: size_t,
}

impl Clone for buf_s {
    fn clone(&self) -> Self {
        *self
    }
}
