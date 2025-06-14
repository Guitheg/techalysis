#!.venv/bin/python3
# -*- coding: utf-8 -*-
import pyperf

def benchmark_roc():
    runner = pyperf.Runner()
    setup = "import numpy as np; data = np.random.random(10_000_000); period = 100"
    runner.timeit("tx.roc", "tx.roc(data, period)", setup="import techalib as tx;" + setup)
    runner.timeit("ta.roc", "ta.ROC(data, period)", setup="import talib as ta;" + setup)

if __name__ == "__main__":
    benchmark_roc()
