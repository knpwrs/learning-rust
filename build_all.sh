#!/usr/bin/env sh

find . -name 'Cargo.toml' | while read file; do
    pushd $(dirname $file)
    cargo build
    popd
done
