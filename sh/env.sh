#!/usr/bin/env bash

export RUSTFLAGS='--cfg reqwest_unstable'
export RUST_LOG=debug,supervisor=warn,hyper=warn,rustls=warn,h2=warn,tower=warn
