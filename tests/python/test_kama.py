import techalib as tx
from numpy import testing
import numpy as np

def test_kama_numpy_success(csv_loader):
    df = csv_loader("kama")
    result = tx.kama(np.array(df["close"].iloc[:-1]), 30)
    final_result = tx.kama(np.array(df["close"]), 30)

    next_state = tx.kama_next(df["close"].iloc[-1], result.state)
    testing.assert_allclose(result.values, final_result.values[:-1])
    testing.assert_allclose(final_result.values, np.array(df["out"]))
    assert(next_state.kama == final_result.state.kama)
    assert(next_state.last_window == final_result.state.last_window)

def test_kama_pandas_success(csv_loader):
    df = csv_loader("kama")
    result = tx.kama(df["close"].iloc[:-1], 30)
    final_result = tx.kama(df["close"], 30)

    next_state = tx.kama_next(df["close"].iloc[-1], result.state)
    testing.assert_allclose(result.values, final_result.values[:-1])
    testing.assert_allclose(final_result.values, df["out"], atol=1e-8)
    assert(next_state.kama == final_result.state.kama)
    assert(next_state.last_window == final_result.state.last_window)

def test_thread_kama(thread_test):
    def kama_tx_lambda(data):
        return tx.kama(data, 30, release_gil = True)

    thread_test(kama_tx_lambda, n_threads=4)
