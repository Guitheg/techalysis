import technicalysis as tx
from numpy import testing
import numpy as np

def test_macd_numpy_success(csv_loader):
    df = csv_loader("macd")
    result = tx.macd(np.array(df["close"].iloc[:-1]))
    final_result = tx.macd(np.array(df["close"]))

    next_state = tx.macd_next(df["close"].iloc[-1], result.state)
    testing.assert_allclose(result.macd, final_result.macd[:-1])
    testing.assert_allclose(result.signal, final_result.signal[:-1])
    testing.assert_allclose(result.histogram, final_result.histogram[:-1])
    testing.assert_allclose(final_result.macd, np.array(df["macd"]), atol=1e-8)
    testing.assert_allclose(final_result.signal, np.array(df["signal"]), atol=1e-8)
    testing.assert_allclose(final_result.histogram, np.array(df["histogram"]), atol=1e-8)
    assert(next_state.macd == final_result.state.macd)
    assert(next_state.signal == final_result.state.signal)
    assert(next_state.histogram == final_result.state.histogram)

def test_macd_pandas_success(csv_loader):
    df = csv_loader("macd")
    result = tx.macd(df["close"].iloc[:-1])
    final_result = tx.macd(df["close"])

    next_state = tx.macd_next(df["close"].iloc[-1], result.state)
    testing.assert_allclose(result.macd, final_result.macd[:-1])
    testing.assert_allclose(result.signal, final_result.signal[:-1])
    testing.assert_allclose(result.histogram, final_result.histogram[:-1])
    testing.assert_allclose(final_result.macd, df["macd"], atol=1e-8)
    testing.assert_allclose(final_result.signal, df["signal"], atol=1e-8)
    testing.assert_allclose(final_result.histogram, df["histogram"], atol=1e-8)
    assert(next_state.macd == final_result.state.macd)
    assert(next_state.signal == final_result.state.signal)
    assert(next_state.histogram == final_result.state.histogram)

def test_thread_macd(thread_test):
   def macd_tx_lambda(data):
      return tx.macd(data, release_gil = True)

   thread_test(macd_tx_lambda, n_threads=4)