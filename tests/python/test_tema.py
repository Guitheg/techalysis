from concurrent.futures import ThreadPoolExecutor, as_completed
import techalib as tx
from numpy import testing
import numpy as np
import time

def test_tema_numpy_success(csv_loader):
    df = csv_loader("tema")
    result = tx.tema(np.array(df["close"].iloc[:-1]), 30, 0.06451612903225806)
    final_result = tx.tema(np.array(df["close"]), 30, 0.06451612903225806)

    next_state = tx.tema_next(df["close"].iloc[-1], result.state)
    testing.assert_allclose(result.values, final_result.values[:-1])
    testing.assert_allclose(final_result.values, np.array(df["out"]), atol=1e-8)
    assert(next_state.tema == final_result.state.tema)

def test_tema_pandas_success(csv_loader):
    df = csv_loader("tema")
    result = tx.tema(df["close"].iloc[:-1], 30)
    final_result = tx.tema(df["close"], 30)

    next_state = tx.tema_next(df["close"].iloc[-1], result.state)
    testing.assert_allclose(result.values, final_result.values[:-1])
    testing.assert_allclose(final_result.values, df["out"], atol=1e-8)
    assert(next_state.tema == final_result.state.tema)


def test_thread_tema(thread_test):
    def tema_tx_lambda(data):
        return tx.tema(data, 30, release_gil = True)

    thread_test(tema_tx_lambda, n_threads=4)
