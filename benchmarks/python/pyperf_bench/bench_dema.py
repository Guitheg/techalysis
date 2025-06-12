#!.venv/bin/python3
# -*- coding: utf-8 -*-
import pyperf

def benchmark_dema():
    runner = pyperf.Runner()
    setup = "import numpy as np; data = np.random.random(10_000_000); period = 100"
    runner.timeit("tx.dema", "tx.dema(data, period)", setup="import techalysis as tx;" + setup)
    runner.timeit("ta.dema", "ta.dema(data, period)", setup="import talib as ta;" + setup)

if __name__ == "__main__":
    benchmark_dema()
