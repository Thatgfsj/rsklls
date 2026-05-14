use pyo3::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
enum CalcError {
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Overflow")]
    Overflow,
}

#[pyclass]
struct Calculator {
    #[pyo3(get, set)]
    value: f64,
}

#[pymethods]
impl Calculator {
    #[new]
    fn new(initial: f64) -> Self {
        Calculator { value: initial }
    }

    fn add(&mut self, v: f64) -> f64 {
        self.value += v;
        self.value
    }

    fn subtract(&mut self, v: f64) -> f64 {
        self.value -= v;
        self.value
    }

    fn multiply(&mut self, v: f64) -> f64 {
        self.value *= v;
        self.value
    }

    fn divide(&mut self, v: f64) -> PyResult<f64> {
        if v.abs() < 1e-10 {
            Err(PyErr::new::<pyo3::exceptions::PyZeroDivisionError, _>(
                "division by zero",
            ))
        } else {
            self.value /= v;
            Ok(self.value)
        }
    }

    fn power(&mut self, exp: f64) -> f64 {
        self.value = self.value.powf(exp);
        self.value
    }

    fn sqrt(&mut self) -> PyResult<f64> {
        if self.value < 0.0 {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "cannot take sqrt of negative number",
            ))
        } else {
            self.value = self.value.sqrt();
            Ok(self.value)
        }
    }

    fn reset(&mut self) {
        self.value = 0.0;
    }

    fn __repr__(&self) -> String {
        format!("Calculator(value={})", self.value)
    }
}

#[pyfunction]
fn basic_add(a: f64, b: f64) -> f64 {
    a + b
}

#[pyfunction]
fn basic_subtract(a: f64, b: f64) -> f64 {
    a - b
}

#[pyfunction]
fn basic_multiply(a: f64, b: f64) -> f64 {
    a * b
}

#[pyfunction]
fn basic_divide(a: f64, b: f64) -> PyResult<f64> {
    if b.abs() < 1e-10 {
        Err(PyErr::new::<pyo3::exceptions::PyZeroDivisionError, _>(
            "division by zero",
        ))
    } else {
        Ok(a / b)
    }
}

#[pymodule]
fn pyo3_calc(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(basic_add, m)?)?;
    m.add_function(wrap_pyfunction!(basic_subtract, m)?)?;
    m.add_function(wrap_pyfunction!(basic_multiply, m)?)?;
    m.add_function(wrap_pyfunction!(basic_divide, m)?)?;
    m.add_class::<Calculator>()?;
    Ok(())
}