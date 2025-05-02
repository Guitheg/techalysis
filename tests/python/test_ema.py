import technicalysis as tx
from numpy import testing

def test_ema_numpy_success(csv_numpy_loader):
   inp, expected = csv_numpy_loader("ema")
   out = tx.ema(inp, 30, 2)
   testing.assert_allclose(out, expected)

def test_ema_pandas_success(csv_pandas_loader):
   inp, expected = csv_pandas_loader("ema")
   out = tx.ema(inp, 30, 2)
   assert(type(out) == type(inp))
   testing.assert_allclose(out, expected)