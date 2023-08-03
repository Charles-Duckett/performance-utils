"""
create a fake dataset for testing the rs module
"""

import pandas as pd
import numpy as np
from faker import Faker

# Set a seed for reproducibility (optional)
np.random.seed(42)

# Create a Faker instance
fake = Faker()

def create_fake_data():
    # Generate fake data
    num_rows = 100
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
    print(df.head())

    # Return the DataFrame
    return df

# Save the DataFrame to disk
df_1 = create_fake_data()
df_2 = create_fake_data()
df_3 = create_fake_data()

df_1.to_csv('fake_data-1.csv', index=False)
df_2.to_csv('fake_data-2.csv', index=False)
df_3.to_csv('fake_data-3.csv', index=False)

if __name__ == "__main__":
    pass