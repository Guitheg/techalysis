import csv
from typing import List
import numpy as np
import pandas as pd
import talib
from pathlib import Path
import argparse
from utils.logger import logger
from utils import ohlcv

DATA_DIR = Path(__file__).parent.parent / "tests" / "data" / "generated"
DATA_DIR.mkdir(parents=True, exist_ok=True)
RAND = np.random.default_rng(seed=42)

class Configuration():
    def __init__(self, module: object, fct_name: str, input_names: List[str], parameters: dict, output_names: List[str], sample_size: int = 1000):
        self.module = module
        self.fct_name = fct_name
        self.input_names = input_names
        self.parameters = parameters
        self.output_names = output_names
        self.sample_size = sample_size

CONFIG_DICT = {
    "EMA": Configuration(talib, "EMA", ["close"], dict(timeperiod=30), ["out"]),
    "SMA": Configuration(talib, "SMA", ["close"], dict(timeperiod=30), ["out"]),
    "RSI": Configuration(talib, "RSI", ["close"], dict(timeperiod=14), ["out"]),
    "MACD": Configuration(talib, "MACD", ["close"], dict(fastperiod=12, slowperiod=26, signalperiod=9), ["macd", "signal", "histogram"]),
    "BBANDS": Configuration(talib, "BBANDS", ["close"], dict(timeperiod=20, nbdevup=2, nbdevdn=2, matype=0), ["upper", "middle", "lower"]),
    "WMA": Configuration(talib, "WMA", ["close"], dict(timeperiod=30), ["out"]),
    "DEMA": Configuration(talib, "DEMA", ["close"], dict(timeperiod=30), ["out"]),
    "TEMA": Configuration(talib, "TEMA", ["close"], dict(timeperiod=30), ["out"]),
    "TRIMA": Configuration(talib, "TRIMA", ["close"], dict(timeperiod=30), ["out"]),
    "T3": Configuration(talib, "T3", ["close"], dict(timeperiod=20, vfactor=0.7), ["out"]),
    "KAMA": Configuration(talib, "KAMA", ["close"], dict(timeperiod=30), ["out"]),
    "MIDPOINT": Configuration(talib, "MIDPOINT", ["close"], dict(timeperiod=14), ["out"]),
    "MIDPRICE": Configuration(talib, "MIDPRICE", ["high", "low"], dict(timeperiod=14), ["out"]),
}

def generate_test_data(filename: str, configuration: Configuration, seed: int):
    logger.info(f"📊 ({configuration.fct_name}) Generating test data with parameters: {configuration.parameters}")
    generated_data = ohlcv.random_walk(configuration.sample_size, scale=1.5, start_offset = 50, seed = seed)
    output_data = getattr(configuration.module, configuration.fct_name).__call__(
        *[generated_data[name].values for name in configuration.input_names],
        **configuration.parameters
    )
    if isinstance(output_data, tuple):
        output_df = pd.DataFrame(
            {
                name: output_data[i] for i, name in enumerate(configuration.output_names)
            }
        )
    else:
        output_df = pd.DataFrame(
            {
                configuration.output_names[0]: output_data
            }
        )

    final_df = pd.concat([generated_data[configuration.input_names], output_df], axis=1)

    final_df.to_csv(
        DATA_DIR / f"{filename}.csv",
        index=False,
        float_format="%.8f",
        na_rep="nan",
    )

    logger.info(f"✅ ({configuration.fct_name}) Successfully write data at : {DATA_DIR / filename}.csv")


class ParseKwargs(argparse.Action):
    def __call__(self, parser, namespace, values, option_string=None):
        setattr(namespace, self.dest, dict())
        for value in values:
            key, value = value.split('=')
            if "." in value:
                try:
                    value = float(value)
                except ValueError:
                    logger.warning(f"Could not convert {value} to float, keeping as string.")
            elif value.isdigit():
                try:
                    value = int(value)
                except ValueError:
                    logger.warning(f"Could not convert {value} to int, keeping as string.")
            elif value.lower() in ['true', 'false']:
                value = value.lower() == 'true'
            getattr(namespace, self.dest)[key] = value

def dict_to_posix_filename(d: dict) -> str:
    """Convert a dictionary to a posix filename."""
    return "_".join(f"{k}={v}" for k, v in d.items() if v is not None).replace(" ", "_").replace("/", "_").replace("\\", "_").replace("=","-")

def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("-n", "--name", type=str)
    parser.add_argument("--seed", type=int, default=5)
    parser.add_argument("--args", nargs='*', action=ParseKwargs)
    parser.add_argument("--size", type=int, default=1000, help="Sample size for the generated data.")
    return parser.parse_args()

def main():
    args = parse_args()
    if args.name is None:
        if args.args is not None:
            logger.warning("Ignoring args, generating all test data.")
        for configuration in CONFIG_DICT.values():
            configuration.sample_size = args.size
            generate_test_data(configuration.fct_name.lower(), configuration, args.seed)
    else:
        config = CONFIG_DICT.get(args.name)
        file_name = args.name.lower()
        if args.args is not None:
            config.parameters.update(args.args)
            config.sample_size = args.size
            file_name += f"_{dict_to_posix_filename(args.args)}"
        generate_test_data(file_name, config, args.seed)

if __name__ == "__main__":
    main()
