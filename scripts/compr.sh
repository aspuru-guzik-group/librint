#!/bin/bash

files=(
    "h2/sto3g"
    "h4/sto3g"
    "lih/sto3g"
    "hf/sto3g"
    "h2o/sto3g"
    "nh3/sto3g"
    "ch4/sto3g"
    "h2o/def2svp"
    "c6h6/sto3g"
    "c6h6/631g"
)

mkdir -p comp

for file in "${files[@]}"; do
    mol=$(dirname "$file")
    bas=$(basename "$file")
    output_file="comp/${mol}_${bas}_r.txt"

    echo "Running $file -> $output_file"
    # warmup
    ./target/release/single "/h/332/jpmedina/librint/molecules/$file.txt"

    # run
    ./target/release/single "/h/332/jpmedina/librint/molecules/$file.txt" > "$output_file"
done
