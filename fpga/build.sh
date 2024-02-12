# #!/usr/bin/env bash

# set -eux pipefile

# /media/ssd/Xilinx/Vivado/2023.2/bin/vivado -mode batch -nojournal -nolog -source build.tcl

# # cd fpga

# # node ./verilog/romgen.js blinky >counter_test/components/rom.mem

# # ICARUS_OUT_FILENAME="icarus.out"
# # ALL_VERILOG_FILES=$(find counter_test/components -name '*.v')
# # iverilog -v -gassertions -g2012 -o "$ICARUS_OUT_FILENAME" $ALL_VERILOG_FILES
# # vvp "$ICARUS_OUT_FILENAME" -v -vcd
# # echo "tests passed!"

# # cd counter_test
# # yosys -s yosys-cmds.ys
# # tcl $(python3 -m f4pga.wrappers.tcl "${FAMILY}")"

# # make -C counter_test
# # make download -C counter_test
