#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

name=$(cat Cargo.toml | dasel -r toml 'package.name' | tr -d "'")
exe=target/debug/$name
rm -rf $exe
cargo build -p $name
if [ -f "$exe" ]; then
  GREEN='\033[0;92m'
  NC='\033[0m'
  pkill -9 $name || true
  echo -e "\n${GREEN}❯ $exe$NC\n"
  exec $exe
fi
