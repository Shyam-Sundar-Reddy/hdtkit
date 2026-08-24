//! `hdtkit._native` — PyO3 extension module.
//!
//! Real conversion functions (`ttl2hdt`, `hdt2ttl`, `hdtcat`) are added in
//! Phase 3-5 and wrapped by `hdtkit.convert` in Python.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::path::PathBuf;

mod hdt_read;
mod ttl;

/// Trivial round-trip check: `hdtkit._native.ping() == "pong"`.
#[pyfunction]
fn ping() -> PyResult<String> {
    Ok("pong".to_string())
}

/// Convert an HDT (`.hdt`) file to Turtle (`.ttl`). See `hdtkit.hdt2ttl`.
#[pyfunction]
fn hdt2ttl(input_path: PathBuf, output_path: PathBuf) -> PyResult<()> {
    hdt_read::hdt_to_ttl(&input_path, &output_path).map_err(|e| PyValueError::new_err(e.to_string()))
}

#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ping, m)?)?;
    m.add_function(wrap_pyfunction!(hdt2ttl, m)?)?;
    Ok(())
}
