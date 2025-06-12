
from dataclasses import dataclass
from typing import NamedTuple, List, Tuple

from numpy.typing import NDArray

@dataclass(frozen=True)
class TrimaState:
    """State for the Trima computation"""
    trima: float
    weighted_sum: float
    trailing_sum: float
    heading_sum: float
    last_window: List[float]
    inv_weight_sum: float
    period: int
    ...

class TrimaResult(NamedTuple):
    """Result of the Trima computation"""
    values: NDArray
    state: TrimaState

def trima(
    data: NDArray,
    period: int = 14,
    release_gil: bool = False
) -> TrimaResult | Tuple[NDArray, TrimaState]:
    """
    Trima: Triangular Moving Average
    ----------

    Parameters
    ----------
    data : NDArray
        One dimensional array of numeric observations. Must have
        ``len(data) >= period``.
    period : int, default 14
        Size of the rolling window (must be ``> 0``).

    release_gil : bool, default False
        If ``True``, the GIL is released during the computation.
        This is useful when using this function in a multi-threaded context.

    Returns
    -------
    TrimaResult
        A named tuple containing the result of the Trima computation.
        - values: **NDArray** with the computed Trima values.
        - state: **TrimaState** with (trima: float, weighted_sum: float,
          trailing_sum: float, heading_sum: float, last_window: List[float],
          inv_weight_sum: float, period: int).
    """
    ...

def trima_next(
    new_value: float,
    state: TrimaState
) -> TrimaState:
    """
    Update the Trima state with the next data.

    Parameters
    ----------
    new_value : float
        The new value to include in the Trima computation.

    state : TrimaState
        The current state of the Trima computation.

    Returns
    -------
    TrimaState
        The updated state after including the new value.
    """
    ...
