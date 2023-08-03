import rs_helper

# Create a list of filenames
filenames = ['fake_data-1.csv', 'fake_data-2.csv', 'fake_data-3.csv']
filenames_dict = dict()
for filename in filenames: filenames_dict[filename] = 'csv'

rs_helper.read_data_from_sources(filenames_dict)