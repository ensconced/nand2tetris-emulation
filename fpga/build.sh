#!/usr/bin/env bash

set -eux pipefile

cd fpga

node ./verilog/romgen.js blinky >counter_test/components/rom.mem

ICARUS_OUT_FILENAME="icarus.out"
ALL_VERILOG_FILES=$(find counter_test/components -name '*.v')
iverilog -v -gassertions -g2012 -o "$ICARUS_OUT_FILENAME" $ALL_VERILOG_FILES
vvp "$ICARUS_OUT_FILENAME" -v -vcd
echo "tests passed!"

make -C counter_test
make download -C counter_test
