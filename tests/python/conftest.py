import pytest
from pathlib import Path
from typing import Tuple, Callable
from numpy.typing import NDArray
import pandas as pd

DATA_DIR = "tests/data/generated/{feature_name}.csv"

@pytest.fixture
def csv_loader() -> Callable[[str], pd.DataFrame] :
    def _load(feature_name: str) -> Tuple[pd.Series, pd.Series]:
        csv_path = DATA_DIR.format(feature_name = feature_name)
        return pd.read_csv(csv_path, delimiter=",")
    return _load
