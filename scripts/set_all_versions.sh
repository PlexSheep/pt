#!/bin/bash
export NEW_VER=$1
pwd
sed -i 's/\(^\s*version\)\s*=\s*"\([^"]*\)"$/\1 = "'$NEW_VER'"/g' Cargo.toml
find * -name 'Cargo.toml' -type f \
    -exec sed -i 's/\(libpt.*version\s*=\s*\)"[^"]*"/\1"'$NEW_VER'"/g' Cargo.toml {} +
