#!/bin/bash
set -e
cargo check --all-features
echo ">>>>>>>> SELECT A NEW VERSION"
cargo ws version --no-git-commit
NEW_VERSION=$(cat Cargo.toml | rg '^\s*version\s*=\s*"([^"]*)"\s*$' -or '$1')
bash scripts/set_all_versions.sh $NEW_VERSION
git add -A
git commit -m "Release v$NEW_VERSION" || (echo ">>>>>>>> COMMIT FAILED OR THERE WAS NOTHING TO COMMIT"; sleep 5)
echo ">>>>>>>> SKIP!!!!!"
cargo ws version --amend
echo ">>>>>>>> PUBLISHING RELEASE FOR REPO"
bash scripts/release.sh
echo ">>>>>>>> PUBLISHING TO CRATES.IO NEXT"
sleep 10
cargo publish -p libpt-log
cargo publish -p libpt-core
cargo publish -p libpt-bintols
cargo publish -p libpt-math
cargo publish -p libpt-net
cargo publish -p libpt
echo ">>>>>>>> PUBLISHING TO CSCHERR.DE NEXT"
sleep 3
cargo publish --registry cscherr -p libpt-log
cargo publish --registry cscherr -p libpt-core
cargo publish --registry cscherr -p libpt-bintols
cargo publish --registry cscherr -p libpt-math
cargo publish --registry cscherr -p libpt-net
cargo publish --registry cscherr -p libpt
