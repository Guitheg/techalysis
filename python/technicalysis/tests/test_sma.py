import technicalysis as ta
import numpy as np
import pandas as pd
import talib


def test_sma_numpy_success():
    close = np.random.random(100)

    out1 = talib.SMA(close)
    out2 = ta.sma(close, 30)
    np.testing.assert_allclose(out1, out2)

def test_sma_pandas_success():
    close = pd.Series(np.random.random(100))

    out1 = talib.SMA(close)
    out2 = ta.sma(close, 30)
    assert(type(out2) == type(close))
    np.testing.assert_allclose(out1, out2)