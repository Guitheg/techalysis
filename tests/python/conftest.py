import pytest
from pathlib import Path
from typing import Tuple, Callable
from numpy.typing import NDArray
import pandas as pd

DATA_DIR = "tests/data/oracle/{feature_name}.csv"

def load_csv(feature_name: str) -> Tuple[pd.Series, pd.Series]:
    csv_path = DATA_DIR.format(feature_name = feature_name)
    df = pd.read_csv(csv_path, delimiter=",")
    return df["in"], df["out"]

@pytest.fixture
def csv_numpy_loader() -> Callable[[str], Tuple[NDArray, NDArray]]:
    def _load(feature_name: str) -> Tuple[NDArray, NDArray]:
        df_in, df_out = load_csv(feature_name)
        return df_in.to_numpy(), df_out.to_numpy()
    return _load

@pytest.fixture
def csv_pandas_loader() -> Callable[[str], Tuple[NDArray, NDArray]]:
    def _load(feature_name: str) -> Tuple[NDArray, NDArray]:
        return load_csv(feature_name)
    return _load
