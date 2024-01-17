#!/bin/bash
set -e
cargo ws publish --registry cscherr
cargo publish --registry cscherr -p libpt
