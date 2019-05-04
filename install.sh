#!/bin/bash -eu

exec 9>&1
script_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

cargo_out=$(
    cargo install --force --color always --path "$script_dir" 2>&1 | tee >(cat - >&9)
    exit ${PIPESTATUS[0]}
)

installed=`echo "$cargo_out" | tail -1 | sed 's/[^/]*//'`
strip --strip-all "$installed"
echo "installed and stripped: $installed"
