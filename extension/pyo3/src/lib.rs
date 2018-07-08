#![feature(proc_macro, specialization)]

extern crate pyo3;
extern crate woothee;

use std::collections::HashMap;
use woothee::parser::{Parser, WootheeResult};
use pyo3::prelude::*;

#[py::modinit(fast_woothee_pyo3)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "parse")]
    pub fn parse(agent: &str) -> PyResult<HashMap<String, String>> {
        let parser = Parser::new();
        let result = parser.parse(agent);
        let r = match result {
            Some(r) => r,
            None => WootheeResult::new(),
        };
        let mut h = HashMap::new();
        h.insert("name".to_string(), r.name.to_owned());
        h.insert("category".to_string(), r.category.to_string());
        h.insert("os".to_string(), r.os.to_string());
        h.insert("os_version".to_string(), r.os_version.to_string());
        h.insert("browser_type".to_string(), r.browser_type.to_string());
        h.insert("version".to_string(), r.version.to_string());
        h.insert("vendor".to_string(), r.vendor.to_string());
        Ok(h)
    }

    #[pyfn(m, "parse2")]
    pub fn parse2(agent: &str) -> PyResult<PyObject> {
        let parser = Parser::new();
        let result = parser.parse(agent);
        let r = match result {
            Some(r) => r,
            None => WootheeResult::new(),
        };
        let gil = Python::acquire_gil();
        let py = gil.python();
        let h = PyDict::new(py);
        let _ = h.set_item(PyString::new(py, "name"), PyString::new(py, r.name.as_str()));
        let _ = h.set_item(PyString::new(py, "category"), PyString::new(py, r.category.as_str()));
        let _ = h.set_item(PyString::new(py, "os"), PyString::new(py, r.os.as_str()));
        let _ = h.set_item(PyString::new(py, "os_version"), PyString::new(py, r.os_version.as_str()));
        let _ = h.set_item(PyString::new(py, "browser_type"), PyString::new(py, r.browser_type.as_str()));
        let _ = h.set_item(PyString::new(py, "version"), PyString::new(py, r.version.as_str()));
        let _ = h.set_item(PyString::new(py, "vendor"), PyString::new(py, r.vendor.as_str()));
        Ok(h.to_object(py))
    }

    #[pyfn(m, "is_crawler")]
    pub fn is_crawler(agent: &str) -> PyResult<bool> {
        if agent.is_empty() || agent == "-" {
            return Ok(false);
        }

        let parser = Parser::new();
        let mut result = WootheeResult::new();
        Ok(parser.try_crawler(agent, &mut result))
    }

    Ok(())
}

