#!.venv/bin/python3
# -*- coding: utf-8 -*-
import pyperf

def benchmark_bbands():
    runner = pyperf.Runner()
    setup = "import numpy as np; data = np.random.random(1_000_000); period = 50"
    runner.timeit("tx.bbands", "tx.bbands(data, period)", setup="import techalysis as tx;" + setup)
    runner.timeit("ta.BBANDS", "ta.BBANDS(data, period)", setup="import talib as ta;" + setup)

if __name__ == "__main__":
    benchmark_bbands()
