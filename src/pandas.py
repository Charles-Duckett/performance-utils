import pandas as pd

class PandasHelper:

    @staticmethod
    def set_option_bulk():
        pd.set_option('display.max_rows', 25)
        pd.set_option('display.max_columns', None)
        pd.set_option('display.width', 1500)
        pd.set_option('display.float_format', '{:.2f}'.format)


    @staticmethod
    def read_bulk_files():
        
