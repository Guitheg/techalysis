#!.venv/bin/python3
# -*- coding: utf-8 -*-
import pyperf

def benchmark_macd():
    runner = pyperf.Runner()
    setup = "import numpy as np; data = np.random.random(10_000_000);"
    runner.timeit("tx.macd", "tx.macd(data, 12, 26, 9)", setup="import techalysis as tx;" + setup)
    runner.timeit("ta.MACD", "ta.MACD(data, 12, 26, 9)", setup="import talib as ta;" + setup)

if __name__ == "__main__":
    benchmark_macd()