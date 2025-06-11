
from dataclasses import dataclass
from typing import NamedTuple, Optional, Tuple

from numpy.typing import NDArray

@dataclass(frozen=True)
class TemaState:
    """State for the Tema computation"""
    tema: float
    ema_1: float
    ema_2: float
    ema_3: float
    period: int
    alpha: float
    ...

class TemaResult(NamedTuple):
    """Result of the Tema computation"""
    values: NDArray
    state: TemaState

def tema(
    data: NDArray,
    period: int = 14,
    alpha: Optional[float] = None,
    release_gil: bool = False
) -> TemaResult | Tuple[NDArray, TemaState]:
    """
    Tema: Triple Exponential Moving Average
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
    TemaResult
        A named tuple containing the result of the Tema computation.
        - values: **NDArray** of the same length as *data* containing the Tema values.
        - state: **TemaState** with (tema: float, ema_1: float, ema_2: float, ema_3: float, period: int, alpha: float)
    """
    ...

def tema_next(
    new_value: float,
    state: TemaState
) -> TemaState:
    """
    Update the Tema state with the next data.

    Parameters
    ----------
    new_value : float
        The new value to include in the Tema computation.
    
    state : TemaState
        The current state of the Tema computation.

    Returns
    -------
    TemaState
        The updated state after including the new value.
    """
    ...