#!/bin/bash
set -e
cargo check --all-features
cargo ws version --no-git-commit
NEW_VERSION=$(cat Cargo.toml| rg '^\s*version\s*=\s*"([^"]*)"\s*$' -or '$1')
bash scripts/set_all_versions.sh $NEW_VERSION
git add -A
git commit -m "Release v$NEW_VERSION"
cargo ws publish --registry cscherr --amend --publish-as-is
cargo publish --registry cscherr -p libpt
