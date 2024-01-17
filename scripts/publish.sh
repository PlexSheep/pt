#!/bin/bash
cargo ws publish --registry cscherr
cargo ws publish --registry cscherr --no-git-commit --publish-as-is
cargo publish --registry cscherr -p libpt
