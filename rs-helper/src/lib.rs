use polars::prelude::*;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pyfunction;
use pyo3_polars::PyDataFrame;
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use polars::prelude::DataFrame;

// TODO: Add support for file formats
#[derive(Debug, Clone)]
struct DataFrames<'a> {
    dataframes: HashMap<&'a str, DataFrame>,
}

impl<'a> DataFrames<'a> {
    // Constructor for DataFrames
    fn new() -> DataFrames<'a> {
        DataFrames {
            dataframes: HashMap::new(),
        }
    }

    // Add a method to insert dataframes into the HashMap
    fn insert(&mut self, key: &'a str, dataframe: DataFrame) {
        self.dataframes.insert(key, dataframe);
    }

    fn into_py(self, py: Python<'_>) -> PyObject {
        let dataframes: HashMap<&str, DataFrame> = self.dataframes;
        let py_dict: &PyDict = PyDict::new(py);
        for (key, dataframe) in dataframes {
            let py_dataframe = PyDataFrame(dataframe);
            py_dict.set_item(key, py_dataframe.into_py(py)).unwrap();
        }
        py_dict.into()
    }
}

fn parallel_dataframe_read<'a>(dictionary: HashMap<&'a str, &str>) -> Arc<Mutex<DataFrames<'a>>> {

    // let dataframes: Arc<Mutex<HashMap<& str, PolarsResult<DataFrame>>>> = Arc::new(Mutex::new(HashMap::new()));
    let dataframes: Arc<Mutex<DataFrames<'_>>> = Arc::new(Mutex::new(DataFrames::new()));

    dictionary.par_iter().for_each(|(source, format)| {
        match *format {
            "csv" => {
                let path: &Path = Path::new(*source);
                let reader: CsvReader<'_, std::fs::File> = CsvReader::from_path(path).expect("Failed to create CsvReader");
                let df: Result<DataFrame, PolarsError> = reader.has_header(true).finish();
                match df {
                    Ok(df) => {
                        dataframes.lock().unwrap().insert(source, df);
                    }
                    Err(err) => {
                        // Handle error cases here, e.g., print or log the error.
                        println!("Error reading {}: {:?}", source, err);
                    }
                }
                // dataframes.lock().unwrap().insert(source, df);
            }
            _ => {
                println!("Unsupported file format");
            }
        }
    });

    return dataframes;

}


#[pyfunction]
fn read_data_from_sources(_py: Python<'_>, sources: &PyDict) -> PyResult<PyObject> {
    
    let sources_hmap: HashMap<&str, &str> = sources.extract().unwrap();
    let dataframes_hmap: Arc<Mutex<DataFrames<'_>>> = _py.allow_threads(move || parallel_dataframe_read(sources_hmap));
    let locked_dataframes: std::sync::MutexGuard<'_, DataFrames<'_>> = dataframes_hmap.lock().unwrap();
    let free_mutex_dfs: DataFrames<'_> = locked_dataframes.clone();
    let pyobject: Py<PyAny> = free_mutex_dfs.into_py(_py);

    Ok(pyobject)

}

#[pymodule]
fn rs_helper(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_data_from_sources, m)?)?;
    Ok(())
}
