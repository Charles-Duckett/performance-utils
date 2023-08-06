"""
create a fake dataset for testing the rs module
"""

import pandas as pd
import numpy as np
import os
import glob
from faker import Faker
from tqdm import tqdm

# Set a seed for reproducibility (optional)
np.random.seed(42)

# Create a Faker instance
fake = Faker()

def create_fake_data(num_rows=1_000):
    # Generate fake data
    data = {
        'ID': range(1, num_rows + 1),
        'Name': [fake.name() for _ in range(num_rows)],
        'Age': np.random.randint(18, 65, size=num_rows),
        'City': [fake.city() for _ in range(num_rows)],
        'Salary': np.random.randint(30000, 100000, size=num_rows),
    }

    # Create a DataFrame from the data
    df = pd.DataFrame(data)

    # Print the DataFrame
    # print(df.head())

    # Return the DataFrame
    return df

# Save the DataFrame to disk
def write_fake_data_static_three():
    df_1 = create_fake_data(num_rows=100_000)
    df_2 = create_fake_data(num_rows=100_000)
    df_3 = create_fake_data(num_rows=100_000)

    df_1.to_csv(os.path.join(os.path.dirname(__file__), 'fake-data', 'fake_data-1.csv'), index=False)
    df_2.to_csv(os.path.join(os.path.dirname(__file__), 'fake-data', 'fake_data-2.csv'), index=False)
    df_3.to_csv(os.path.join(os.path.dirname(__file__), 'fake-data', 'fake_data-3.csv'), index=False)

# write N number of fake data files with M number of rows
def write_fake_data(N=None, M=None):
    if N is None: return
    for i in tqdm(range(N)):
        df = create_fake_data(M)
        df.to_csv(os.path.join(os.path.dirname(__file__), 'fake-data', f'fake_data-{i}.csv'), index=False)

# check if fake data folder exists for good editor hygiene
if not os.path.isdir(os.path.join(os.path.dirname(__file__), 'fake-data')): os.mkdir(os.path.join(os.path.dirname(__file__), 'fake-data'))

# clear files in fake-data folder
for filename in glob.glob(os.path.join(os.path.dirname(__file__), 'fake-data', '*.csv')): os.remove(filename)

# write_fake_data_static_three()
write_fake_data(N=1000, M=10)

if __name__ == "__main__": pass