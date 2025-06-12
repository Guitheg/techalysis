import numpy as np
import timeit
from . import print_benchmark
import techalysis as tx
import talib


def benchmark_ema():
    iterations = 50
    data = np.random.random(5_000_000)
    window_size = 100

    duration = timeit.timeit(lambda: tx.ema(data, window_size, 2), number=iterations)
    average_time_rs = duration / iterations

    duration = timeit.timeit(lambda: talib.EMA(data, window_size), number=iterations)
    average_time_c = duration / iterations

    print_benchmark(iterations, {"length": len(data), "window size": window_size}, rust=average_time_rs, c=average_time_c)

    iterations = 50
    data = np.random.random(3_000_000)
    window_size = 100

    duration = timeit.timeit(lambda: tx.ema(data, window_size, 2), number=iterations)
    average_time_rs = duration / iterations

    duration = timeit.timeit(lambda: talib.EMA(data, window_size), number=iterations)
    average_time_c = duration / iterations

    print_benchmark(iterations, {"length": len(data), "window size": window_size}, rust=average_time_rs, c=average_time_c)

    iterations = 50
    data = np.random.random(50_000)
    window_size = 30

    duration = timeit.timeit(lambda: tx.ema(data, window_size, 2), number=iterations)
    average_time_rs = duration / iterations

    duration = timeit.timeit(lambda: talib.EMA(data, window_size), number=iterations)
    average_time_c = duration / iterations

    print_benchmark(iterations, {"length": len(data), "window size": window_size}, rust=average_time_rs, c=average_time_c)
