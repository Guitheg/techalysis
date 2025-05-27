import csv
import numpy as np
import talib
from pathlib import Path
import argparse

DATA_DIR = Path(__file__).parent.parent / "tests" / "data" / "oracle"
DATA_DIR.mkdir(parents=True, exist_ok=True)
RAND = np.random.default_rng(seed=42)  
CASES = {
    "EMA":  dict(timeperiod=30),
    "SMA":  dict(timeperiod=30),
    "RSI":  dict(timeperiod=14),
    "MACD": dict(fastperiod=12, slowperiod=26, signalperiod=9),
}

def generate_test_data(name, **kwargs):
    input_data = RAND.random(1000) * 100.0
    output_data = getattr(talib, name)(input_data, **kwargs)
    fn = DATA_DIR / f"{name.lower()}.csv"
    with fn.open("w", newline="") as fp:
        w = csv.writer(fp)
        if isinstance(output_data, tuple):
            w.writerow(["in"] + ["out" + str(i) for i in range(len(output_data))])
            w.writerows(zip(input_data, *output_data))
        else:
            w.writerow(["in", "out"])
            w.writerows(zip(input_data, output_data))
    print("create", fn)

def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("--name", type=str, nargs="*")
    return parser.parse_args()

def main():
    args = parse_args()
    if args.name is None:
        for name, kwargs in CASES.items():
            generate_test_data(name, **kwargs)
    else:
        for name in args.name:
            generate_test_data(name, **CASES[name])

if __name__ == "__main__":
    main()