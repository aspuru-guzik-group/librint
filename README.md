# Librint
Librint is a library for computing molecular integrals. It is written in Rust and uses Enzyme for automatic differentiation.


## Rust compilation with Enzyme
Enzyme build:
```
$ RUSTFLAGS="-Z autodiff=Enable,LooseTypes" cargo +enzyme build --release
```
To run simple.rs:
```
./target/release/simple <file>
```
File can be any .txt file from the molecules folder example:

```
$ ./target/release/simple molecules/h2/sto3g.txt
```

## Python interface
We are using uv, to install,
```
curl -LsSf https://astral.sh/uv/install.sh | sh
```

then run
```
uv sync
```
