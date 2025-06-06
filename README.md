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
  | **Category**     | **Name**                                        | **Status** |
  | ---------------- | ---------------------------------------------               | ---------- |
  | **_Trend_**      |                                                             |            |
  || **SMA** - Simple Moving Average                                               | ✅         |
  || **EMA** - Exponential Moving Average                                          | ✅         |
  || *Work in progress...*                                                         | 🚧         |
  | **_Momentum_**   |                                                             |            |
  || **RSI** - Relative Strength Index                                             | ✅         |
  || **MACD** - Moving Average Convergence Divergence                              | ✅         |
  || *Work in progress...*                                                         | 🚧         |
  | **_Volatility_** |                                                             |            |
  || Bollinger Bands (BBANDS)                                                      | ✅         |
  || *Work in progress...*                                                         | 🚧         |
  | **_Volume_**     |                                                             |            |
  || *Work in progress...*                                                         | 🚧         |
  | **_Oscillators_**|                                                             |            |
  || *Work in progress...*                                                         | 🚧         |
  | **_Other_**      |                                                             |            |
  || *Work in progress...*                                                         | 🚧         |

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
