#[macro_use]
extern crate cpython;
extern crate woothee;

use std::collections::HashMap;
use cpython::{PyResult, Python};
use woothee::parser::{Parser, WootheeResult};

py_exception!(fast_woothee, ExtParseError);

py_module_initializer!(fast_woothee_cpy, initfast_woothee_cpy, PyInit_fast_woothee_cpy, |py, m| {
    m.add(py, "is_crawler", py_fn!(py, is_crawler(agent: &str)))?;
    m.add(py, "parse", py_fn!(py, parse(agent: &str)))?;
    Ok(())
});

pub fn parse(py: Python, agent: &str) -> PyResult<HashMap<String, String>> {
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

pub fn is_crawler(_: Python, agent: &str) -> PyResult<bool> {
    if agent.is_empty() || agent == "-" {
        return Ok(false);
    }

    let parser = Parser::new();
    let mut result = WootheeResult::new();
    Ok(parser.try_crawler(agent, &mut result))
}
