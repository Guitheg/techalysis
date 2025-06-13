
from dataclasses import dataclass
from typing import NamedTuple, List, Tuple

from numpy.typing import NDArray

@dataclass(frozen=True)
class MidpointState:
    """State for the Midpoint computation"""
    midpoint: float
    last_window: List[float]
    period: int
    ...

class MidpointResult(NamedTuple):
    """Result of the Midpoint computation"""
    values: NDArray
    state: MidpointState

def midpoint(
    data: NDArray,
    period: int = 14,
    release_gil: bool = False
) -> MidpointResult | Tuple[NDArray, MidpointState]:
    """
    Midpoint: Middle point of the highest high and lowest low over a specified period.
    ----------

    Parameters
    ----------
    data : NDArray
        Input data for the Midpoint calculation, typically a price series.
    period : int, default 14
    release_gil : bool, default False
        If ``True``, the GIL is released during the computation.
        This is useful when using this function in a multi-threaded context.

    Returns
    -------
    MidpointResult
        A named tuple containing the result of the Midpoint computation.
        - ... TODO:OUTPUTS
        - state: **MidpointState** with (TODO:ATTRIBUTES)
    """
    ...

def midpoint_next(
    new_value: float,
    state: MidpointState
) -> MidpointState:
    """
    Update the Midpoint state with the next data.

    Parameters
    ----------
    new_value : float
        The next data point to include in the Midpoint computation.
    state : MidpointState
        The current state of the Midpoint computation.

    Returns
    -------
    MidpointState
        The updated state after including the new value.
    """
    ...
