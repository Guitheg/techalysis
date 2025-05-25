def time_as_str(seconds) -> str:
    if seconds > 1:
        return f"{seconds:10f} s"
    if seconds < 1/1_000:
        return f"{(seconds * 1_000_000):10f} µs"
    if seconds < 1:
        return f"{(seconds*1000):10f} ms"
    