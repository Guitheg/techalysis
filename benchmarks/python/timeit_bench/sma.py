import numpy as np
import timeit
from . import print_benchmark, time_as_str
import techalib as tx
import talib

def py_sma(data, window_size):
    if data.size < window_size:
        return np.array([])
    window = np.ones(window_size) / window_size
    return np.convolve(data, window, mode='valid')

def benchmark_sma():
    iterations = 50
    data = np.random.random(5_000_000)
    window_size = 100

    duration = timeit.timeit(lambda: tx.sma(data, window_size), number=iterations)
    average_time_rs = duration / iterations

    duration = timeit.timeit(lambda: talib.SMA(data, window_size), number=iterations)
    average_time_c = duration / iterations

    duration = timeit.timeit(lambda: py_sma(data, window_size), number=iterations)
    average_time_py = duration / iterations
    print_benchmark(iterations, {"length": len(data), "window size": window_size}, rust=average_time_rs, c=average_time_c, python=average_time_py)

    iterations = 50
    data = np.random.random(1_000_000)
    window_size = 30

    duration = timeit.timeit(lambda: py_sma(data, window_size), number=iterations)
    average_time_py = duration / iterations

    duration = timeit.timeit(lambda: tx.sma(data, window_size), number=iterations)
    average_time_rs = duration / iterations

    duration = timeit.timeit(lambda: talib.SMA(data, window_size), number=iterations)
    average_time_c = duration / iterations
    print_benchmark(iterations, {"length": len(data), "window size": window_size}, rust=average_time_rs, c=average_time_c, python=average_time_py)

    iterations = 50
    data = np.random.random(50_000)
    window_size = 30

    duration = timeit.timeit(lambda: py_sma(data, window_size), number=iterations)
    average_time_py = duration / iterations

    duration = timeit.timeit(lambda: tx.sma(data, window_size), number=iterations)
    average_time_rs = duration / iterations

    duration = timeit.timeit(lambda: talib.SMA(data, window_size), number=iterations)
    average_time_c = duration / iterations
    print_benchmark(iterations, {"length": len(data), "window size": window_size}, rust=average_time_rs, c=average_time_c, python=average_time_py)
