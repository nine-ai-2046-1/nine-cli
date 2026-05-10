#!/usr/bin/env sh
# Minimal automated test for the example create-skill
# It attempts to locate the nine-cli binary and run `nine-cli skill verify . --json`
# Exits with non-zero on failure.

set -eu

# Try environment override first
if [ -n "${TEST_BIN:-}" ]; then
  BIN="$TEST_BIN"
elif [ -x "$(pwd)/../../../..//target/debug/nine-cli" ]; then
  BIN="$(pwd)/../../../..//target/debug/nine-cli"
elif command -v nine-cli >/dev/null 2>&1; then
  BIN="nine-cli"
else
  echo "ERROR: nine-cli binary not found. Build the project or set TEST_BIN to the binary path." >&2
  exit 2
fi

echo "Using nine-cli at: $BIN"

OUT="$($BIN skill verify . --json 2>&1)" || {
  echo "nine-cli verify failed to run:" >&2
  echo "$OUT" >&2
  exit 3
}

echo "verify output: $OUT"

# Parse JSON and check success==true using Python (available in CI)
python3 - <<PY
import sys, json
try:
    j = json.loads('''$OUT''')
except Exception as e:
    print('invalid json output:', e, file=sys.stderr)
    sys.exit(4)
if not isinstance(j, dict) or not j.get('success'):
    print('verify reported failure:', j, file=sys.stderr)
    sys.exit(5)
print('verify success')
PY

echo "OK"
