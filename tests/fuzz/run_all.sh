#!/usr/bin/env bash
set -euo pipefail

FUZZ_DIR=tests/fuzz

perform_fuzzing=1
failures=()
runs=5000000
max_len=4096

mapfile -t targets < <(cargo fuzz list --fuzz-dir $FUZZ_DIR)

if (( ${#targets[@]} == 0 )); then
  echo "❌ No fuzz targets found. Exiting."
  exit 0
  else
  echo "Found ${#targets[@]} fuzz targets:"
  for t in "${targets[@]}"; do
    echo " - $t"
  done
fi

for t in "${targets[@]}"; do
  echo "=== running $t ==="
  if ! cargo +nightly fuzz run $t --fuzz-dir $FUZZ_DIR -s none -- -runs="$runs" -max_len="$max_len" ; then
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
    echo "❌ Something went wrong"
  else
    echo "✅ All fuzz targets passed 🎉"
  fi
  exit ${perform_fuzzing}
fi
