
from dataclasses import dataclass
from typing import NamedTuple, Optional, Tuple

from numpy.typing import NDArray

@dataclass(frozen=True)
class TrimaState:
    """State for the Trima computation"""
    # TODO: DEFINE STATE ATTRIBUTES
    ...

class TrimaResult(NamedTuple):
    """Result of the Trima computation"""
    # TODO: DEFINE RESULTS OUTPUTS ATTRIBUTES
    state: TrimaState

def trima(
    # TODO: DEFINE ARGUMENTS INPUTS
    release_gil: bool = False
) -> TrimaResult | Tuple[NDArray, TrimaState]:
    # TODO: FILL THE DOCUMENTATION
    """
    Trima: ...
    ----------
    TODO: DESCRIPTION

    Parameters
    ----------
    TODO:ARG_NAME : TODO:ARG_TYPE
        TODO:DESCRIPTION

    release_gil : bool, default False
        If ``True``, the GIL is released during the computation.
        This is useful when using this function in a multi-threaded context.

    Returns
    -------
    TrimaResult
        A named tuple containing the result of the Trima computation.
        - ... TODO:OUTPUTS
        - state: **TrimaState** with (TODO:ATTRIBUTES)
    """
    ...

def trima_next(
    # TODO: DEFINE ARGUMENTS INPUTS
    state: TrimaState
) -> TrimaState:
    """
    Update the Trima state with the next data.

    Parameters
    ----------
    TODO:ARG_NAME : TODO:ARG_TYPE
        TODO:DESCRIPTION
    
    state : TrimaState
        The current state of the Trima computation.

    Returns
    -------
    TrimaState
        The updated state after including the new value.
    """
    ...