import numpy as np
import technicalysis as ta
import talib

if __name__ == "__main__":
    close = np.random.random(100)

    out1 = talib.SMA(close)

    out2 = ta.sma(close, 30)

    print(out1, type(out1))
    print(out2, type(out2))