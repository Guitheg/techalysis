#!.venv/bin/python3
# -*- coding: utf-8 -*-
import pyperf

def benchmark_t3():
    runner = pyperf.Runner()
    setup = "import numpy as np; data = np.random.random(1_000_000); period = 10"
    runner.timeit("ta.T3", "ta.T3(data, period)", setup="import talib as ta;" + setup)
    runner.timeit("tx.t3", "tx.t3(data, period)", setup="import techalib as tx;" + setup)


if __name__ == "__main__":
    benchmark_t3()
