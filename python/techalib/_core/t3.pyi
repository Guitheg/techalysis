
from dataclasses import dataclass
from typing import NamedTuple, Optional, Tuple

from numpy.typing import NDArray

@dataclass(frozen=True)
class T3State:
    """State for the T3 computation"""
    t3: float
    ema1: float
    ema2: float
    ema3: float
    ema4: float
    ema5: float
    ema6: float
    period: int
    alpha: float
    vfactor: float
    c1: float
    c2: float
    c3: float
    c4: float
    ...

class T3Result(NamedTuple):
    """Result of the T3 computation"""
    values: NDArray
    state: T3State

def t3(
    data: NDArray,
    period: int = 5,
    vfactor: float = 0.7,
    alpha: Optional[float] = None,
    release_gil: bool = False
) -> T3Result | Tuple[NDArray, T3State]:
    """
    T3: Tillson Triple Exponential Moving Average
    ----------

    Parameters
    ----------
    data : NDArray
        One dimensional array of numeric observations. Must have
        ``len(data) >= period``.
    period : int, default 5
        Size of the rolling window (must be ``> 0``).
    vfactor : float, default 0.7
        Volume factor for the T3 calculation.
    alpha : float, default ``2.0 / (period + 1)``
        Smoothing factor. If not provided, it is calculated based on the period.

    release_gil : bool, default False
        If ``True``, the GIL is released during the computation.
        This is useful when using this function in a multi-threaded context.

    Returns
    -------
    T3Result
        A named tuple containing the result of the T3 computation.
        - values: **NDArray** of the same length as *data* containing the T3 values.
        - state: **T3State** with (t3: float, ema1: float, ema2: float, ema3: float,
        ema4: float, ema5: float, ema6: float, period: int, alpha: float, vfactor: float,
        c1: float, c2: float, c3: float, c4: float)
    """
    ...

def t3_next(
    new_value: float,
    state: T3State
) -> T3State:
    """
    Update the T3 state with the next data.

    Parameters
    ----------
    new_value : float
        The next data point to include in the T3 computation.

    state : T3State
        The current state of the T3 computation.

    Returns
    -------
    T3State
        The updated state after including the new value.
    """
    ...
