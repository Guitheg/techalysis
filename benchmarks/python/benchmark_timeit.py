import argparse
from timeit_bench.roc import benchmark_roc
from timeit_bench.midprice import benchmark_midprice
from timeit_bench.midpoint import benchmark_midpoint
from timeit_bench.kama import benchmark_kama
from timeit_bench.t3 import benchmark_t3
from timeit_bench.trima import benchmark_trima
from timeit_bench.tema import benchmark_tema
from timeit_bench.dema import benchmark_dema
from timeit_bench.wma import benchmark_wma
from timeit_bench.bbands import benchmark_bbands
from timeit_bench.rsi import benchmark_rsi
from timeit_bench.sma import benchmark_sma
from timeit_bench.ema import benchmark_ema
from timeit_bench.macd import benchmark_macd

BENCHMARKS = {
    'roc': benchmark_roc,
    'midprice': benchmark_midprice,
    'midpoint': benchmark_midpoint,
    'kama': benchmark_kama,
    't3': benchmark_t3,
    'trima': benchmark_trima,
    'tema': benchmark_tema,
    'dema': benchmark_dema,
    'wma': benchmark_wma,
    'bbands': benchmark_bbands,
    'sma': benchmark_sma,
    'ema': benchmark_ema,
    'rsi': benchmark_rsi,
    'macd': benchmark_macd
}

def parse_args():
    parser = argparse.ArgumentParser(description="Benchmark technical indicators.")
    parser.add_argument(
        '-n', '--name', nargs='*',
        choices=list(BENCHMARKS.keys()),
        help="List of indicators to benchmark."
    )
    return parser.parse_args()


def main():
    args = parse_args()
    if not args.name:
        for name in BENCHMARKS:
            print(f"Running benchmark for {name}...")
            BENCHMARKS[name]()
    else:
        for name in args.name:
            if name in BENCHMARKS:
                print(f"Running benchmark for {name}...")
                BENCHMARKS[name]()
            else:
                print(f"Benchmark for {name} not found.")


if __name__ == '__main__':
    main()
