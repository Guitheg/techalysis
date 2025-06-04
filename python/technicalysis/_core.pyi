from typing import Optional
from numpy.typing import NDArray
from typing import Tuple

def sma(
    data: NDArray,
    period: int,
    release_gil: bool = False
) -> NDArray:
    """
    SMA: Simple Moving Average
    ----------

    Parameters
    ----------
    data : 1-D array
        One dimensional array. Must satisfy
        ``len(data) >= period``.
    period : int
        Size of the rolling window (must be ``> 0``).

    release_gil : bool, default False
        If ``True``, the GIL is released during the computation.
        This is useful when using this function in a multi-threaded context.

    Returns
    -------
    1-D array
        Array of the same length as *data* containing the SMA.
    """
    ...


def ema(
    data: NDArray,
    period: int,
    alpha: Optional[float] = None,
    release_gil: bool = False
) -> NDArray:
    """
    EMA / EWMA: Exponential (Weighted) Moving Average 
    ----------

    Parameters
    ----------
    data : 1-D array
        One dimensional array of numeric observations. Must have
        ``len(data) >= period``.

    period : int
        Size of the rolling window (must be ``> 0``).

    alpha : float, default ``2.0 / (period + 1)``

    release_gil : bool, default False
        If ``True``, the GIL is released during the computation.
        This is useful when using this function in a multi-threaded context.

    Returns
    -------
    1-D array
        Array of the same length as *data* containing the EMA.

    """
    ...

def rsi(
    data: NDArray,
    period: int,
    release_gil: bool = False
) -> NDArray:
    """
    RSI: Relative Strength Index
    ----------

    Parameters
    ----------
    data : 1-D array
        One dimensional array.

    period : int
        Size of the rolling window (must be ``> 0``).

    release_gil : bool, default False
        If ``True``, the GIL is released during the computation.
        This is useful when using this function in a multi-threaded context.

    Returns
    -------
    1-D array
        Array of the same length as *data* containing the RSI.

    """
    ...

def macd(
    data: NDArray,
    fast_period: int = 12,
    slow_period: int = 26,
    signal_period: int = 9,
    release_gil: bool = False
) -> Tuple[NDArray, NDArray, NDArray]:
    """
    MACD: Moving Average Convergence Divergence
    ----------

    Parameters
    ----------
    data : 1-D array
        One dimensional array.

    fast_period : int, default 12
        Size of the fast EMA (must be ``> 0``).

    slow_period : int, default 26
        Size of the slow EMA (must be ``> 0``).

    signal_period : int, default 9
        Size of the signal EMA (must be ``> 0``).

    release_gil : bool, default False
        If ``True``, the GIL is released during the computation.
        This is useful when using this function in a multi-threaded context.

    Returns
    -------
    Tuple[1-D array, 1-D array, 1-D array]
        A tuple containing three numpy arrays of the same length as *data*:
            - MACD line
            - Signal line
            - Histogram
    """
    ...