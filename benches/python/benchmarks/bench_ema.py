#!.venv/bin/python3
# -*- coding: utf-8 -*-
import pyperf

def benchmark_ema():
    runner = pyperf.Runner()
    setup = "import numpy as np; data = np.random.random(10_000_000); window_size = 100"
    runner.timeit("tx.ema", "tx.ema(data, window_size)", setup="import technicalysis as tx;" + setup)
    runner.timeit("ta.EMA", "ta.EMA(data, window_size)", setup="import talib as ta;" + setup)

if __name__ == "__main__":
    benchmark_ema()