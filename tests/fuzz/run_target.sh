#!/usr/bin/env bash
set -euo pipefail

FUZZ_DIR=tests/fuzz

perform_fuzzing=1
failures=()
runs=5000000
max_len=4096

target="$1"
if [[ -z "$target" ]]; then
  echo "❌ No fuzz target specified. Usage: $0 <target_name>"
  exit 1
fi

echo "=== running $target ==="
if ! cargo +nightly fuzz run $target --fuzz-dir $FUZZ_DIR -s none -- -runs="$runs" -max_len="$max_len" ; then
echo "❌  $target failed"
failures+=("$target")
perform_fuzzing=0
else
echo "✅  $target passed"
perform_fuzzing=0
fi


echo
if ((${#failures[@]})); then
  printf '❌ Fuzz failures (%d): %s\n' "${#failures[@]}" "${failures[*]}"
  exit 2
else
  if (( perform_fuzzing )); then
    echo "❌ Something went wrong"
  else
    echo "✅ Success 🎉"
  fi
  exit ${perform_fuzzing}
fi
