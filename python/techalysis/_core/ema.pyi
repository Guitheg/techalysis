
from dataclasses import dataclass
from typing import NamedTuple, Optional, Tuple

from numpy.typing import NDArray

@dataclass(frozen=True)
class EmaState:
    """State for the EMA computation"""
    ema: float
    period: int
    alpha: float
    ...

class EmaResult(NamedTuple):
    """Result of the EMA computation"""
    values: NDArray
    state: EmaState

def ema(
    data: NDArray,
    period: int = 14,
    alpha: Optional[float] = None,
    release_gil: bool = False
) -> EmaResult | Tuple[NDArray, EmaState]:
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
    EmaResult
        A named tuple containing the result of the EMA computation.
        - values: **1-D array** of the same length as *data* containing the EMA.
        - state: **EmaState** (ema: float, period: int, alpha: float)
    """
    ...

def ema_next(
    new_value: float,
    state: EmaState
) -> EmaState:
    """
    Update the EMA state with the next data point.

    Parameters
    ----------
    new_value : float
        The next data point to include in the EMA calculation.
    
    state : EmaState
        The current state of the EMA computation.

    Returns
    -------
    EmaState
        The updated state after including the new value.
    """
    ...