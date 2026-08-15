#!/usr/bin/env bash

set -xueo pipefail

cmd=${1:-"test"}

targets_except_doc="--lib --bins --examples --tests --benches"

cargo metadata --format-version 1 | jq '.packages[] | select(.name == "scryfall") | .features | keys[]' -r | while read -r feature; do
    cargo "$cmd" $targets_except_doc --no-default-features --features "$feature,default-tls"
    if [ "$cmd" = "test" ]; then 
        cargo "$cmd" --doc --no-default-features --features "$feature,default-tls,auto_rate_limit_blocking" -- --test-threads 1
    fi
done

cargo "$cmd" $targets_except_doc --all-features
if [ "$cmd" = "test" ]; then 
    cargo "$cmd" --doc --all-features  -- --test-threads 1 # auto_rate_limit_blocking is already included
fi

