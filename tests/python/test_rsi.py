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

def test_thread_rsi(thread_test):
   def rsi_tx_lambda(data):
      return tx.rsi(data, 30)

   thread_test(rsi_tx_lambda, n_threads=4)