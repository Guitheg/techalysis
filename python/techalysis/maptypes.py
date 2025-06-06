from collections import namedtuple

FCT_TO_NAMEDTUPLE = {
    "bbands": namedtuple("BbandsResult", ["upper", "middle", "lower", "state"]),
    "ema": namedtuple("EmaResult", ["values", "state"]),
    "sma": namedtuple("SmaResult", ["values", "state"]),
    "rsi": namedtuple("RsiResult", ["values", "state"]),
    "macd": namedtuple("MacdResult", ["macd", "signal", "histogram", "state"]),
}

def __tuple2types__(function, result: tuple) -> object:
    tech_result = FCT_TO_NAMEDTUPLE.get(function.__name__)
    if tech_result is None:
        return result
    return tech_result(*result)