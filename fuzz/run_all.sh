#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"


mapfile -t targets < <(cargo fuzz list)

failures=()
runs=5000000
max_len=4096

for t in "${targets[@]}"; do
  echo "=== running $t ==="
  if ! cargo +nightly fuzz run "$t" -s none -- -runs="$runs" -max_len="$max_len" ; then
    echo "❌  $t failed"
    failures+=("$t")
  else
    echo "✅  $t passed"
  fi
done

echo
if ((${#failures[@]})); then
  printf '❌ Fuzz failures (%d): %s\n' "${#failures[@]}" "${failures[*]}"
  exit 1
else
  echo "✅ All fuzz targets passed 🎉"
  exit 0
fi
