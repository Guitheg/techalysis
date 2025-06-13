#!.venv/bin/python3
# -*- coding: utf-8 -*-
import pyperf

def benchmark_midpoint():
    runner = pyperf.Runner()
    setup = "import numpy as np; data = np.random.random(1_000_000); period = 100"
    runner.timeit("tx.midpoint", "tx.midpoint(data, period)", setup="import techalib as tx;" + setup)
    runner.timeit("ta.MIDPOINT", "ta.MIDPOINT(data, period)", setup="import talib as ta;" + setup)

if __name__ == "__main__":
    benchmark_midpoint()
