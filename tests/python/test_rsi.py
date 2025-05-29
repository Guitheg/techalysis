import technicalysis as tx
from numpy import testing
import numpy as np

def test_rsi_numpy_success(csv_loader):
   df = csv_loader("rsi")
   out = tx.rsi(np.array(df["close"]), 14)
   testing.assert_allclose(out, np.array(df["out"]))

def test_rsi_pandas_success(csv_loader):
   df = csv_loader("rsi")
   out = tx.rsi(df["close"], 14)
   testing.assert_allclose(out, df["out"])