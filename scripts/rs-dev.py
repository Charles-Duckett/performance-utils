import rs_helper
import pandas as pd
import time
import os
import glob

# Create a list of filenames
# filenames = [os.path.join(os.path.dirname(__file__), 'fake-data', filename) for filename in ['fake_data-1.csv', 'fake_data-2.csv', 'fake_data-3.csv']]

if not os.path.isdir(os.path.join(os.path.dirname(__file__), 'times')): os.mkdir(os.path.join(os.path.dirname(__file__), 'times'))


# glob files in fake-data folder
filenames = glob.glob(os.path.join(os.path.dirname(__file__), 'fake-data', '*.csv'))

filenames_dict = dict()
for filename in filenames: filenames_dict[filename] = 'csv'

# time rust vs python
time_pandas = time.time()

# Read the data into Pandas DataFrames
for filename in filenames: pd.read_csv(filename)

stop_pandas = time.time()
pandas_time = stop_pandas - time_pandas
print('pandas time: ', pandas_time)
with open(os.path.join(os.path.dirname(__file__), 'times', 'pandas_time.txt'), 'w') as f: f.write(str(pandas_time))

# time rust vs python
time_rust = time.time()

df_dict = rs_helper.read_data_from_sources(filenames_dict)

stop_rust = time.time()
rust_time = stop_rust - time_rust
print('rust time: ', rust_time)
with open(os.path.join(os.path.dirname(__file__), 'times', 'rust_time.txt'), 'w') as f: f.write(str(rust_time))

"""
result:
    sources:
        {'fake_data-1.csv': 'csv', 'fake_data-2.csv': 'csv', 'fake_data-3.csv': 'csv'}
    sources_iter:
        [('fake_data-1.csv', 'csv'), ('fake_data-2.csv', 'csv'), ('fake_data-3.csv', 'csv')]
    results:
        {
            "fake_data-1.csv": {
                df `pl.dataframe`
            },
            ...
        }
"""