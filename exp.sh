#!/bin/bash

types=("none" "cache" "opt" "all")

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

mkdir -p exp

for type in "${types[@]}"; do
    for file in "${files[@]}"; do
        mol=$(dirname "$file")
        bas=$(basename "$file")
        output_file="exp/${mol}_${bas}_${type}.txt"

        echo "Running $type on $file -> $output_file"
        # warmup
        ./target/release/$type "/h/332/jpmedina/librint/molecules/$file.txt"

        # run
        ./target/release/$type "/h/332/jpmedina/librint/molecules/$file.txt" > "$output_file"
    done
done
