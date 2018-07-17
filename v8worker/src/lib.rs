extern crate bytes;

pub mod binding;
pub mod handler;
pub mod worker;

extern crate libc;

use bytes::Bytes;
use std::mem;
use std::os::raw::{c_int, c_void};

pub fn new_handler() -> handler::Handler {
    let h = handler::Handler::new();
    h
}

#[test]
fn test_wrapper() {
    let mut _h = new_handler();
    let _recv_cb = move |data: Bytes| Box::new(data);
    let mut worker = worker::Worker::new(_recv_cb);
    worker.load(
        &"code.js".to_string(),
        "V8Worker2.send(new ArrayBuffer(10))".to_string(),
    );
}

#[no_mangle]
pub extern "C" fn recv(
    _buf: *mut c_void,
    _len: c_int,
    raw_cb: *mut fn(Bytes) -> Box<Bytes>,
) -> *mut binding::buf_s {
    let _contents: *mut u8;
    unsafe {
        _contents = mem::transmute(_buf);
        let slice: &[u8] = std::slice::from_raw_parts(_contents, _len as usize);
        let slice_bytes = Bytes::from(slice);
        let data = (*raw_cb)(slice_bytes);
        let data_len = data.len() as usize;

        let boxed_buf_s = Box::new(binding::buf_s {
            data: data.as_ptr() as *mut c_void,
            len: data_len,
        });
        // TODO: develop a mechanism to free the box contents:
        Box::into_raw(boxed_buf_s)
    }
}
