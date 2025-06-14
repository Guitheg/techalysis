import techalib as tb
from numpy import testing
import numpy as np

def test_roc_numpy_success(csv_loader):
    df = csv_loader("roc")
    result = tb.roc(np.array(df["close"].iloc[:-1]), 10)
    final_result = tb.roc(np.array(df["close"]), 10)

    next_state = tb.roc_next(df["close"].iloc[-1], result.state)
    testing.assert_allclose(result.values, final_result.values[:-1], atol=1e-7)
    testing.assert_allclose(final_result.values, np.array(df["out"]), atol=1e-7)
    assert(next_state.roc == final_result.state.roc)

def test_roc_pandas_success(csv_loader):
    df = csv_loader("roc")
    result = tb.roc(df["close"].iloc[:-1], 10)
    final_result = tb.roc(df["close"], 10)

    next_state = tb.roc_next(df["close"].iloc[-1], result.state)
    testing.assert_allclose(result.values, final_result.values[:-1], atol=1e-7)
    testing.assert_allclose(final_result.values, df["out"], atol=1e-7)
    assert(next_state.roc == final_result.state.roc)

def test_thread_roc(thread_test):
    def roc_tx_lambda(data):
        return tb.roc(data, 10, release_gil = True)

    thread_test(roc_tx_lambda, n_threads=4)
