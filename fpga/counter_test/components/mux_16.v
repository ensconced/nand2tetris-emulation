module mux_16 (input [15:0] in_a,
               input [15:0] in_b,
               input sel,
               output [15:0] out);
generate
genvar i;
for (i = 0; i<16; i++)
    mux MUX_I (in_a[i], in_b[i], sel, out[i]);
    endgenerate
endmodule
