use pyo3::prelude::*;

#[pyclass]
pub struct Point {
    #[pyo3(get, set)]
    x: u32,
    #[pyo3(get, set)]
    y: u32,
}

#[pymethods]
impl Point {
    #[new]
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    fn dist(&self, rhs: &Self) -> f32 {
        ((self.x as f32 - rhs.x as f32).powi(2) + (self.y as f32 - rhs.y as f32).powi(2)).sqrt()
    }

    fn __str__(slf: PyRef<'_, Self>) -> String {
        format!("Point {{x = {}, y = {}}}", slf.x, slf.y)
    }
}
