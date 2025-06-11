# Techalysis

**Techalysis** is a fast, reliable, and ergonomic technical analysis library written in Rust, with seamless Python bindings.

Built for developers and quants who need the performance of Rust with the ease of use of Python.

## 🚀 Features

- ⚡ **High performance**  
  Core engine written in Rust with optimized algorithms — at least as fast as TA-Lib

- 🧠 **Ergonomic API**  
  Designed for Python developers with a clean and intuitive interface and well documented.

- 🔒 **Safe and reliable**  
  Backed by a large test suite, consistency checks against TA-Lib and fuzz testing

- 🧩 **Easy integration**  
  Use seamlessly in both Python and Rust projects

- ⏱️ **Real-time updates**  
  Indicators support incremental computation via internal state objects and a next() method — ideal for streaming data or large datasets

- 🐍 **Python friendly**  
  Pythonic API with rich return types using NamedTuples, and optional GIL unlocking for true multithreaded performance

- 🖥️ **Multi-platform**  
  Supports macOS, Linux, and Windows

- 📊 Supported Indicators
  | **Category**              | **Function name - Name**                                        | **Status** |
  | ----------------          | ---------------------------------------------                   | ---------- |
  | **_Overlap_**             |||   
  |                           | **ichimoku** - Ichimoku kinkō hyō                               | ⬜        |
  |                           | **bbands** - Bollinger Bands                                    | ✅        |
  |                           | **sar** - Parabolic SAR                                         | ⬜        |
  |                           | **sarext** - Parabolic SAR, Extended                            | ⬜        |
  |                           | **ht_trend** - Hilbert Transform, Instantaneous Trendline       | ⬜        |
  |                           | **midpoint** - MidPoint over period                             | ⬜        |
  |                           | **midprice** - Midpoint Price over period                       | ⬜        |
  | _Moving Average_          | **sma** - Simple Moving Average                                 | ✅        |
  |                           | **ema** - Exponential Moving Average                            | ✅        |
  |                           | **wma** - Weighted Moving Average                               | ✅        |
  |                           | **dema** - Double Exponential Moving Average                    | ✅        |
  |                           | **tema** - Triple Exponential Moving Average                    | ✅        |
  |                           | **trima** - Triangular Moving Average                           | ✅        |
  |                           | **t3** - Tillson Moving Average                                 | ✅        |
  |                           | **kama** - Kaufman Adaptive Moving Average                      | ⬜        |
  |                           | **mama** - MESA Adaptive Moving Average                         | ⬜        |
  |                           | **mavp** - Moving Average with Variable Period                  | ⬜        |
  | **_Momentum_**            |||   
  |                           | **macd** - Moving Average Convergence Divergence                | ✅        |
  |                           | **adx** - Average Directional Movement Index                    | ⬜        |
  |                           | **adxr** - Average Directional Movement Index Rating            | ⬜        |
  |                           | **aroon** - Aroon                                               | ⬜        |
  |                           | **bop** - Balance Of Power                                      | ⬜        |
  |                           | **cci** - Commodity Channel Index                               | ⬜        |
  |                           | **dx** - Directional Movement Index                             | ⬜        |
  |                           | **minus_di** - Minus Directional Indicator                      | ⬜        |
  |                           | **minus_dm** - Minus Directional Movement                       | ⬜        |
  |                           | **plus_di** - Plus Directional Indicator                        | ⬜        |
  |                           | **plus_dm** - Plus Directional Movement                         | ⬜        |
  |                           | **roc** - Rate of change                                        | ⬜        |
  |                           | **rocp** - Rate of change Percentage                            | ⬜        |
  |                           | **rocr** - Rate of change ratio                                 | ⬜        |
  |                           | **rocr100** - Rate of change ratio 100 scale                    | ⬜        |
  |                           | **trix** - 1-day Rate-Of-Change (ROC) of a Triple Smooth EMA    | ⬜        |
  |  _Oscillator_             | **rsi** - Relative Strength Index                               | ✅        |
  |                           | **aroonosc** - Aroon Oscillator                                 | ⬜        |
  |                           | **cmo** - Chande Momentum Oscillator                            | ⬜        |
  |                           | **mfi** - Money Flow Index                                      | ⬜        |
  |                           | **ppo** - Percentage Price Oscillator                           | ⬜        |
  |                           | **stoch** - Stochastic                                          | ⬜        |
  |                           | **stochf** - Stochastic Fast                                    | ⬜        |
  |                           | **stochrsi** - Stochastic Relative Strength Index               | ⬜        |
  |                           | **ultosc** - Ultimate Oscillator                                | ⬜        |
  |                           | **willr** - Williams' %R                                        | ⬜        |
  |                           | **apo** - Absolute Price Oscillator                             | ⬜        |
  | **_Volume_**              |||   
  |                           | **ad** - Chaikin A/D Line                                       | ⬜        |
  |                           | **adosc** - Chaikin A/D Oscillator                              | ⬜        |
  |                           | **obv** - On Balance Volume                                     | ⬜        |
  | **_Cycle_**               |||   
  |                           | _Work in progress..._                                           | ⬜        |
  | **_Price Transform_**     |||   
  |                           | _Work in progress..._                                           | ⬜        |
  | **_Volatility_**          |||   
  |                           | **atr** - Average True Range                                    | ⬜        |
  |                           | **natr** - Normalized Average True Range                        | ⬜        |
  |                           | **trange** - True Range                                         | ⬜        |
  | **_Pattern Recognition_** |||   
  |                           | _Work in progress..._                                           | ⬜        |
  | **_Statistic_**           |||   
  |                           | _Work in progress..._                                           | ⬜        |

## 📦 Installation

**Available soon on PyPI and Cargo**

## 📚 Documentation

**Available soon**


## For developers

### Build with maturin

```
maturin develop --release
```

### Fuzz requirements

Install `cargo-fuzz` (more info [here](https://github.com/rust-fuzz/cargo-fuzz)):

```
cargo install cargo-fuzz
```
