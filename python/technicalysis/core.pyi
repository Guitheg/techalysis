from numpy.typing import NDArray

def sma(
    data: NDArray, window_size: int, handle_nan: bool = False
) -> NDArray:
    """
    Compute the Simple Moving Average (SMA) of a given data array.

    Parameters
    ----------
    data : numpy.ndarray[f64]
        Input array of data points.
    window_size : int
        Window size for computing the moving average.
    handle_nan : bool, optional (default: False)
        If True, handle NaN values gracefully; if False, raise an error on NaN values.

    Returns
    -------
    numpy.ndarray[f64]
        Array containing the computed SMA.

    Raises
    ------
    ValueError
        If an error occurs during computation.

    Examples
    --------
    >>> import numpy as np
    >>> sma(np.array([1, 2, 3, 4, 5]), 2)
    array([nan, 1.5, 2.5, 3.5, 4.5])
    """
    ...