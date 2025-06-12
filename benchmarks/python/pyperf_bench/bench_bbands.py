#!.venv/bin/python3
# -*- coding: utf-8 -*-
import pyperf

def benchmark_bbands():
    runner = pyperf.Runner()
    setup = "import numpy as np; data = np.random.random(10_000_000); period = 100"
    runner.timeit("tx.bbands", "tx.bbands(data, period)", setup="import techalysis as tx;" + setup)
    runner.timeit("ta.bbands", "ta.bbands(data, period)", setup="import talib as ta;" + setup)

if __name__ == "__main__":
    benchmark_bbands()
