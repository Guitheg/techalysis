import technicalysis as tx
from numpy import testing

def test_rsi_numpy_success(csv_numpy_loader):
   inp, expected = csv_numpy_loader("rsi")
   out = tx.rsi(inp, 14)
   testing.assert_allclose(out, expected)

def test_rsi_pandas_success(csv_pandas_loader):
   inp, expected = csv_pandas_loader("rsi")
   out = tx.rsi(inp, 14)
   assert(type(out) == type(inp))
   testing.assert_allclose(out, expected)