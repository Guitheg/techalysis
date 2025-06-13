import numpy as np
import timeit
from . import print_benchmark
import techalib as tb
import talib

def benchmark_t3():
    iterations = 50
    data = np.random.random(5_000_000)

    duration = timeit.timeit(lambda: talib.T3(data, timeperiod=50, vfactor=0.7), number=iterations)
    average_time_c = duration / iterations

    duration = timeit.timeit(lambda: tb.t3(data, period=50, vfactor=0.7), number=iterations)
    average_time_rs = duration / iterations

    print_benchmark(iterations, {"length": len(data)}, rust=average_time_rs, c=average_time_c)

    iterations = 50
    data = np.random.random(1_000_000)

    duration = timeit.timeit(lambda: talib.T3(data, timeperiod=50, vfactor=0.7), number=iterations)
    average_time_c = duration / iterations

    duration = timeit.timeit(lambda: tb.t3(data, period=50, vfactor=0.7), number=iterations)
    average_time_rs = duration / iterations

    print_benchmark(iterations, {"length": len(data)}, rust=average_time_rs, c=average_time_c)

    iterations = 50
    data = np.random.random(50_000)

    duration = timeit.timeit(lambda: talib.T3(data, timeperiod=50, vfactor=0.7), number=iterations)
    average_time_c = duration / iterations

    duration = timeit.timeit(lambda: tb.t3(data, period=50, vfactor=0.7), number=iterations)
    average_time_rs = duration / iterations

    print_benchmark(iterations, {"length": len(data)}, rust=average_time_rs, c=average_time_c)
