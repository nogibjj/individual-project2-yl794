import time
import psutil
import pandas as pd
def get_median(data_frame):
    '''
    calculate the median
    '''
    return data_frame.median()

if __name__ == "__main__":
    start = time.time()
    CSV = "grades.csv"
    df = pd.read_csv(CSV)
    print(get_median(df))
    end = time.time()
    time_spend = end - start
    memory_info = psutil.virtual_memory()
    print(f"Time Usage: {time_spend} seconds")
    print(f"Memory Usage: {memory_info.percent}%")