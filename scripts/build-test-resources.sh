#!/usr/bin/env bash

cur_dir=$(pwd)

for path in "$cur_dir"/test_resources/profiling/*; do
    echo "$path"
    cd "$path" || exit

    cargo component clean
    cargo component build &
done

wait

cd "$cur_dir" || exit
