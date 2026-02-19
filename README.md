### Librint

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
