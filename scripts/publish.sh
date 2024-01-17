#!/bin/bash
set -e
cargo ws publish --registry cscherr --publish-as-is --no-git-push
