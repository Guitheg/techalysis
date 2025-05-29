import argparse
from indicators.rsi import benchmark_rsi
from indicators.sma import benchmark_sma
from indicators.ema import benchmark_ema
from indicators.macd import benchmark_macd

BENCHMARKS = {
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