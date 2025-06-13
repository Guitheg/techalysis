from concurrent.futures import ThreadPoolExecutor, as_completed
import techalib as tb
from numpy import testing
import numpy as np
import time

def test_trima_numpy_success(csv_loader):
    df = csv_loader("trima")
    result = tb.trima(np.array(df["close"].iloc[:-1]), 30)
    final_result = tb.trima(np.array(df["close"]), 30)

    next_state = tb.trima_next(df["close"].iloc[-1], result.state)
    testing.assert_allclose(result.values, final_result.values[:-1])
    testing.assert_allclose(final_result.values, np.array(df["out"]))
    assert(next_state.trima == final_result.state.trima)

def test_trima_pandas_success(csv_loader):
    df = csv_loader("trima")
    result = tb.trima(df["close"].iloc[:-1], 30)
    final_result = tb.trima(df["close"], 30)

    next_state = tb.trima_next(df["close"].iloc[-1], result.state)
    testing.assert_allclose(result.values, final_result.values[:-1])
    testing.assert_allclose(final_result.values, df["out"], atol=1e-8)
    assert(next_state.trima == final_result.state.trima)

def test_thread_trima(thread_test):
    def trima_tx_lambda(data):
        return tb.trima(data, 30, release_gil = True)

    thread_test(trima_tx_lambda, n_threads=4)
