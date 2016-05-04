#!/bin/sh
multirust run nightly cargo bench --features benchmarks -- --test --nocapture
