#!.venv/bin/python3
# -*- coding: utf-8 -*-
import pyperf

def benchmark_rsi():
    runner = pyperf.Runner()
    setup = "import numpy as np; data = np.random.random(1_000_000); window_size = 100"
    runner.timeit("tx.rsi", "tx.rsi(data, window_size)", setup="import techalysis as tx;" + setup)
    runner.timeit("ta.RSI", "ta.RSI(data, window_size)", setup="import talib as ta;" + setup)

if __name__ == "__main__":
    benchmark_rsi()
