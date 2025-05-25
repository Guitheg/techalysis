from numpy.typing import NDArray

def sma(
    data: NDArray,
    window_size: int,
) -> NDArray:
    """
    Simple Moving Average (SMA).

    Computes a **simple (arithmetic) moving average** over *data* using a
    fixed length sliding window.  
    The result has the **same length** as the input.  
    By convention, the first ``window_size - 1`` values are set to *NaN*
    because a complete window is not yet available.

    Parameters
    ----------
    data : numpy.ndarray[f64]
        One dimensional array. Must satisfy
        ``len(data) >= window_size``.
    window_size : int
        Size of the rolling window (must be ``> 0``).

    Returns
    -------
    numpy.ndarray[f64]
        Array of the same length as *data* containing the SMA.

    Raises
    ------
    ValueError
        If ``window_size`` is not in ``1 .. len(data)``, or if *data* contains at least one *NaN*.

    Examples
    --------
    >>> import numpy as np, technicalysis as tx
    >>> tx.sma(np.array([1., 2., 3., 4., 5.]), window_size=2)
    array([nan, 1.5, 2.5, 3.5, 4.5])
    """
    ...


def ema(
    data: NDArray,
    window_size: int,
    smoothing: float = 2.,
) -> NDArray:
    """
    Exponential (Weighted) Moving Average (EMA) / (EWMA).

    Computes an **exponential** moving average, also called an exponentially
    weighted moving average (EWMA) over *data*.  
    The smoothing factor `alpha` is derived from the conventional
    formula:

        ```
        alpha = smoothing / (window_size + 1)
        ```

    The first ``window_size - 1`` values of the result are set to *NaN*
    because the EMA is undefined until a full window is available.

    Parameters
    ----------
    data : numpy.ndarray[f64]
        One dimensional array of numeric observations. Must have
        ``len(data) >= window_size``.
    window_size : int
        Size of the rolling window (must be ``> 0``).
    smoothing : float, default ``2.0``
        Numerator used to compute the weighting factor *alpha*.
        A common choice is ``smoothing = 2.0``.

    Returns
    -------
    numpy.ndarray[f64]
        Array of the same length as *data* containing the EMA.

    Raises
    ------
    ValueError
        If ``window_size`` is not in ``1 .. len(data)`` or if *data* contains at least one *NaN*.

    Examples
    --------
    >>> import numpy as np, technicalysis as tx
    >>> tx.ema(np.array([1., 2., 3., 4., 5.]), window_size=2, smoothing=2.)
    array([nan, 1.5, 2.5, 3.5, 4.5])
    """
    ...

def rsi(
    data: NDArray,
    window_size: int,
) -> NDArray:
    """
    Relative Strength Index (RSI).

    Computes the **Relative Strength Index** (RSI) over *data* using a fixed length sliding window.
    The result has the **same length** as the input.
    By convention, the first ``window_size - 1`` values are set to *NaN* because a complete window is not yet available.

    Parameters
    ----------
    data : numpy.ndarray[f64]
        One dimensional array.
    window_size : int
        Size of the rolling window (must be ``> 0``).

    Returns
    -------
    numpy.ndarray[f64]
        Array of the same length as *data* containing the RSI.

    Raises
    ------
    ValueError
        If ``window_size`` is not in ``1 .. len(data)``, or if *data* contains at least one *NaN*.

    Examples
    --------
    >>> import numpy as np, technicalysis as tx
    >>> tx.rsi(np.array([1., 2., 3., 4., 5.]), window_size=2)
    array([nan, 1.0, 1.0, 1.0, 1.0])
    """
    ...