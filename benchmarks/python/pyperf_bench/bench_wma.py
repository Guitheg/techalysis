#!.venv/bin/python3
# -*- coding: utf-8 -*-
import pyperf

def benchmark_wma():
    runner = pyperf.Runner()
    setup = "import numpy as np; data = np.random.random(1_000_000); period = 100"
    runner.timeit("tx.wma", "tx.wma(data, period)", setup="import techalysis as tx;" + setup)
    runner.timeit("ta.WMA", "ta.WMA(data, period)", setup="import talib as ta;" + setup)

if __name__ == "__main__":
    benchmark_wma()
