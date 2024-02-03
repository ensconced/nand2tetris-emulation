module reg_16 (input [15:0] in,
               input load,
               input clock,
               output [15:0] out);
generate
genvar i;
for (i = 0; i<16; i++)
    bit_register BIT_I (in[i], load, clock, out[i]);
    endgenerate
endmodule
