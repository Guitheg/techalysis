import numpy as np
import timeit
from . import print_benchmark
import techalib as tx
import talib

def benchmark_midprice():
    iterations = 50
    high = np.random.random(5_000_000)
    low = np.clip(high - 0.1, 0, 1)

    duration = timeit.timeit(lambda: talib.MIDPRICE(high, low, 14), number=iterations)
    average_time_c = duration / iterations

    duration = timeit.timeit(lambda: tx.midprice(high, low, 14), number=iterations)
    average_time_rs = duration / iterations

    print_benchmark(iterations, {"length": len(high)}, rust=average_time_rs, c=average_time_c)

    iterations = 50
    high = np.random.random(1_000_000)
    low = np.clip(high - 0.1, 0, 1)

    duration = timeit.timeit(lambda: talib.MIDPRICE(high, low, 14), number=iterations)
    average_time_c = duration / iterations

    duration = timeit.timeit(lambda: tx.midprice(high, low, 14), number=iterations)
    average_time_rs = duration / iterations

    print_benchmark(iterations, {"length": len(high)}, rust=average_time_rs, c=average_time_c)

    iterations = 50
    high = np.random.random(50_000)
    low = np.clip(high - 0.1, 0, 1)

    duration = timeit.timeit(lambda: talib.MIDPRICE(high, low, 14), number=iterations)
    average_time_c = duration / iterations

    duration = timeit.timeit(lambda: tx.midprice(high, low, 14), number=iterations)
    average_time_rs = duration / iterations

    print_benchmark(iterations, {"length": len(high)}, rust=average_time_rs, c=average_time_c)
