
from dataclasses import dataclass
from typing import NamedTuple, List, Tuple

from numpy.typing import NDArray

@dataclass(frozen=True)
class MidpriceState:
    """State for the Midprice computation"""
    midprice: float
    last_high_window: List[float]
    last_low_window: List[float]
    period: int
    ...

class MidpriceResult(NamedTuple):
    """Result of the Midprice computation"""
    values: NDArray
    state: MidpriceState

def midprice(
    high: NDArray,
    low: NDArray,
    period: int = 14,
    release_gil: bool = False
) -> MidpriceResult | Tuple[NDArray, MidpriceState]:
    """
    Midprice: Middle price of the high and low prices over a specified period.
    ----------

    Parameters
    ----------
    high : NDArray
        Array of high prices.
    low : NDArray
        Array of low prices.
    period : int, default 14
        The number of periods over which to calculate the midprice.
        Must be a positive integer.
    release_gil : bool, default False
        If ``True``, the GIL is released during the computation.
        This is useful when using this function in a multi-threaded context.

    Returns
    -------
    MidpriceResult
        A named tuple containing the result of the Midprice computation.
        - values: NDArray
            The computed midprice values.
        - state: **MidpriceState**
    """
    ...

def midprice_next(
    high_price: float,
    low_price: float,
    state: MidpriceState
) -> MidpriceState:
    """
    Update the Midprice state with the next data.

    Parameters
    ----------
    high_price : float
        The next high price to include in the computation.
    low_price : float
        The next low price to include in the computation.
    state : MidpriceState
        The current state of the Midprice computation.

    Returns
    -------
    MidpriceState
        The updated state after including the new value.
    """
    ...
