module inc (input [15:0] in,
            output [15:0] out);
    add_16 ADDER (in, 16'b1, out);
endmodule
