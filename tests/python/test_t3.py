from concurrent.futures import ThreadPoolExecutor, as_completed
import techalib as tb
from numpy import testing
import numpy as np
import time

def test_t3_numpy_success(csv_loader):
    df = csv_loader("t3")
    result = tb.t3(np.array(df["close"].iloc[:-1]), 20)
    final_result = tb.t3(np.array(df["close"]), 20)

    next_state = tb.t3_next(df["close"].iloc[-1], result.state)
    testing.assert_allclose(result.values, final_result.values[:-1])
    testing.assert_allclose(final_result.values, np.array(df["out"]), atol=1e-8)
    assert(next_state.t3 == final_result.state.t3)

def test_t3_pandas_success(csv_loader):
    df = csv_loader("t3")
    result = tb.t3(df["close"].iloc[:-1], 20)
    final_result = tb.t3(df["close"], 20)

    next_state = tb.t3_next(df["close"].iloc[-1], result.state)
    testing.assert_allclose(result.values, final_result.values[:-1])
    testing.assert_allclose(final_result.values, df["out"], atol=1e-8)
    assert(next_state.t3 == final_result.state.t3)


def test_thread_t3(thread_test):
    def t3_tx_lambda(data):
        return tb.t3(data, 20, release_gil = True)

    thread_test(t3_tx_lambda, n_threads=4)
