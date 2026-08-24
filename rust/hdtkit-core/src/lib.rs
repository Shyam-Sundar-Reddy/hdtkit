//! `hdtkit._native` — PyO3 extension module.
//!
//! Phase 1: skeleton only, proving the Python <-> Rust round trip. Real
//! conversion functions (`ttl2hdt`, `hdt2ttl`, `hdtcat`) are added in
//! Phase 3-5 and wrapped by `hdtkit.convert` in Python.

use pyo3::prelude::*;

/// Trivial round-trip check: `hdtkit._native.ping() == "pong"`.
#[pyfunction]
fn ping() -> PyResult<String> {
    Ok("pong".to_string())
}

#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ping, m)?)?;
    Ok(())
}
