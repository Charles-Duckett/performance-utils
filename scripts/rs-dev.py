import rs_helper
import pandas as pd
import polars as pl
import time
import os
import glob

# Create a list of filenames
if not os.path.isdir(os.path.join(os.path.dirname(__file__), 'times')): os.mkdir(os.path.join(os.path.dirname(__file__), 'times'))

# glob files in fake-data folder
filenames = glob.glob(os.path.join(os.path.dirname(__file__), 'fake-data', '*.csv'))

filenames_dict = dict()
filenames_dict_pl = dict()
for filename in (filenames): filenames_dict[filename] = 'csv'

# create dict for pd read_csv
filenames_dict_pd = dict()

# time rust vs python
time_pandas = time.time()

# Read the data into Pandas DataFrames
for filename in filenames: filenames_dict_pd[filename] = pd.read_csv(filename)

stop_pandas = time.time()
pandas_time = stop_pandas - time_pandas
print('pandas time: ', pandas_time)
with open(os.path.join(os.path.dirname(__file__), 'times', 'pandas_time.txt'), 'w') as f: f.write(str(pandas_time))

# time rust vs python
time_rust_pu = time.time()

df_dict = rs_helper.read_data_from_sources(filenames_dict)

stop_rust_pu = time.time()
rust_time_pu = stop_rust_pu - time_rust_pu
print('rust_pu time: ', rust_time_pu)
with open(os.path.join(os.path.dirname(__file__), 'times', 'rust_time_pu.txt'), 'w') as f: f.write(str(rust_time_pu))



# time rust vs python
time_rust_pl = time.time()

for filename in filenames: filenames_dict_pl[filename] = pl.read_csv(filename)

stop_rust_pl = time.time()
rust_time_pl = stop_rust_pl - time_rust_pl
print('rust_pl time: ', rust_time_pl)
with open(os.path.join(os.path.dirname(__file__), 'times', 'rust_pl_time.txt'), 'w') as f: f.write(str(rust_time_pl))



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