use bytes::Bytes;
use std::ffi::{CStr, CString};
use std::marker;
use std::os::raw::c_void;
use std::ptr::NonNull;
use std::str::Utf8Error;

use std::sync::{Once, ONCE_INIT};
static START: Once = ONCE_INIT;

use binding;

// Wrapper for the V8 worker pointer, allows sending it over threads
struct WorkerPtr(NonNull<binding::worker>);
unsafe impl marker::Send for WorkerPtr {}

// Worker structure to wrap FFI calls, etc.
#[repr(C)]
pub struct Worker {
    ptr: WorkerPtr,
    cb: fn(Bytes) -> Box<Bytes>,
}

impl Worker {
    pub fn new(cb: fn(Bytes) -> Box<Bytes>) -> Worker {
        START.call_once(|| {
            unsafe {
                binding::v8_init();
            };
        });

        // Initialize a V8 worker:
        let mut _ptr: *mut binding::worker;
        _ptr = unsafe { binding::worker_new() };

        // Wrap and store the worker pointer:
        let wrapper = WorkerPtr(NonNull::new(_ptr).unwrap());
        let w = Worker {
            ptr: wrapper,
            cb: cb,
        };

        // Also set a pointer to our Rust object:
        let boxed_cb = Box::new(cb);
        unsafe { binding::worker_set_rust_callback(_ptr, Box::into_raw(boxed_cb)) };

        w
    }

    pub fn load(&mut self, script_name: &str, code: String) {
        let c_script_name = CString::new(script_name).unwrap();
        let c_code = CString::new(code).unwrap();
        unsafe {
            binding::worker_load(self.as_ptr(), c_script_name.as_ptr(), c_code.as_ptr());
        }
    }

    pub fn send_bytes(&mut self, data: Bytes) {
        unsafe {
            binding::worker_send_bytes(self.as_ptr(), data.as_ptr() as *mut c_void, data.len());
        };
    }

    pub fn last_exception(&mut self) -> Result<&str, Utf8Error> {
        unsafe {
            let v = binding::worker_get_last_exception(self.as_ptr());
            let v = CStr::from_ptr(v);
            let v = v.to_str();
            v
        }
    }

    pub fn dispose(&mut self) {
        unsafe {
            binding::worker_dispose(self.as_ptr());
        }
    }

    pub fn terminate_execution(&mut self) {
        unsafe {
            binding::worker_terminate_execution(self.as_ptr());
        }
    }

    pub fn rs2js_worker_run_task(&mut self, data: Bytes) {
        unsafe {
            binding::rs2js_worker_run_task(self.as_ptr(), data.as_ptr() as *mut c_void, data.len());
        }
    }

    pub fn as_ptr(&mut self) -> *mut binding::worker {
        unsafe { self.ptr.0.as_mut() }
    }

    pub fn get_version() -> Result<&'static str, Utf8Error> {
        unsafe {
            let v = CStr::from_ptr(binding::get_version());
            let v = v.to_str();
            v
        }
    }
}
