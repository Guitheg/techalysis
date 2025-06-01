from concurrent.futures import ThreadPoolExecutor, as_completed
import technicalysis as tx
from numpy import testing
import numpy as np
import time

def test_ema_numpy_success(csv_loader):
   df = csv_loader("ema")
   out = tx.ema(np.array(df["close"]), 30, 0.06451612903225806)
   testing.assert_allclose(out, np.array(df["out"]))

def test_ema_pandas_success(csv_loader):
   df = csv_loader("ema")
   out = tx.ema(df["close"], 30)
   testing.assert_allclose(out, df["out"])

def test_thread_ema(thread_test):
   def ema_tx_lambda(data):
      return tx.ema(data, 30)

   thread_test(ema_tx_lambda, n_threads=4)
