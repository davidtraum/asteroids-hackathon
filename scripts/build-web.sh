#!/usr/bin/env bash
set -euo pipefail

project_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$project_root"

if [[ -f "$HOME/.cargo/env" ]]; then
    # Make the script work from non-interactive shells and dev servers.
    # shellcheck disable=SC1091
    . "$HOME/.cargo/env"
fi

cargo build --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/debug/astroid-rust.wasm web/astroid-rust.wasm

echo "Built web/astroid-rust.wasm"
echo "Serve with: ./scripts/dev-web.py"
