from concurrent.futures import ThreadPoolExecutor
import pytest
from typing import Tuple, Callable
from numpy.typing import NDArray
import pandas as pd
import numpy as np
import time

DATA_DIR = "tests/data/generated/{feature_name}.csv"

@pytest.fixture
def csv_loader() -> Callable[[str], pd.DataFrame] :
    def _load(feature_name: str) -> Tuple[pd.Series, pd.Series]:
        csv_path = DATA_DIR.format(feature_name = feature_name)
        return pd.read_csv(csv_path, delimiter=",")
    return _load


@pytest.fixture
def thread_test() -> Callable[[Callable], None]:
    def _thread_test(tx_lambda: Callable, n_threads: int = 4, tolerance: float = 0.5) -> None:
        data = np.array([float(i) for i in range(10_000_000)])
        t0 = time.perf_counter()
        for _ in range(n_threads):
            _ = tx_lambda(data)
        seq_time = (time.perf_counter() - t0) / n_threads
        with ThreadPoolExecutor(max_workers=n_threads) as pool:
            t0 = time.perf_counter()
            futures = [pool.submit(tx_lambda, data) for _ in range(n_threads)]
            _ = [future.result() for future in futures]
            conc_time = (time.perf_counter() - t0) / n_threads
        
        print(f"Sequential time: {seq_time:.4f} seconds")
        print(f"Concurrent time: {conc_time:.4f} seconds")
        conc_compoare = (conc_time * n_threads) * tolerance
        print(f"Concurrent time for comparison with {tolerance*100:.0f}% tolerance: {conc_compoare:.4f} seconds")
        assert conc_compoare < seq_time, f"Concurrent execution should be faster than sequential execution. {conc_time:.4f} <? {seq_time:.4f}"
    return _thread_test