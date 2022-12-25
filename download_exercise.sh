#!/bin/sh

exercism download --exercise=${1} --track=rust
dasel put string -f Cargo.toml  "workspace.members.[]" ${1}
pandoc -f markdown -t org "./${1}/README.md" -o "./${1}/README.org"
# rm "./${1}/README.md"
