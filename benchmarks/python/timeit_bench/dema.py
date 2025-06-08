import numpy as np
import timeit
from . import print_benchmark
import techalysis as tx
import talib

def benchmark_dema():
    iterations = 50
    data = np.random.random(5_000_000)

    duration = timeit.timeit(lambda: talib.DEMA(data), number=iterations)
    average_time_c = duration / iterations

    duration = timeit.timeit(lambda: tx.dema(data), number=iterations)
    average_time_rs = duration / iterations

    print_benchmark(iterations, {"length": len(data)}, rust=average_time_rs, c=average_time_c)

    iterations = 50
    data = np.random.random(1_000_000)
    
    duration = timeit.timeit(lambda: talib.DEMA(data), number=iterations)
    average_time_c = duration / iterations

    duration = timeit.timeit(lambda: tx.dema(data), number=iterations)
    average_time_rs = duration / iterations

    print_benchmark(iterations, {"length": len(data)}, rust=average_time_rs, c=average_time_c)

    iterations = 50
    data = np.random.random(50_000)

    duration = timeit.timeit(lambda: talib.DEMA(data), number=iterations)
    average_time_c = duration / iterations

    duration = timeit.timeit(lambda: tx.dema(data), number=iterations)
    average_time_rs = duration / iterations

    print_benchmark(iterations, {"length": len(data)}, rust=average_time_rs, c=average_time_c)