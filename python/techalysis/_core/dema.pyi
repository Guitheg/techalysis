
from dataclasses import dataclass
from typing import NamedTuple, Optional, Tuple

from numpy.typing import NDArray

@dataclass(frozen=True)
class DemaState:
    """State for the Dema computation"""
    dema: float
    ema_1: float
    ema_2: float
    period: int
    alpha: float
    ...

class DemaResult(NamedTuple):
    """Result of the Dema computation"""
    values: NDArray
    state: DemaState

def dema(
    data: NDArray,
    period: int = 14,
    alpha: Optional[float] = None,
    release_gil: bool = False
) -> DemaResult | Tuple[NDArray, DemaState]:
    """
    Dema: Double Exponential Moving Average
    ----------

    Parameters
    ----------
    data : NDArray
        One dimensional array of numeric observations. Must have
        ``len(data) >= period``.
    period : int, default 14
        Size of the rolling window (must be ``> 0``).
    alpha : float, default ``2.0 / (period + 1)``
        Smoothing factor. If not provided, it is calculated based on the period.

    release_gil : bool, default False
        If ``True``, the GIL is released during the computation.
        This is useful when using this function in a multi-threaded context.

    Returns
    -------
    DemaResult
        A named tuple containing the result of the Dema computation.
        - values: **NDArray** of the same length as *data* containing the Dema values.
        - state: **DemaState** with (dema: float, ema_1: float, ema_2: float, period: int, alpha: float)
    """
    ...

def dema_next(
    new_value: float,
    state: DemaState
) -> DemaState:
    """
    Update the Dema state with the next data.

    Parameters
    ----------
    new_value : float
        The next data point to include in the Dema computation.

    state : DemaState
        The current state of the Dema computation.

    Returns
    -------
    DemaState
        The updated state after including the new value.
    """
    ...
