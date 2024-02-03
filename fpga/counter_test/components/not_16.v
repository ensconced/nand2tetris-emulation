module not_16 (input [15:0] in,
               output [15:0] out);
    generate
    genvar i;
    for (i = 0; i<16; i++)
        not_gate NOT_I (in[i], out[i]);
    endgenerate
endmodule
