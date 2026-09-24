#!/bin/bash

set -ueo pipefail

cargo metadata --format-version 1 | jq '.packages[] | select(.name == "scryfall") | .features | keys[]' -r | while read -r feature; do
    cargo test --no-default-features --features "$feature,default-tls" -- --test-threads 1
done

cargo test --all-features
