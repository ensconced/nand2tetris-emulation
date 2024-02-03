module add_16 (input [15:0] in_a,
               input [15:0] in_b,
               output [15:0] out);
    wire [15:0] adder_out;

    half_adder HALFADDERA ({ in_a[0], in_b[0] }, { adder_out[0], out[0] });

    generate
    genvar i;
    for (i = 1; i<16; i++)
        full_adder FULLADDERI ({ in_a[i], in_b[i], adder_out[i - 1] }, { adder_out[i], out[i] });
        endgenerate
endmodule
