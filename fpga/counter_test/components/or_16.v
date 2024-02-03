module or_16 (input [15:0] in_a,
              input [15:0] in_b,
              output [15:0] out);
generate
genvar i;
for (i = 0; i<16; i++)
    or_gate OR_I (in_a[i], in_b[i], out[i]);
    endgenerate
endmodule
