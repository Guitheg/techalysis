#!.venv/bin/python3
# -*- coding: utf-8 -*-
import pyperf

def benchmark_midprice():
    runner = pyperf.Runner()
    setup = "import numpy as np; high = np.random.random(10_000_000); low = np.clip(high - 0.1, 0, 1); period = 100"
    runner.timeit("tx.midprice", "tx.midprice(high, low, period)", setup="import techalib as tx;" + setup)
    runner.timeit("ta.midprice", "ta.MIDPRICE(high, low, period)", setup="import talib as ta;" + setup)

if __name__ == "__main__":
    benchmark_midprice()
