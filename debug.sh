#!/bin/bash

cargo +nightly build --profile release-lto -Z unstable-options
cargo +nightly run --profile release-lto -Z unstable-options
