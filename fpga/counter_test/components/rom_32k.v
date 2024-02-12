module rom_32k(input [14:0] addr, output [15:0] out);
    (* rom_style = "block" *)
    reg [15:0] memory [0:32767];
    initial begin
        $readmemb("/home/joe/dev/nand2tetris-emulation/fpga/fpga/counter_test/components/rom.mem", memory);
    end
    assign out = memory[addr];
endmodule
