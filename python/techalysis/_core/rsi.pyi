
from dataclasses import dataclass
from typing import NamedTuple, Tuple
from numpy.typing import NDArray

@dataclass(frozen=True)
class RsiState:
    """State for the RSI computation"""
    rsi: float
    avg_gain: float
    avg_loss: float
    period: int
    
    def __repr__(self):
        return (f"RsiState(rsi={self.rsi:.4f}, "
                f"avg_gain={self.avg_gain:.4f}, "
                f"avg_loss={self.avg_loss:.4f}, "
                f"period={self.period})")
    ...

class RsiResult(NamedTuple):
    """Result of the RSI computation"""
    values: NDArray
    state: RsiState

def rsi(
    data: NDArray,
    period: int = 14,
    release_gil: bool = False
) -> Tuple[NDArray, RsiState]:
    """
    RSI: Relative Strength Index
    ----------

    Parameters
    ----------
    data : 1-D array
        One dimensional array.

    period : int
        Size of the rolling window (must be ``> 0``).

    release_gil : bool, default False
        If ``True``, the GIL is released during the computation.
        This is useful when using this function in a multi-threaded context.

    Returns
    -------
    RsiResult
        A named tuple containing the result of the RSI computation.
        - values: **1-D array** of the same length as *data* containing the RSI.
        - state: **RsiState** (rsi: float, avg_gain: float, avg_loss: float, period: int)
    """
    ...

def rsi_next(
    new_value: float,
    state: RsiState
) -> RsiState:
    """
    Update the RSI state with the next value.

    Parameters
    ----------
    new_value : float
        The next value to include in the RSI calculation.

    state : RsiState
        The current state of the RSI computation.

    Returns
    -------
    RsiState
        The updated state of the RSI computation.
    """
    ...