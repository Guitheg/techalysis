
from dataclasses import dataclass
from typing import NamedTuple, List, Tuple

from numpy.typing import NDArray

@dataclass(frozen=True)
class KamaState:
    """State for the Kama computation"""
    kama: float
    last_window: List[float]
    roc_sum: float
    trailing_value: float
    period: int
    ...

class KamaResult(NamedTuple):
    """Result of the Kama computation"""
    values: NDArray
    state: KamaState

def kama(
    data: NDArray,
    period: int = 30,
    release_gil: bool = False
) -> KamaResult | Tuple[NDArray, KamaState]:
    """
    Kama: Kaufman's Adaptive Moving Average (KAMA)
    ----------

    Parameters
    ----------
    data : NDArray
        Input data for the Kama computation, typically a price series.
    period : int, default 30
        The period over which to compute the Kama.
    release_gil : bool, default False
        If ``True``, the GIL is released during the computation.
        This is useful when using this function in a multi-threaded context.

    Returns
    -------
    KamaResult
        A named tuple containing the result of the Kama computation.
        - values: **NDArray** with the computed Kama values.
        - state: `KamaState`
    """
    ...

def kama_next(
    new_value: float,
    state: KamaState
) -> KamaState:
    """
    Update the Kama state with the next data.

    Parameters
    ----------
    new_value : float
        The new value to include in the Kama computation.

    state : KamaState
        The current state of the Kama computation.

    Returns
    -------
    KamaState
        The updated state after including the new value.
    """
    ...
