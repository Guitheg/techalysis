import technicalysis as tx
from numpy import testing
import numpy as np

def test_sma_numpy_success(csv_loader):
   df = csv_loader("sma")
   out = tx.sma(np.array(df["close"]), 30)
   testing.assert_allclose(out, np.array(df["out"]))

def test_sma_pandas_success(csv_loader):
   df = csv_loader("sma")
   out = tx.sma(df["close"], 30)
   testing.assert_allclose(out, df["out"])

def test_thread_sma(thread_test):
   def sma_tx_lambda(data):
      return tx.sma(data, 30)

   thread_test(sma_tx_lambda, n_threads=4)