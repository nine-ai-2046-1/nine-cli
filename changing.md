Changelog for dev/welcome-i18n work

What I changed
- Added an examples/skills/create-skill/tests/ directory with .gitkeep
- Added a minimal test runner examples/skills/create-skill/tests/test_run.sh which calls nine-cli skill verify <skilldir> --json and asserts success
- Ensured examples/skills/create-skill/cli/run is executable
- Committed and pushed changes to branch dev/welcome-i18n

How I verified
- cargo build succeeded and produced target/debug/nine-cli
- Ran the test runner with TEST_BIN=./target/debug/nine-cli and it returned success (exit 0)
- Ran cargo test (no tests in Rust code changed; all crates report 0 tests but build and tests completed)

Notes
- The example test runner expects Python3 to be available in CI to parse JSON output. It can be adapted to use jq or other parsers.
- Next: implement richer verify JSON (failures array + non-zero exit code on verification failures) on a dedicated branch dev/verify-json.
