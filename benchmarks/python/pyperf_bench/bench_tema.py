#!.venv/bin/python3
# -*- coding: utf-8 -*-
import pyperf

def benchmark_tema():
    runner = pyperf.Runner()
    setup = "import numpy as np; data = np.random.random(10_000_000); period = 100"
    runner.timeit("tx.tema", "tx.tema(data, period)", setup="import techalysis as tx;" + setup)
    runner.timeit("ta.tema", "ta.tema(data, period)", setup="import talib as ta;" + setup)

if __name__ == "__main__":
    benchmark_tema()