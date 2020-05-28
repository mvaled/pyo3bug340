use pyo3::prelude::*;

#[pymodule]
fn pyo3bug(_py: Python, _m: &PyModule) -> PyResult<()> {
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use pyo3::types::{IntoPyDict, PyDateTime};

    #[test]
    fn pydelta_conversion() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let datetime = py.import("datetime").unwrap();
        let locals = [("datetime", datetime)].into_py_dict(py);
        let now: &PyDateTime = py
            .eval("datetime.datetime.utcnow()", None, Some(&locals))
            .unwrap()
            .downcast()
            .unwrap();
        println!("{:?}", now);
    }
}
