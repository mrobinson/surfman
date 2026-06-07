// surfman/surfman/src/platform/windows/angle/trace.rs
//
//! EGL call tracing, enabled by setting the `SURFMAN_EGL_TRACE` environment variable.
//! Similar in spirit to `WAYLAND_DEBUG=1`.

use std::sync::OnceLock;
use std::time::Instant;

pub(super) fn enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var("SURFMAN_EGL_TRACE").is_ok())
}

pub(super) fn trace<T>(name: &str, f: impl FnOnce() -> T) -> T {
    if enabled() {
        eprintln!("[EGL trace] {name}...");
        let t = Instant::now();
        let result = f();
        eprintln!("[EGL trace] {name}: {:?}", t.elapsed());
        result
    } else {
        f()
    }
}

/// Log a single labelled point without wrapping a call — use when the call
/// mutates local variables and can't be moved into a closure.
pub(super) fn trace_point(msg: &str) {
    if enabled() {
        eprintln!("[EGL trace] {msg}");
    }
}
