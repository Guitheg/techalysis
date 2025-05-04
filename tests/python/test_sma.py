import technicalysis as tx
from numpy import testing

def test_sma_numpy_success(csv_numpy_loader):
   inp, expected = csv_numpy_loader("sma")
   out = tx.sma(inp, 30)
   testing.assert_allclose(out, expected)

def test_sma_pandas_success(csv_pandas_loader):
   inp, expected = csv_pandas_loader("sma")
   out = tx.sma(inp, 30)
   assert(type(out) == type(inp))
   testing.assert_allclose(out, expected)