import numpy as np
import pandas as pd
import technicalysis as ta
import talib

def np_sma(data: np.ndarray, window_size: int) -> np.ndarray:
    if data.size < window_size:
        return np.array([])  # Return an empty array if window size is larger than data.
    # Create a uniform window and compute the moving average using convolution.
    window = np.ones(window_size) / window_size
    return np.convolve(data, window, mode='valid')

if __name__ == "__main__":
    close = np.random.random(100)
    close[55] = np.nan
    print(close)
    close_df = pd.Series(close)
    out = ta.sma(close, 3)
    out2 = np_sma(close, 3)
    # out_df = ta.sma(close_df, 10)
    # out = talib.SMA(close)
    # out_df = talib.SMA(close_df)

    print(f"Out: Numpy:{out}\nPandas:{out2}")

