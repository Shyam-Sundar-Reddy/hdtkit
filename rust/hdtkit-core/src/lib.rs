//! `hdtkit._native` — PyO3 extension module.
//!
//! Real conversion functions (`ttl2hdt`, `hdt2ttl`, `hdtcat`) are added in
//! Phase 3-5 and wrapped by `hdtkit.convert` in Python.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::path::PathBuf;

mod hdt_merge;
mod hdt_read;
mod hdt_write;
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

/// Convert a Turtle (`.ttl`) file to HDT (`.hdt`). See `hdtkit.ttl2hdt`.
#[pyfunction]
#[pyo3(signature = (input_path, output_path, base_uri=None))]
fn ttl2hdt(input_path: PathBuf, output_path: PathBuf, base_uri: Option<String>) -> PyResult<()> {
    hdt_write::ttl_to_hdt(&input_path, &output_path, base_uri.as_deref())
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

/// Combine 2+ HDT (`.hdt`) files into one, de-duplicating triples. See `hdtkit.hdtcat`.
#[pyfunction]
fn hdtcat(input_paths: Vec<PathBuf>, output_path: PathBuf) -> PyResult<()> {
    hdt_merge::hdtcat(&input_paths, &output_path).map_err(|e| PyValueError::new_err(e.to_string()))
}

#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ping, m)?)?;
    m.add_function(wrap_pyfunction!(hdt2ttl, m)?)?;
    m.add_function(wrap_pyfunction!(ttl2hdt, m)?)?;
    m.add_function(wrap_pyfunction!(hdtcat, m)?)?;
    Ok(())
}
