use polars::prelude::*;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::collections::HashMap;

fn main() {
    source = "C:\Users\ducke\VSCodeProjects\performance-utils\scripts\fake_data.csv";
    df = DataFrame::read_csv(source);
    // print the dataframe
    println!("{:?}", df);
}
