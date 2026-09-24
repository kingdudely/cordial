//! Minimal fallback for guest symbols that Cordial does not implement yet.

use std::ffi::c_void;
use std::sync::atomic::{AtomicUsize, Ordering};

static HITS: AtomicUsize = AtomicUsize::new(0);

extern "C" fn generic_stub() -> i64 {
    HITS.fetch_add(1, Ordering::Relaxed);
    0
}

pub fn stub_ptr() -> *mut c_void {
    generic_stub as *const () as *mut c_void
}

pub fn hit() -> i64 {
    generic_stub()
}

pub fn report() {
    eprintln!("[stubs] {} generic stub call(s)", HITS.load(Ordering::Relaxed));
}
