import techalib as tb
from numpy import testing
import numpy as np

def test_sma_numpy_success(csv_loader):
    df = csv_loader("sma")
    result = tb.sma(np.array(df["close"].iloc[:-1]), 30)
    final_result = tb.sma(np.array(df["close"]), 30)

    next_state = tb.sma_next(df["close"].iloc[-1], result.state)
    testing.assert_allclose(result.values, final_result.values[:-1])
    testing.assert_allclose(final_result.values, np.array(df["out"]))
    assert(next_state.sma == final_result.state.sma)
    assert(next_state.window == final_result.state.window)

def test_sma_pandas_success(csv_loader):
    df = csv_loader("sma")
    result = tb.sma(df["close"].iloc[:-1], 30)
    final_result = tb.sma(df["close"], 30)

    next_state = tb.sma_next(df["close"].iloc[-1], result.state)
    testing.assert_allclose(result.values, final_result.values[:-1])
    testing.assert_allclose(final_result.values, df["out"], atol=1e-8)
    assert(next_state.sma == final_result.state.sma)
    assert(next_state.window == final_result.state.window)

def test_thread_sma(thread_test):
    def sma_tx_lambda(data):
        return tb.sma(data, 30, release_gil = True)

    thread_test(sma_tx_lambda, n_threads=4)
