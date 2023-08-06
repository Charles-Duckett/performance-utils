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

# time rust vs python
time_rust_pu = time.time()

df_dict = rs_helper.read_data_from_sources(filenames_dict)

stop_rust_pu = time.time()
rust_time_pu = stop_rust_pu - time_rust_pu
print('rust_pu time: ', rust_time_pu)
with open(os.path.join(os.path.dirname(__file__), 'times', 'rust_time_pu.txt'), 'w') as f: f.write(str(rust_time_pu))


