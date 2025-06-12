import techalib as tx
from numpy import testing
import numpy as np

def test_rsi_numpy_success(csv_loader):
    df = csv_loader("rsi")
    result = tx.rsi(np.array(df["close"].iloc[:-1]), 14)
    final_result = tx.rsi(np.array(df["close"]), 14)

    next_state = tx.rsi_next(df["close"].iloc[-1], result.state)
    testing.assert_allclose(result.values, final_result.values[:-1])
    testing.assert_allclose(final_result.values, np.array(df["out"]), atol=1e-8)
    assert(next_state.rsi == final_result.state.rsi)
    assert(next_state.avg_gain == final_result.state.avg_gain)
    assert(next_state.avg_loss == final_result.state.avg_loss)

def test_rsi_pandas_success(csv_loader):
    df = csv_loader("rsi")
    result = tx.rsi(df["close"].iloc[:-1], 14)
    final_result = tx.rsi(df["close"], 14)

    next_state = tx.rsi_next(df["close"].iloc[-1], result.state)
    testing.assert_allclose(result.values, final_result.values[:-1])
    testing.assert_allclose(final_result.values, df["out"], atol=1e-8)
    assert(next_state.rsi == final_result.state.rsi)
    assert(next_state.avg_gain == final_result.state.avg_gain)
    assert(next_state.avg_loss == final_result.state.avg_loss)

def test_thread_rsi(thread_test):
    def rsi_tx_lambda(data):
        return tx.rsi(data, 30, release_gil = True)

    thread_test(rsi_tx_lambda, n_threads=4)
