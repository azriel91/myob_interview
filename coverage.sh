#! /bin/bash
set -euo pipefail

cargo coverage_clean
rm -rf ./target/coverage ./target/llvm-cov-target
mkdir -p ./target/coverage

# See `.config/cargo.toml`
cargo coverage

cargo coverage_merge
