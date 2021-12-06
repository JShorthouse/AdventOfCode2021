#!/bin/bash
set -e

cargo build --release 2>/dev/null

pushd target/release > /dev/null
files=(day[0-9][0-9])
popd > /dev/null

# Warmup
for i in {1..10}; do
    for f in ${files[@]}; do
        target/release/${f} > /dev/null
    done
done

# Bench
export TIMEFORMAT='%3Rs'
for f in ${files[@]}; do
    echo -n "${f}: "
    time target/release/${f} > /dev/null
done
unset TIMEFORMAT
