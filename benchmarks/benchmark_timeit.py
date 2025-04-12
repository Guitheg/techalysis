import timeit
import technicalysis

def benchmark_py_add():
    iterations = 1_000_000
    duration = timeit.timeit(lambda: technicalysis.py_add(1, 2), number=iterations)
    average_time = duration / iterations
    print(f"Exécution moyenne de py_add sur {iterations} itérations : {average_time:.10f} secondes")

    duration = timeit.timeit(lambda: 1+2, number=iterations)
    average_time = duration / iterations
    print(f"Exécution moyenne de __add__ sur {iterations} itérations : {average_time:.10f} secondes")

if __name__ == '__main__':
    benchmark_py_add()