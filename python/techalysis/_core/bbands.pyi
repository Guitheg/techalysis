
from dataclasses import dataclass
from typing import NamedTuple, Tuple, List
from enum import Enum

from numpy.typing import NDArray

@dataclass(frozen=True)
class BBandsState:
    """State for the BBands computation"""
    upper: float
    middle: float
    lower: float
    mean_sma: float
    mean_sq: float
    window: List
    period: int
    std_up: float
    std_down: float
    ...

class BBandsResult(NamedTuple):
    """Result of the BBands computation"""
    upper: NDArray
    middle: NDArray
    lower: NDArray
    state: BBandsState

class BBandsMA(Enum):
    SMA = 0
    EMA = 1

def bbands(
    data: NDArray,
    period: int = 20,
    std_up: float = 2.0,
    std_down: float = 2.0,
    ma_type: BBandsMA = BBandsMA.SMA,
    release_gil: bool = False
) -> BBandsResult | Tuple[NDArray, BBandsState]:
    """
    BBands: Bollinger Bands computation.
    ----------

    Parameters
    ----------
    data : NDArray
        Input data for the BBands computation, typically a closure price series.

    period : int, default 20
        The number of periods to use for the moving average and standard deviation.

    std_up : float, default 2.0
        The multiplier of standard deviations for the upper band.

    std_down : float, default 2.0
        The multiplier of standard deviations for the lower band.
    
    ma_type : BBandsMA, default BBandsMA.SMA
        The type of moving average to use for the middle band.
        Options:
        - BBandsMA.SMA: Simple Moving Average
        - BBandsMA.EMA: Exponential Moving Average

    release_gil : bool, default False
        If ``True``, the GIL is released during the computation.
        This is useful when using this function in a multi-threaded context.

    Returns
    -------
    BBandsResult
        A named tuple containing the result of the BBands computation.
        - upper: **NDArray** with the upper Bollinger Band values.
        - middle: **NDArray** with the middle Bollinger Band values (moving average).
        - lower: **NDArray** with the lower Bollinger Band values.
        - state: **BBandsState** with (upper, middle, lower, mean_sma, mean_sq, window, period, std_up, std_down)
    """
    ...

def bbands_next(
    new_value: float,
    state: BBandsState
) -> BBandsState:
    """
    Update the BBands state with the next data.

    Parameters
    ----------
    new_value : float
        The new value to include in the BBands computation.
    
    state : BBandsState
        The current state of the BBands computation.

    Returns
    -------
    BBandsState
        The updated state after including the new value.
    """
    ...