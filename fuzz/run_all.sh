#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"


mapfile -t targets < <(cargo fuzz list)

perform_fuzzing=1
failures=()
runs=5000000
max_len=4096

for t in "${targets[@]}"; do
  echo "=== running $t ==="
  if ! cargo +nightly fuzz run "$t" -s none -- -runs="$runs" -max_len="$max_len" ; then
    echo "❌  $t failed"
    failures+=("$t")
    perform_fuzzing=0
  else
    echo "✅  $t passed"
    perform_fuzzing=0
  fi
done

echo
if ((${#failures[@]})); then
  printf '❌ Fuzz failures (%d): %s\n' "${#failures[@]}" "${failures[*]}"
  exit 2
else
  if (( perform_fuzzing )); then
    echo "❌ Something went wrong :("
  else
    echo "✅ All fuzz targets passed 🎉"
  fi
  exit ${perform_fuzzing}
fi
