#!/bin/bash
set -e
cargo check --all-features --workspace
echo ">>>>>>>> PUBLISHING RELEASE FOR REPO"
bash scripts/release.sh
echo ">>>>>>>> PUBLISHING TO CRATES.IO NEXT"
sleep 10
cargo publish -p libpt-log
cargo publish -p libpt-core
cargo publish -p libpt-bintols
cargo publish -p libpt
echo ">>>>>>>> PUBLISHING TO CSCHERR.DE NEXT"
sleep 3
cargo publish --registry cscherr -p libpt-log
cargo publish --registry cscherr -p libpt-core
cargo publish --registry cscherr -p libpt-bintols
cargo publish --registry cscherr -p libpt
