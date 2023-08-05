use polars::prelude::*;
use pyo3::prelude::*;
use pyo3::types::PyDict;
// use pyo3::types::{IntoPyDict, PyDict, PyResult};
use pyo3::wrap_pyfunction;
use pyo3_polars::error::PyPolarsErr;
use pyo3_polars::PyDataFrame;
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

// TODO: Add support for file formats
// remove lifetime specifier to make it work without returing hashmap


fn parallel_dataframe_read<'a>(dictionary: HashMap<&str, &str>) {
    // fn parallel_dataframe_read(dictionary: HashMap<&str, &str>) -> PyResult<Py<PyDict>> {
    // read the data from the sources in parallel
    // return a hashmap of dataframes with their names
    let dataframes: Arc<Mutex<HashMap<& str, PolarsResult<DataFrame>>>> = Arc::new(Mutex::new(HashMap::new()));

    dictionary.par_iter().for_each(|(source, format)| {
        println!("{}, {}", source, format);
        match *format {
            "csv" => {
                let path = Path::new(*source);
                let reader = CsvReader::from_path(path).expect("Failed to create CsvReader");
                let df = reader.has_header(true).finish();
                // println!("{:?}", df);
                // insert the file into the hashmap
                dataframes.lock().unwrap().insert(source, df);
            }
            _ => {
                println!("Unsupported file format");
                // (source, Err(PolarsError::Other("Unsupported file format")))
            }
        }
    });

    // println!("{:?}", dataframes);
    // works till here for sure
    // return the hashmap of dataframes
    // return dataframes;
    // println!("{:?}", dataframes);
    // Ok(py_dict.into())

    println!("{:?}", dataframes);

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
fn read_data_from_sources(_py: Python<'_>, sources: &PyDict) -> PyResult<()> {
    println!("{:?}", sources);

    // move the data from the python dict to a rust hashmap
    let sources_hmap: HashMap<&str, &str> = sources.extract().unwrap();
    println!("{:?}", sources_hmap);

    let dataframes_hmap =
        _py.allow_threads(move || parallel_dataframe_read(sources_hmap));
    // we reacquire the GIL here because we need to return a python object

    println!("{:?}", dataframes_hmap);

    // turn the hashmap into a python dict
    // let py_dict = PyDict::new(_py);

    Ok(())
}

#[pymodule]
fn rs_helper(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_data_from_sources, m)?)?;
    Ok(())
}
