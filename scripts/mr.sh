#!/bin/bash

OPT=/u/jpmedina/rust-latest2/build/x86_64-unknown-linux-gnu/llvm/bin/opt
SO=/u/jpmedina/rust-latest2/build/x86_64-unknown-linux-gnu/enzyme/build/Enzyme/LLVMEnzyme-21.so

# RUSTFLAGS="-Z autodiff=Enable,LooseTypes,PrintModBefore" cargo +enzymel build --release --bin overhead &> out.ll

$OPT out.ll -load-pass-plugin=$SO -passes="enzyme" -enzyme-strict-aliasing=0 -S

/u/jpmedina/rust-latest2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-extract -s --func=<> --rfunc="enzyme_autodiff*" --rfunc="enzyme_fwddiff*" --rfunc=<fnc_called_by_enzyme> out.ll -o mwe.ll


# extract -S   --func="__enzyme_fwddiff_ZN8overhead9dovlppfor17h6567dfe19281e937E"   --recursive  --rfunc="enzyme_fwddiff*"   --rfunc="ovlpp"   out.ll -o mwe.ll

_ZN8overhead4main17h25736e915a18b782E

/u/jpmedina/rust-latest2/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-extract -S   --func="_ZN8overhead4main17h25736e915a18b782E"   --recursive  --rfunc="enzyme_fwddiff*"   --rfunc="ovlpp"   out.ll -o mwe.ll
