use mbn::{
    enums::{Action, RType, Schema, Side},
    metadata::Metadata,
    records::{Mbp1Msg, OhlcvMsg, RecordHeader},
    symbols::SymbolMap,
};

use mbn::python::buffer::BufferStore;
use pyo3::{prelude::*, wrap_pyfunction, PyClass};

#[pymodule] // The name of the function must match `lib.name` in `Cargo.toml`
#[pyo3(name = "_lib")]
fn python_mbn(_py: Python<'_>, m: &Bound<PyModule>) -> PyResult<()> {
    // ensure a module was specified, otherwise it defaults to builtins
    fn checked_add_class<T: PyClass>(m: &Bound<PyModule>) -> PyResult<()> {
        assert_eq!(T::MODULE.unwrap(), "mbn");
        m.add_class::<T>()
    }

    checked_add_class::<Side>(m)?;
    checked_add_class::<Action>(m)?;
    checked_add_class::<Schema>(m)?;
    checked_add_class::<RType>(m)?;
    checked_add_class::<SymbolMap>(m)?;
    checked_add_class::<Metadata>(m)?;
    checked_add_class::<RecordHeader>(m)?;
    checked_add_class::<OhlcvMsg>(m)?;
    checked_add_class::<Mbp1Msg>(m)?;

    checked_add_class::<BufferStore>(m)?;

    Ok(())
}
