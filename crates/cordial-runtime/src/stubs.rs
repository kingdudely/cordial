//! Runtime fallback stubs for Android/ABI symbols that have no implementation yet.
//!
//! The linker only needs an address for an unresolved symbol. Generating hundreds
//! of distinct Rust functions existed mainly to make post-call diagnostics identify
//! a stub by address. A single runtime fallback is enough for the compatibility
//! path and matches the synthetic-library approach used by Mocktail.

use std::collections::BTreeSet;
use std::ffi::c_void;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

static DECLARED: OnceLock<Mutex<BTreeSet<String>>> = OnceLock::new();
static HITS: AtomicU64 = AtomicU64::new(0);

fn declared() -> &'static Mutex<BTreeSet<String>> {
    DECLARED.get_or_init(|| Mutex::new(BTreeSet::new()))
}

pub fn declare(symbol: &str) {
    let mut guard = declared().lock().unwrap_or_else(|e| e.into_inner());
    guard.insert(symbol.to_owned());
}

fn abort_on_hit() -> bool {
    std::env::var_os("CORDIAL_STUB_ABORT").is_some()
}

fn quiet() -> bool {
    std::env::var_os("CORDIAL_STUB_QUIET").is_some()
}

pub fn address() -> *mut c_void {
    generic as *mut c_void
}

extern "C" fn generic() -> i64 {
    let first = HITS.fetch_add(1, Ordering::Relaxed) == 0;
    if first {
        crate::unimplemented::record(
            crate::unimplemented::Kind::LibcStub,
            "generic Android/ABI stub",
        );
        if !quiet() {
            eprintln!("[stub] generic Android/ABI stub hit");
        }
    }

    if abort_on_hit() {
        eprintln!("[stub] CORDIAL_STUB_ABORT set — aborting on first generic stub hit");
        report();
        std::process::abort();
    }

    0
}

pub fn report() {
    let hits = HITS.load(Ordering::Relaxed);
    if hits == 0 {
        return;
    }

    let guard = declared().lock().unwrap_or_else(|e| e.into_inner());
    eprintln!(
        "\n=== generic stubs called: {hits}; registered stub symbols: {} ===",
        guard.len()
    );
    for symbol in guard.iter() {
        eprintln!("  {symbol}");
    }
}
