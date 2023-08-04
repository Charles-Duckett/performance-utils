https://github.com/pola-rs/polars/blob/30122d0a846bd22319cf1b4c38769b3a53e6ee59/py-polars/src/dataframe.rs#L772-L812

https://docs.rs/datafusion/latest/datafusion/

I also think that using pyo3 polars with datafusion might be the best way under the hood to read in parallel multiple dataframes,
and also perform the same operations on multiple datasets at the same time.

There's also object store if we want to write in parallel to the cloud.

We could also pass in queries if we'd like to use an engine written in rust at scale with multiple datasets in mind.