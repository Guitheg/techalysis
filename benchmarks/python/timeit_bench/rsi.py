import numpy as np
import timeit
from . import print_benchmark, time_as_str
import techalysis as tx
import talib

def py_rsi(data, window_size):
    gains = []
    losses = []
    for i in range(1, len(data)):
        delta = data[i] - data[i - 1]
        gains.append(max(delta, 0))
        losses.append(abs(min(delta, 0)))

    avg_gain = sum(gains[:window_size]) / window_size
    avg_loss = sum(losses[:window_size]) / window_size

    rsi_values = []
    for i in range(window_size, len(data) - 1):
        current_gain = gains[i]
        current_loss = losses[i]

        avg_gain = (avg_gain * (window_size - 1) + current_gain) / window_size
        avg_loss = (avg_loss * (window_size - 1) + current_loss) / window_size

        rs = avg_gain / avg_loss if avg_loss != 0 else float('inf')
        rsi = 100 - (100 / (1 + rs))
        rsi_values.append(rsi)

    return rsi_values

def benchmark_rsi():
    iterations = 50
    data = np.random.random(5_000_000)
    window_size = 50

    duration = timeit.timeit(lambda: talib.RSI(data, window_size), number=iterations)
    average_time_c = duration / iterations

    duration = timeit.timeit(lambda: tx.rsi(data, window_size, release_gil = False), number=iterations)
    average_time_rs = duration / iterations

    print_benchmark(iterations, {"length": len(data), "window size": window_size}, rust=average_time_rs, c=average_time_c)

    iterations = 50
    data = np.random.random(3_000_000)
    window_size = 50

    duration = timeit.timeit(lambda: talib.RSI(data, window_size), number=iterations)
    average_time_c = duration / iterations

    duration = timeit.timeit(lambda: tx.rsi(data, window_size), number=iterations)
    average_time_rs = duration / iterations

    print_benchmark(iterations, {"length": len(data), "window size": window_size}, rust=average_time_rs, c=average_time_c)

    iterations = 50
    data = np.random.random(50_000)
    window_size = 14

    duration = timeit.timeit(lambda: tx.rsi(data, window_size), number=iterations)
    average_time_rs = duration / iterations

    duration = timeit.timeit(lambda: talib.RSI(data, window_size), number=iterations)
    average_time_c = duration / iterations

    print_benchmark(iterations, {"length": len(data), "window size": window_size}, rust=average_time_rs, c=average_time_c)