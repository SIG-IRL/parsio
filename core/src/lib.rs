use pyo3::prelude::*;

pub mod modules;

pub use modules::tokenizer::Tokenizer;

#[pymodule]
fn _parsio(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("VERSION", "0.1.0")?;
    m.add_class::<Tokenizer>()?;
    Ok(())
}
