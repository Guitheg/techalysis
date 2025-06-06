
from dataclasses import dataclass
from typing import NamedTuple, Tuple
from numpy.typing import NDArray

@dataclass(frozen=True)
class MacdState:
    """State for the MACD computation"""
    fast_ema: float
    slow_ema: float
    macd: float
    signal: float
    histogram: float
    fast_period: int
    slow_period: int
    signal_period: int
    ...

class MacdResult(NamedTuple):
    """Result of the MACD computation"""
    macd: NDArray
    signal: NDArray
    histogram: NDArray
    state: MacdState

def macd(
    data: NDArray,
    fast_period: int = 12,
    slow_period: int = 26,
    signal_period: int = 9,
    release_gil: bool = False
) -> MacdResult | Tuple[NDArray, NDArray, NDArray, MacdState]:
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
    MacdResult
        A named tuple containing the result of the MACD computation.
        - macd: **1-D array** of the same length as *data* containing the MACD line values.
        - signal: **1-D array** of the same length as *data* containing the signal line values.
        - histogram: **1-D array** of the same length as *data* containing the MACD histogram values.
        - state: **MacdState** object containing the last MACD, signal, histogram, fast EMA, slow EMA, and periods used in the computation.
    """
    ...

def macd_next(
    new_value: float,
    state: MacdState
) -> MacdState:
    """
    Update the MACD state with the next value.

    Parameters
    ----------
    new_value : float
        The next value to include in the MACD calculation.

    state : MacdState
        The current state of the MACD computation.

    Returns
    -------
    MacdState
        Updated state with the new MACD, signal, histogram, fast EMA, and slow EMA values.
    """
    ...