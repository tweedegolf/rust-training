use num::BigUint;
use pyo3::prelude::*;
use std::ops::Mul;

mod asynk;
mod error;
mod point;

#[pyfunction]
pub fn say_hello() {
    println!("Hello world!");
}

#[pyfunction]
pub fn fact(n: u32) -> BigUint {
    (1..n + 1)
        .into_iter()
        .map(BigUint::from)
        .reduce(Mul::mul)
        .unwrap()
}

#[pymodule]
fn hello_py(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    m.add_function(wrap_pyfunction!(fact, m)?)?;
    m.add_function(wrap_pyfunction!(error::throws_error, m)?)?;
    m.add_class::<point::Point>()?;
    m.add_function(wrap_pyfunction!(asynk::print_sleep, m)?)?;
    Ok(())
}
