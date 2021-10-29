#!/bin/bash

set -e

for crate in "std" "core" "alloc"; do
    ./x.py doc "library/${crate}" --stage 1
    mv "build/x86_64-unknown-linux-gnu/doc/${crate}.json" ../roogle-index/crate
done
