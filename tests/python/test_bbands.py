from concurrent.futures import ThreadPoolExecutor, as_completed
import techalib as tb
from numpy import testing
import numpy as np
import time

def test_bbands_numpy_success(csv_loader):
    df = csv_loader("bbands")
    result = tb.bbands(df["close"].iloc[:-1].to_numpy(), period=20, std_up=2.0, std_down=2.0)
    final_result = tb.bbands(df["close"].to_numpy(), period=20, std_up=2.0, std_down=2.0)

    next_state = tb.bbands_next(df["close"].iloc[-1], result.state)
    testing.assert_allclose(result.upper, final_result.upper[:-1])
    testing.assert_allclose(result.middle, final_result.middle[:-1])
    testing.assert_allclose(result.lower, final_result.lower[:-1])
    testing.assert_allclose(final_result.upper, np.array(df["upper"]), atol=1e-8)
    testing.assert_allclose(final_result.middle, np.array(df["middle"]), atol=1e-8)
    testing.assert_allclose(final_result.lower, np.array(df["lower"]), atol=1e-8)
    assert(next_state.upper == final_result.state.upper)
    assert(next_state.middle == final_result.state.middle)
    assert(next_state.lower == final_result.state.lower)

def test_bbands_pandas_success(csv_loader):
    df = csv_loader("bbands")
    result = tb.bbands(df["close"].iloc[:-1], period=20, std_up=2.0, std_down=2.0)
    final_result = tb.bbands(df["close"], period=20, std_up=2.0, std_down=2.0)
    next_state = tb.bbands_next(df["close"].iloc[-1], result.state)
    testing.assert_allclose(result.upper, final_result.upper[:-1])
    testing.assert_allclose(result.middle, final_result.middle[:-1])
    testing.assert_allclose(result.lower, final_result.lower[:-1])
    testing.assert_allclose(final_result.upper, df["upper"], atol=1e-8)
    testing.assert_allclose(final_result.middle, df["middle"], atol=1e-8)
    testing.assert_allclose(final_result.lower, df["lower"], atol=1e-8)
    assert(next_state.upper == final_result.state.upper)
    assert(next_state.middle == final_result.state.middle)
    assert(next_state.lower == final_result.state.lower)

def test_bbands_matype_ema_success(csv_loader):
    df = csv_loader("bbands_matype-1")
    result = tb.bbands(df["close"].iloc[:-1], period=20, std_up=2.0, std_down=2.0, ma_type=tb.BBandsMA.EMA)
    final_result = tb.bbands(df["close"], period=20, std_up=2.0, std_down=2.0, ma_type=tb.BBandsMA.EMA)
    next_state = tb.bbands_next(df["close"].iloc[-1], result.state)
    testing.assert_allclose(result.upper, final_result.upper[:-1])
    testing.assert_allclose(result.middle, final_result.middle[:-1])
    testing.assert_allclose(result.lower, final_result.lower[:-1])
    testing.assert_allclose(final_result.upper, df["upper"], atol=1e-8)
    testing.assert_allclose(final_result.middle, df["middle"], atol=1e-8)
    testing.assert_allclose(final_result.lower, df["lower"], atol=1e-8)
    assert(next_state.upper == final_result.state.upper)
    assert(next_state.middle == final_result.state.middle)
    assert(next_state.lower == final_result.state.lower)

def test_thread_bbands(thread_test):
    def bbands_tx_lambda(data):
      return tb.bbands(data, 20, 2.0, 2.0, release_gil = True)

    thread_test(bbands_tx_lambda, n_threads=4)
