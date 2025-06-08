from concurrent.futures import ThreadPoolExecutor, as_completed
import techalysis as tx
from numpy import testing
import numpy as np
import time

def test_wma_numpy_success(csv_loader):
    df = csv_loader("wma")
    result = tx.wma(np.array(df["close"].iloc[:-1]), 30)
    final_result = tx.wma(np.array(df["close"]), 30)

    next_state = tx.wma_next(df["close"].iloc[-1], result.state)
    testing.assert_allclose(result.values, final_result.values[:-1])
    testing.assert_allclose(final_result.values, np.array(df["out"]))
    assert(next_state.wma == final_result.state.wma)
    assert(next_state.window == final_result.state.window)

def test_wma_pandas_success(csv_loader):
    df = csv_loader("wma")
    result = tx.wma(df["close"].iloc[:-1], 30)
    final_result = tx.wma(df["close"], 30)

    next_state = tx.wma_next(df["close"].iloc[-1], result.state)
    testing.assert_allclose(result.values, final_result.values[:-1])
    testing.assert_allclose(final_result.values, df["out"], atol=1e-8)
    assert(next_state.wma == final_result.state.wma)
    assert(next_state.window == final_result.state.window)

def test_thread_wma(thread_test):
    def wma_tx_lambda(data):
        return tx.wma(data, 30, release_gil = True)

    thread_test(wma_tx_lambda, n_threads=4)
