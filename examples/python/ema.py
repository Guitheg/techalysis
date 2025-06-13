import numpy as np
import pandas as pd
import techalib as tb
import matplotlib.pyplot as plt


if __name__ == "__main__":
    close = np.random.random(100)
    plt.plot(close)

    out = tb.ema(close, 20)
    print(f"Input: {close}")
    print(f"Output1: {out}")
    plt.plot(out)

    # show
    plt.title('Exponential Moving Average')
    plt.show()
