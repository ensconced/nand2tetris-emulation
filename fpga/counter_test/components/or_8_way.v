module or_8_way (input [7:0] in,
                 output out);
    wire or_b_out, or_c_out, or_d_out, or_e_out, or_f_out, or_g_out;
    // top layer
    or_gate OR_A (or_b_out, or_c_out, out);
    // middle layer
    or_gate OR_B (or_d_out, or_e_out, or_b_out);
    or_gate OR_C (or_f_out, or_g_out, or_c_out);
    // bottom layer
    or_gate OR_D (in[7], in[6], or_d_out);
    or_gate OR_E (in[5], in[4], or_e_out);
    or_gate OR_F (in[3], in[2], or_f_out);
    or_gate OR_G (in[1], in[0], or_g_out);
endmodule
