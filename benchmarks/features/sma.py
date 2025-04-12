import timeit
import technicalysis as ta

def py_sma(data, window_size):
    if len(data) < window_size:
        return []
    return [sum(data[i:i+window_size]) / window_size for i in range(len(data) - window_size + 1)]

def benchmark_sma():
    iterations = 10
    data = [ i for i in range(100_000) ]
    window_size = 1_000

    duration = timeit.timeit(lambda: py_sma(data, window_size), number=iterations)
    average_time_py = duration / iterations
    
    duration = timeit.timeit(lambda: ta.sma(0, len(data) - 1, data, window_size), number=iterations)
    average_time_rs = duration / iterations
    print(f"Exécution moyenne sur {iterations} itérations. Python : {average_time_py:.10f} secondes, Rust : {average_time_rs:.10f} secondes")