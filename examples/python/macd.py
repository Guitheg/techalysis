import numpy as np
import techalysis as tx
import matplotlib.pyplot as plt


if __name__ == "__main__":
    close = np.random.random(100)
    plt.plot(close)

    out = tx.macd(close, fast_period=12, slow_period=26, signal_period=9)
    print(f"Input: {close}")
    print(f"Output MACD: {out[0]}")
    print(f"Output SIGNAL: {out[1]}")
    print(f"Output HISTOGRAM: {out[2]}")
    plt.plot(out[0], label='MACD', scaley=False)
    plt.plot(out[1], label='Signal', scaley=False)
    plt.bar(range(len(out[2])), out[2], label='Histogram', color='gray', alpha=0.5)

    # show
    plt.title('Moving Average Convergence Divergence')
    plt.show()
