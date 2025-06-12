from termcolor import colored
def time_as_str(seconds) -> str:
    if seconds > 1:
        return f"{seconds:10f} s"
    if seconds < 1/1_000:
        return f"{(seconds * 1_000_000):10f} µs"
    if seconds < 1:
        return f"{(seconds*1000):10f} ms"

def print_benchmark(iteration: int, parameters: dict, **benchmarks):
    print(f"\tExécution moyenne sur {iteration} itérations ({parameters})")
    rust_time = benchmarks.get("rust")
    sorted_benchmarks = list(sorted(benchmarks.items(), key=lambda x: x[1]))
    time_ratio = sorted_benchmarks[1][1] / sorted_benchmarks[0][1]
    for i, (name, time) in enumerate(sorted_benchmarks):
        if i == 0:
            if name == "rust":
                time_ratio = sorted_benchmarks[1][1] / rust_time
                print(colored(f"\t\t*{name}: {time_as_str(time)}* ({time_ratio:.2f}x faster)", "green"))
            else:
                print(f"\t\t*{name}: {time_as_str(time)}*")
        else:
            if name == "rust":
                time_ratio = rust_time / sorted_benchmarks[0][1]
                print(colored(f"\t\t{name}: {time_as_str(time)} ({time_ratio:.2f}x slower)", "red"))
            else:
                print(f"\t\t{name}: {time_as_str(time)}")
