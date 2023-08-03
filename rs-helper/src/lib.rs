use polars::prelude::*;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pyfunction;
use rayon::prelude::*;

use std::fs::File;
use std::io::BufReader;
use std::io::Cursor;
use std::path::PathBuf;

// TODO: Add support for other file formats
// TODO: .finish() the readers and then in the read_data_from_sources function, convert all the dataframes to pyobjects

// fn read_data_from_source(source: &str, format: &str) -> PyResult<Py<PyAny>> {
//     let path: PathBuf = source.into();

//     match format.to_lowercase().as_str() {
//         "csv" => {
//             let df = CsvReader::from_path(path).unwrap();
//             Ok(df.to_py(None))
//         }
//         "parquet" => {
//             let df = ParquetReader::from_path(path).unwrap();
//             Ok(df.to_py(None))
//         }
//         "json" => {
//             let mut file = std::fs::File::open(source).unwrap();
//             let _df = JsonReader::new(&mut file).finish().unwrap();
//             Ok(df.to_py(None))
//         }
//     }
// }

#[pyfunction]
// fn read_data_from_sources(sources: &PyDict) -> PyResult<Vec<Py<PyAny>>> {
fn read_data_from_sources(sources: &PyDict) -> PyResult<()> {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let sources_iter = sources.items();
    println!("{:?}", sources_iter);
    // let dataframes: Vec<Py<PyAny>> = sources_iter
    //     .into_iter()
    //     .par_map(|(source_obj, format_obj)| {
    //         let source = source_obj.extract::<String>().unwrap();
    //         let format = format_obj.extract::<String>().unwrap();
    //         read_data_from_source(&source, &format).unwrap()
    //     })
    //     .collect();

    // Ok(dataframes)

    Ok(())
}

#[pymodule]
fn rs_helper(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_data_from_sources, m)?)?;
    Ok(())
}
