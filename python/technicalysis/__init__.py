from functools import wraps
from itertools import chain
from technicalysis.core import sma, ema # type: ignore

TECHNICALYSIS_CORE_FCT_NAMES = [
    "sma",
    "ema"
]


try:
    from pandas import Series as _pd_Series
except ImportError as import_error:
    try:
        if not isinstance(import_error, ModuleNotFoundError) or import_error.name != 'pandas':
            raise import_error
    except NameError:
        pass

    _pd_Series = None

if _pd_Series is not None:
    def _wrapper(func):
        @wraps(func)
        def wrapper(*args, **kwds):
            if _pd_Series is not None:
                use_pd = any(isinstance(arg, _pd_Series) for arg in args) or any(isinstance(v, _pd_Series) for v in kwds.values())
            else:
                use_pd = False

            if use_pd:
                index = next(arg.index
                             for arg in chain(args, kwds.values())
                             if isinstance(arg, _pd_Series))

                _args = [arg.to_numpy().astype(float) if isinstance(arg, _pd_Series) else
                         arg for arg in args]
                _kwds = {k: v.to_numpy().astype(float) if isinstance(v, _pd_Series) else
                            v for k, v in kwds.items()}

            else:
                _args = args
                _kwds = kwds

            result = func(*_args, **_kwds)

            if use_pd:
                if isinstance(result, tuple):
                    return tuple(_pd_Series(arr, index=index) for arr in result)
                else:
                    return _pd_Series(result, index=index)

            else:
                return result

        return wrapper
else:
    _wrapper = lambda x: x

func = __import__("core", globals(), locals(), TECHNICALYSIS_CORE_FCT_NAMES, level=1)
for func_name in TECHNICALYSIS_CORE_FCT_NAMES:
    wrapped_func = _wrapper(getattr(func, func_name))
    setattr(func, func_name, wrapped_func)
    globals()[func_name] = wrapped_func


__all__ = TECHNICALYSIS_CORE_FCT_NAMES