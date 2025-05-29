import technicalysis as tx
from numpy import testing
import numpy as np

def test_ema_numpy_success(csv_loader):
   df = csv_loader("ema")
   out = tx.ema(np.array(df["close"]), 30, 0.06451612903225806)
   testing.assert_allclose(out, np.array(df["out"]))

def test_ema_pandas_success(csv_loader):
   df = csv_loader("ema")
   out = tx.ema(df["close"], 30)
   testing.assert_allclose(out, df["out"])