# technicalysis
Technical analysis library for financial market trading applications written in rust and available in python.


### **Available soon on PyPI and Cargo**


## Features

| Function      | Description                                                                     | More                                                    | Status   |
|---------------|---------------------------------------------------------------------------------|---------------------------------------------------------|----------|
| sma           | Simple Moving Average: calculates the arithmetic mean of prices over N periods. | [Détails](https://www.investopedia.com/terms/s/sma.asp) | ✅       |
| ema           | Exponential Moving Average: is a type of moving average (MA) that places a greater weight and significance on the most recent data points. | [Détails](https://www.investopedia.com/terms/e/ema.asp) | ✅       |
| rsi           | Relative Strength Index: a momentum oscillator measuring speed and change of recent price movements. | [Détails](https://www.investopedia.com/terms/r/rsi.asp) | ⏳       |
| macd          | Moving Average Convergence Divergence: shows the relationship between two EMAs to identify momentum shifts. | [Détails](https://www.investopedia.com/terms/m/macd.asp)| ⏳       |
| bbands        | Bollinger Bands: volatility bands placed above and below a moving average, based on standard deviation. | [Détails](https://www.investopedia.com/terms/b/bollingerbands.asp)| ⏳ |
| atr           | Average True Range: measures market volatility by calculating the range of price movement. | [Détails](https://www.investopedia.com/terms/a/atr.asp) | ⏳       |
| supertrend    | SuperTrend: an indicator that identifies trend direction and provides buy/sell signals based on ATR. | [Détails](https://www.tradingview.com/support/solutions/43000634738-supertrend/)| ⏳     |
| stochastic    | Stochastic Oscillator: compares the closing price to the price range over a specified period to identify potential reversals. | [Détails](https://www.investopedia.com/terms/s/stochasticoscillator.asp)| ⏳|
| ichimoku      | Ichimoku Kinko Hyo: comprehensive indicator that identifies trends, momentum, and support/resistance levels. | [Détails](https://www.investopedia.com/terms/i/ichimoku-cloud.asp)| ⏳ |


## For developers

### Build python wheel for CPython with maturin

```
maturin develop --release --features python
```

### Fuzz requirements

Install `cargo-fuzz` (more info [here](https://github.com/rust-fuzz/cargo-fuzz)):

```
cargo install cargo-fuzz
```
