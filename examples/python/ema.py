import numpy as np
import pandas as pd
import technicalysis as ta
import matplotlib.pyplot as plt
import talib


if __name__ == "__main__":
    price = np.random.random(50)
    out = talib.EMA(price, 10)
    print("[", end="")
    for i in price:
        formatted_number = "{:.6f}".format(i)
        print(f"{formatted_number}, ", end=" ")
    print("]", end="")
    print()
    print("[", end="")
    for i in out:
        formatted_number = "{:.6f}".format(i)
        print(f"{formatted_number}, ", end=" ")
    print("]", end="")

    # close = np.random.random(100)
    # plt.plot(close)

    # out = ta.sma(close, 20)
    # print(f"Input: {close}")
    # print(f"Output1: {out}")
    # plt.plot(out)

    # # show
    # plt.title('Simple Moving Average')
    # plt.show()

