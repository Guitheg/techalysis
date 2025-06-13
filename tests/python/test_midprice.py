import techalib as tb
from numpy import testing
import numpy as np

def test_midprice_numpy_success(csv_loader):
    df = csv_loader("midprice")
    result = tb.midprice(df["high"].iloc[:-1].to_numpy(), df["low"].iloc[:-1].to_numpy(), 14)
    final_result = tb.midprice(df["high"].to_numpy(), df["low"].to_numpy(), 14)

    next_state = tb.midprice_next(df["high"].iloc[-1], df["low"].iloc[-1], result.state)
    testing.assert_allclose(result.values, final_result.values[:-1])
    testing.assert_allclose(final_result.values, np.array(df["out"]))
    assert(next_state.midprice == final_result.state.midprice)

def test_midprice_pandas_success(csv_loader):
    df = csv_loader("midprice")
    result = tb.midprice(df["high"].iloc[:-1], df["low"].iloc[:-1], 14)
    final_result = tb.midprice(df["high"], df["low"], 14)

    next_state = tb.midprice_next(df["high"].iloc[-1], df["low"].iloc[-1], result.state)
    testing.assert_allclose(result.values, final_result.values[:-1])
    testing.assert_allclose(final_result.values, df["out"], atol=1e-8)
    assert(next_state.midprice == final_result.state.midprice)

def test_thread_midprice(thread_test):
    def midprice_tx_lambda(data):
        return tb.midprice(data, data - 2.0, 14, release_gil = True)

    thread_test(midprice_tx_lambda, n_threads=4)
