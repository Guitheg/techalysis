import numpy as np
import timeit
from . import time_as_str
import technicalysis as ta
import talib


def benchmark_ema():
    iterations = 50
    data = np.random.random(1_000_000)
    window_size = 100
    
    duration = timeit.timeit(lambda: ta.ema(data, window_size, 2), number=iterations)
    average_time_rs = duration / iterations

    duration = timeit.timeit(lambda: talib.EMA(data, window_size), number=iterations)
    average_time_c = duration / iterations

    # duration = timeit.timeit(lambda: py_sma(data, window_size), number=iterations)
    # average_time_py = duration / iterations
    print(f"Exécution moyenne sur {iterations} itérations: (lenght: {len(data)}, window size: {window_size})")
    # print(f"\tPython:\t{time_as_str(average_time_py)}")
    print(f"\tRust:\t{time_as_str(average_time_rs)}")
    print(f"\tC:\t{time_as_str(average_time_c)}")
