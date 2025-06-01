import technicalysis as tx
from numpy import testing
import numpy as np

def test_macd_numpy_success(csv_loader):
    df = csv_loader("macd")
    out = tx.macd(np.array(df["close"]), fast_period=12, slow_period=26, signal_period=9)
    testing.assert_allclose(out[0], np.array(df["macd"]), atol=1e-8)
    testing.assert_allclose(out[1], np.array(df["signal"]), atol=1e-8)
    testing.assert_allclose(out[2], np.array(df["histogram"]), atol=1e-8)


def test_macd_pandas_success(csv_loader):
    df = csv_loader("macd")
    out = tx.macd(df["close"])
    testing.assert_allclose(out[0], df["macd"], atol=1e-8)
    testing.assert_allclose(out[1], df["signal"], atol=1e-8)
    testing.assert_allclose(out[2], df["histogram"], atol=1e-8)
