#!/bin/bash

/u/jpmedina/rust-latest2/build/x86_64-unknown-linux-gnu/llvm/bin/opt $1 \
    -load-pass-plugin=/u/jpmedina/rust-latest2/build/x86_64-unknown-linux-gnu/enzyme/build/Enzyme/LLVMEnzyme-21.so \
    -passes="enzyme" \
    -enzyme-strict-aliasing=0 \
    -S \
    |& grep "LLVM ERROR: function failed verification (4)"
