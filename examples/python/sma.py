import numpy as np
import pandas as pd
import technicalysis as ta
import matplotlib.pyplot as plt


if __name__ == "__main__":
    close = np.random.random(100)
    plt.plot(close)

    out = ta.sma(close, 20)
    print(f"Input: {close}")
    print(f"Output1: {out}")
    plt.plot(out)

    # show
    plt.title('Simple Moving Average')
    plt.show()

