module is_non_zero (input [15:0] in,
                    output out);
    wire or_a_out, or_b_out;
    or_8_way OR_A (in[15:8], or_a_out);
    or_8_way OR_B (in[7:0], or_b_out);
    or_gate OR_C (or_a_out, or_b_out, out);
endmodule
