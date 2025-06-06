from dataclasses import dataclass
from typing import List, NamedTuple, Tuple
from numpy.typing import NDArray


@dataclass(frozen=True)
class SmaState:
    """State for the SMA computation"""
    sma: float
    period: int
    window: List[float]
    ...

class SmaResult(NamedTuple):
    """Result of the SMA computation"""
    values: NDArray
    state: SmaState

def sma(
    data: NDArray,
    period: int,
    release_gil: bool = False
) -> SmaResult | Tuple[NDArray, SmaState]:
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
    SmaResult
        A named tuple containing the result of the SMA computation.
        - values: **1-D array** of the same length as *data* containing the SMA.
        - state: **SmaState** object containing the last SMA value and period used in the computation.

    """
    ...

def sma_next(
    new_value: float,
    state: SmaState
) -> SmaState:
    """
    Update the SMA state with the next data point.

    Parameters
    ----------
    new_value : float
        The next data point to include in the SMA.
    state : SmaState
        The current state of the SMA computation.

    Returns
    -------
    SmaState
        Updated state with the new SMA value.
    """
    ...