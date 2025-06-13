import techalib as tb
from numpy import testing
import numpy as np

def test_midpoint_numpy_success(csv_loader):
    df = csv_loader("midpoint")
    result = tb.midpoint(np.array(df["close"].iloc[:-1]), 14)
    final_result = tb.midpoint(np.array(df["close"]), 14)

    next_state = tb.midpoint_next(df["close"].iloc[-1], result.state)
    testing.assert_allclose(result.values, final_result.values[:-1])
    testing.assert_allclose(final_result.values, np.array(df["out"]))
    assert(next_state.midpoint == final_result.state.midpoint)
    assert(next_state.last_window == final_result.state.last_window)

def test_midpoint_pandas_success(csv_loader):
    df = csv_loader("midpoint")
    result = tb.midpoint(df["close"].iloc[:-1], 14)
    final_result = tb.midpoint(df["close"], 14)

    next_state = tb.midpoint_next(df["close"].iloc[-1], result.state)
    testing.assert_allclose(result.values, final_result.values[:-1])
    testing.assert_allclose(final_result.values, df["out"], atol=1e-8)
    assert(next_state.midpoint == final_result.state.midpoint)
    assert(next_state.last_window == final_result.state.last_window)

def test_thread_midpoint(thread_test):
    def midpoint_tx_lambda(data):
        return tb.midpoint(data, 14, release_gil = True)

    thread_test(midpoint_tx_lambda, n_threads=4)
