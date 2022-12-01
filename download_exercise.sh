#!/bin/sh

exercism download --exercise=${1} --track=rust
dasel put string -f Cargo.toml  "workspace.members.[]" ${1}
pgrep -f clion && clion ${1}/**/*.rs
cd ${1}
