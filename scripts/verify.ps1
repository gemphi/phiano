# verify.ps1 — the one check (docs/166): build, test, examples, the phinum door.
# Run from phinet\:  powershell -ExecutionPolicy Bypass -File crates\phiano\scripts\verify.ps1
$ErrorActionPreference = "Stop"
Write-Host "== cargo check --workspace --all-targets"
cargo check --workspace --all-targets
Write-Host "== cargo test --workspace"
cargo test --workspace 2>&1 | Select-String "test result|FAILED|panicked"
Write-Host "== cargo test -p phiano --features phinum (the door)"
cargo test -p phiano --features phinum --lib 2>&1 | Select-String "test result|FAILED"
Write-Host "== the two rings"
cargo test -p phinet --test personas 2>&1 | Select-String "test result"
Write-Host "== the Lisp, dry"
cargo run --release -p phinet --example lisp -- -e "(count (closure (pairs (a b) (b c) (c d))))"
Write-Host "done — expect 390 passed / 0 failed, and 6 from the Lisp"
