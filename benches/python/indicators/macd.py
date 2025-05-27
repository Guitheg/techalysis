import numpy as np
import timeit
from . import time_as_str
import technicalysis as tx
import talib

def benchmark_macd():
    print("Benchmarking MACD...")
    iterations = 50
    data = np.random.random(1_000_000)
    
    # duration = timeit.timeit(lambda: tx.rsi(data, window_size), number=iterations)
    # average_time_rs = duration / iterations

    duration = timeit.timeit(lambda: talib.RSI(data), number=iterations)
    average_time_c = duration / iterations

    print(f"Exécution moyenne sur {iterations} itérations: (lenght: {len(data)}\n\tC:\t{time_as_str(average_time_c)}") # \n\tRust:\t{time_as_str(average_time_rs)}

    iterations = 50
    data = np.random.random(50_000)

    # duration = timeit.timeit(lambda: tx.rsi(data, window_size), number=iterations)
    # average_time_rs = duration / iterations

    duration = timeit.timeit(lambda: talib.RSI(data), number=iterations)
    average_time_c = duration / iterations
    print(f"Exécution moyenne sur {iterations} itérations: (lenght: {len(data)}\n\tC:\t{time_as_str(average_time_c)}") #\n\tRust:\t{time_as_str(average_time_rs)}