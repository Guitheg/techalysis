import numpy as np
import timeit
from . import print_benchmark
import techalib as tx
import talib

def benchmark_bbands():
    iterations = 50
    data = np.random.random(5_000_000)

    duration = timeit.timeit(lambda: talib.BBANDS(data, 20), number=iterations)
    average_time_c = duration / iterations

    duration = timeit.timeit(lambda: tx.bbands(data, 20), number=iterations)
    average_time_rs = duration / iterations

    print_benchmark(iterations, {"length": len(data)}, rust=average_time_rs, c=average_time_c)

    iterations = 50
    data = np.random.random(1_000_000)

    duration = timeit.timeit(lambda: talib.BBANDS(data, 20), number=iterations)
    average_time_c = duration / iterations

    duration = timeit.timeit(lambda: tx.bbands(data, 20), number=iterations)
    average_time_rs = duration / iterations

    print_benchmark(iterations, {"length": len(data)}, rust=average_time_rs, c=average_time_c)

    iterations = 50
    data = np.random.random(50_000)

    duration = timeit.timeit(lambda: tx.bbands(data, 20), number=iterations)
    average_time_rs = duration / iterations

    duration = timeit.timeit(lambda: talib.BBANDS(data, 20), number=iterations)
    average_time_c = duration / iterations
    print_benchmark(iterations, {"length": len(data)}, rust=average_time_rs, c=average_time_c)

    iterations = 50
    data = np.random.random(1_000_000)

    duration = timeit.timeit(lambda: talib.BBANDS(data, 20, matype=1), number=iterations)
    average_time_c = duration / iterations

    duration = timeit.timeit(lambda: tx.bbands(data, 20, ma_type=tx.BBandsMA.EMA), number=iterations)
    average_time_rs = duration / iterations

    print_benchmark(iterations, {"length": len(data)}, rust=average_time_rs, c=average_time_c)
