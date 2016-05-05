#!/bin/sh
rm -rf .criterion
multirust run nightly cargo bench --features benchmarks -- --test --nocapture
qlmanage -t -s 1000 -o /tmp .criterion/Rng/summary/new/violin_plot.svg
pngcrush /tmp/violin_plot.svg.png results.png
