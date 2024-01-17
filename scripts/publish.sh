#!/bin/bash
set -e
cargo ws publish --registry cscherr --publish-as-is || cargo publish --registry cscherr -p libpt
