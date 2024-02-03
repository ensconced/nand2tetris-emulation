module or_3_way (input [2:0] in,
                 output out);
    wire or_b_out;
    or_gate OR_A (or_b_out, in[2], out);
    or_gate OR_B (in[1], in[0], or_b_out);
endmodule
