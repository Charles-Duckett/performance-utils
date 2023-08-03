use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pyfunction;
use std::collections::HashMap;
use polars::prelude::*;
// use polars_core::prelude::*;
// use polars_io::prelude::*;
use rayon::prelude::*;
use std::path::Path;
use std::sync::{Arc, Mutex};

// TODO: Add support for file formats

fn parallel_dataframe_read(dictionary: HashMap<&str, &str>) {
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

    println!("Hello from parallel_dataframe_read");
    println!("{:?}", dataframes);

    // use rayon to iterate over the hashmap in parallel and insert the dataframes and the names of the dataframes into the hashmap
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
