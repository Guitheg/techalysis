from concurrent.futures import ThreadPoolExecutor, as_completed
import techalib as tb
from numpy import testing
import numpy as np
import time

def test_dema_numpy_success(csv_loader):
    df = csv_loader("dema")
    result = tb.dema(np.array(df["close"].iloc[:-1]), 30, 0.06451612903225806)
    final_result = tb.dema(np.array(df["close"]), 30, 0.06451612903225806)

    next_state = tb.dema_next(df["close"].iloc[-1], result.state)
    testing.assert_allclose(result.values, final_result.values[:-1])
    testing.assert_allclose(final_result.values, np.array(df["out"]), atol=1e-8)
    assert(next_state.dema == final_result.state.dema)

def test_dema_pandas_success(csv_loader):
    df = csv_loader("dema")
    result = tb.dema(df["close"].iloc[:-1], 30)
    final_result = tb.dema(df["close"], 30)

    next_state = tb.dema_next(df["close"].iloc[-1], result.state)
    testing.assert_allclose(result.values, final_result.values[:-1])
    testing.assert_allclose(final_result.values, df["out"], atol=1e-8)
    assert(next_state.dema == final_result.state.dema)


def test_thread_dema(thread_test):
    def dema_tx_lambda(data):
        return tb.dema(data, 30, release_gil = True)

    thread_test(dema_tx_lambda, n_threads=4)
