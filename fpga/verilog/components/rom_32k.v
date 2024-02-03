module rom_32k(input [14:0] addr, output [15:0] out);
    reg [15:0] memory [0:32767];
    initial begin
        $readmemb(`ROM_FILE, memory);
    end
    assign out = memory[addr];
endmodule
