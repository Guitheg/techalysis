import numpy as np
import pandas as pd

def random_walk(size: int, seed: int = 5, scale = 1.0, mean = 0.0, start_offset = 0) -> pd.DataFrame:
    """ OLHCV random walk data generator.
    Generates a random walk for open, low, high, close, and volume data.
    """
    rng = np.random.default_rng(seed=seed)
    prices = rng.normal(loc=mean, scale=scale, size=size).cumsum()
    prices -= prices.min()
    open_prices = prices + rng.uniform(-scale, scale, size=size) + start_offset
    low_prices = open_prices - rng.uniform(scale/2, scale*4, size=size)
    high_prices = open_prices + rng.uniform(scale/2, scale*4, size=size)
    close_prices = open_prices + rng.uniform(-scale, scale, size=size)
    volumes = rng.integers(low=100, high=1000, size=size)
    close_prices = np.clip(close_prices, low_prices, high_prices)
    open_prices = np.clip(open_prices, low_prices, high_prices)
    open_prices = np.maximum(open_prices, 0)
    low_prices = np.maximum(low_prices, 0)
    high_prices = np.maximum(high_prices, 0)
    close_prices = np.maximum(close_prices, 0)
    volumes = np.maximum(volumes, 0)
    return pd.DataFrame(
        {
            "open": open_prices,
            "low": low_prices,
            "high": high_prices,
            "close": close_prices,
            "volume": volumes
        }
    )
