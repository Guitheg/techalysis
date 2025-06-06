from ._core import *

def __init__() -> list[str]:
    from ._core import __all__ as __core_all
    from .maptypes import __tuple2types__
    from importlib import import_module
    from numpy import ndarray

    core = import_module("._core", __name__)

    __all__ = [
        name for name in dir(core)
        if not name.startswith("_")
        and callable(getattr(core, name))
    ]

    try:
        from pandas import Series as _pd_Series
        from pandas import DataFrame as _pd_DataFrame
    except ModuleNotFoundError:
        _pd_Series = None
        _pd_DataFrame = None

    def convert_to_pandas(obj: object, index: list) -> object:
        if isinstance(obj, tuple):
            return tuple(convert_to_pandas(o, index) for o in obj)
        if isinstance(obj, ndarray):
            if obj.ndim == 1:
                return _pd_Series(obj, index=index)
            else:
                return _pd_DataFrame(obj, index=index)
        return obj
                    

    def wrapper(function):
        from functools import wraps
        from itertools import chain
        if _pd_Series is None or _pd_DataFrame is None:
            return function
        
        @wraps(function)
        def inner_wrapper(*args, **kwargs):
            use_pd = any(isinstance(arg, _pd_Series) for arg in chain(args, kwargs.values()))

            _args = args
            _kwargs = kwargs

            if use_pd:
                index = next(
                    arg.index
                    for arg in chain(args, kwargs.values())
                    if isinstance(arg, _pd_Series)
                )

                _args = [
                    arg.to_numpy().astype(float)
                    if isinstance(arg, _pd_Series) else arg
                    for arg in args
                ]

                _kwargs = {
                    k: (v.to_numpy().astype(float) if isinstance(v, _pd_Series) else v)
                    for k, v in kwargs.items()
                }

            out = function(*_args, **_kwargs)
            if use_pd:
                out = convert_to_pandas(out, index)
            return __tuple2types__(function, out)
        return inner_wrapper

    for name in __all__:
        wrapped = wrapper(getattr(core, name))
        globals()[name] = wrapped
        setattr(core, name, wrapped)

    return __all__

__all__ = __init__()
