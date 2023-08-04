import rs_helper
import os

# Create a list of filenames
filenames = [os.path.join(os.path.dirname(__file__), filename) for filename in ['fake_data-1.csv', 'fake_data-2.csv', 'fake_data-3.csv']]
filenames_dict = dict()
for filename in filenames: filenames_dict[filename] = 'csv'

# rs_helper.read_data_from_sources(filenames_dict)
df_dict = rs_helper.read_data_from_sources(filenames_dict)
print('result:')
print(df_dict)

"""
result:
    sources:
        {'fake_data-1.csv': 'csv', 'fake_data-2.csv': 'csv', 'fake_data-3.csv': 'csv'}
    sources_iter:
        [('fake_data-1.csv', 'csv'), ('fake_data-2.csv', 'csv'), ('fake_data-3.csv', 'csv')]
"""