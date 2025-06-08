
from dataclasses import dataclass
from typing import List, NamedTuple, Optional, Tuple

from numpy.typing import NDArray

@dataclass(frozen=True)
class WmaState:
    """State for the Wma computation"""
    wma: float
    period: int
    period_sub: float
    period_sum: float
    window: List[float]
    ...

class WmaResult(NamedTuple):
    """Result of the Wma computation"""
    values: NDArray
    state: WmaState

def wma(
    data: NDArray,
    period: int = 14,
    release_gil: bool = False
) -> WmaResult | Tuple[NDArray, WmaState]:
    """
    WMA: Weighted Moving Average
    ----------

    Parameters
    ----------
    data : NDArray
        One dimensional array. Must satisfy ``len(data) >= period``.
    period : int
        Size of the rolling window (must be ``> 0``).

    release_gil : bool, default False
        If ``True``, the GIL is released during the computation.
        This is useful when using this function in a multi-threaded context.

    Returns
    -------
    WmaResult
        A named tuple containing the result of the Wma computation.
        - values: **NDArray** of the same length as *data* containing the WMA.
        - state: **WmaState** with (wma: float, period: int, period_sub: float, period_sum: float, window: List[float])
    """
    ...

def wma_next(
    new_value: float,
    state: WmaState
) -> WmaState:
    """
    Update the Wma state with the next data.

    Parameters
    ----------
    new_value : float
        The next data point to include in the WMA.
    
    state : WmaState
        The current state of the Wma computation.

    Returns
    -------
    WmaState
        The updated state after including the new value.
    """
    ...