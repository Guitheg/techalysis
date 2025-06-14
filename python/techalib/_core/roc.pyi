
from dataclasses import dataclass
from typing import NamedTuple, List, Tuple

from numpy.typing import NDArray

@dataclass(frozen=True)
class RocState:
    """State for the Roc computation"""
    roc: float
    last_window: List[float]
    period: int
    ...

class RocResult(NamedTuple):
    """Result of the Roc computation"""
    values: NDArray
    state: RocState

def roc(
    data: NDArray,
    period: int = 10,
    release_gil: bool = False
) -> RocResult | Tuple[NDArray, RocState]:
    """
    Roc: Rate of Change (ROC) Indicator
    ----------

    Parameters
    ----------
    data : NDArray
        Input data array, typically a time series of prices or values.
    period : int, default 10
        The number of periods over which to calculate the rate of change.
    release_gil : bool, default False
        If ``True``, the GIL is released during the computation.
        This is useful when using this function in a multi-threaded context.

    Returns
    -------
    RocResult
        A named tuple containing the result of the Roc computation.
        - values: NDArray
            The computed values.
        - state: `RocState`
    """
    ...

def roc_next(
    new_value: float,
    state: RocState
) -> RocState:
    """
    Update the Roc state with the next data.

    Parameters
    ----------
    new_value : float
        The new value to include in the Roc computation.
    state : RocState
        The current state of the Roc computation.

    Returns
    -------
    RocState
        The updated state after including the new value.
    """
    ...
