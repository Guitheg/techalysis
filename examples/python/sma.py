import numpy as np
import pandas as pd
import technicalysis as ta
import talib

if __name__ == "__main__":
    close = np.random.random(100)
    close_df = pd.Series(close)
    out = ta.sma(close, 30)
    out_df = ta.sma(close_df, 30)

    print(f"Out: Numpy:{out}\nPandas:{out_df}")

