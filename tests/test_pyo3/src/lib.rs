use pyo3::prelude::*;

#[pyfunction]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[pyfunction]
fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => {
            let mut a: u64 = 0;
            let mut b: u64 = 1;
            for _ in 2..=n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
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
        if v == 0.0 {
            Err(pyo3::exceptions::PyZeroDivisionError::new_err("cannot divide by zero"))
        } else {
            self.value /= v;
            Ok(self.value)
        }
    }

    fn __repr__(&self) -> String {
        format!("Calculator({})", self.value)
    }
}

#[pymodule]
fn test_pyo3(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    m.add_class::<Calculator>()?;
    Ok(())
}