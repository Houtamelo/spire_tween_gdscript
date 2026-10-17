#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd -- "$repo_root"

cargo run --locked -p spire_release -- build --platform linux --out "${SPIRE_RELEASE_OUT:-dist/staging/linux}"
