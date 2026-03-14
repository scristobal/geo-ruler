use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    let stub = ruler_py::stub_info()?;
    stub.generate()?;
    Ok(())
}
