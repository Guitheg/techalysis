from concurrent.futures import ThreadPoolExecutor, as_completed
import techalib as tb
from numpy import testing
import numpy as np
import time

def test_ema_numpy_success(csv_loader):
    df = csv_loader("ema")
    result = tb.ema(np.array(df["close"].iloc[:-1]), 30, 0.06451612903225806)
    final_result = tb.ema(np.array(df["close"]), 30, 0.06451612903225806)

    next_state = tb.ema_next(df["close"].iloc[-1], result.state)
    testing.assert_allclose(result.values, final_result.values[:-1])
    testing.assert_allclose(final_result.values, np.array(df["out"]), atol=1e-8)
    assert(next_state.ema == final_result.state.ema)

def test_ema_pandas_success(csv_loader):
    df = csv_loader("ema")
    result = tb.ema(df["close"].iloc[:-1], 30)
    final_result = tb.ema(df["close"], 30)

    next_state = tb.ema_next(df["close"].iloc[-1], result.state)
    testing.assert_allclose(result.values, final_result.values[:-1])
    testing.assert_allclose(final_result.values, df["out"], atol=1e-8)
    assert(next_state.ema == final_result.state.ema)


def test_thread_ema(thread_test):
    def ema_tx_lambda(data):
        return tb.ema(data, 30, release_gil = True)

    thread_test(ema_tx_lambda, n_threads=4)
