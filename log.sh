#!/usr/bin/env bash

name=$(cat Cargo.toml | dasel -r toml 'package.name' | tr -d "'")
exec journalctl -f -n 999 --no-pager --no-hostname -u $name
