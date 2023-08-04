use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pyfunction;
use std::collections::HashMap;
use polars::prelude::*;
// use polars::dataframe::PyDataFrame;
// use arrow::csv::ReaderBuilder;
// use csv::ReaderBuilder;
use csv::ReaderBuilder;
use arrow::error::ArrowError;
use std::fs::File;
use rayon::prelude::*;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::io::BufReader;
use std::io::Read;
use pyo3_polars::{
    PyDataFrame,
    PyLazyFrame,
};
use pyo3_polars::error::PyPolarsErr;

// TODO: Add support for file formats

fn parallel_dataframe_read(dictionary: HashMap<&str, &str>) {
// fn parallel_dataframe_read(dictionary: HashMap<&str, &str>) -> PyResult<Py<PyDict>> {
    // read the data from the sources in parallel
    // return a hashmap of dataframes with their names
    let dataframes: Arc<Mutex<HashMap<&str, PolarsResult<DataFrame>>>> = Arc::new(Mutex::new(HashMap::new()));

    dictionary
        .par_iter()
        .for_each(|(source, format)| {
        println!("{}, {}", source, format);
        match *format {
            "csv" => {
                let path = Path::new(*source);
                let reader = CsvReader::from_path(path).expect("Failed to create CsvReader");
                let df = reader.has_header(true).finish();
                println!("{:?}", df);
                // insert the file into the hashmap
                dataframes.lock().unwrap().insert(source, df);
            },
            _ => {
                println!("Unsupported file format");
                // (source, Err(PolarsError::Other("Unsupported file format")))
            }
        }
    });

    // println!("{:?}", dataframes);
    // Ok(py_dict.into())
}

fn parallel_arrow_read(dictionary: HashMap<&str, &str>) {
    // let input_file = File::open("my_data.csv")?;
    // let csv_reader = ReaderBuilder::new().build(input_file)?;

    // dictionary
    //     .par_iter()
    //     .for_each(|(source, format)| {
    //         println!("{}, {}", source, format);
    //         match *format {
    //             "csv" => {
    //                 // let path = Path::new(*source);
    //                 let input_file = File::open(*source);
    //                 // let csv_reader = ReaderBuilder::new().build(input_file).expect("Failed to create CsvReader");
    //                 let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(input_file);
    //             },
    //             _ => {
    //                 println!("Unsupported file format");
    //                 // (source, Err(PolarsError::Other("Unsupported file format")))
    //             }
    //         }
    //     });
}




#[pyfunction]
// fn read_data_from_sources(sources: &PyDict) -> PyResult<Vec<Py<PyAny>>> {
fn read_data_from_sources(sources: &PyDict) -> PyResult<()> {
    let gil = Python::acquire_gil();
    let _py = gil.python();

    println!("{:?}", sources);

    // move the data from the python dict to a rust hashmap
    let mut sources_hmap: HashMap<&str, &str> = sources.extract().unwrap();
    println!("{:?}", sources_hmap);
    
    let dataframes_hmap = _py.allow_threads(move || parallel_dataframe_read(sources_hmap));
    // we reacquire the GIL here because we need to return a python object

    Ok(())
}

#[pymodule]
fn rs_helper(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_data_from_sources, m)?)?;
    Ok(())
}
