module or_funnel_8_way_16 (input [127:0] in,
                           output [15:0] out);
    wire [15:0] or_b_out, or_c_out, or_d_out, or_e_out, or_f_out, or_g_out;
    // top layer
    or_16 OR_A (or_b_out, or_c_out, out);
    // middle layer
    or_16 OR_B (or_d_out, or_e_out, or_b_out);
    or_16 OR_C (or_f_out, or_g_out, or_c_out);
    // bottom layer
    or_16 OR_D (in[127:112], in[111:96], or_d_out);
    or_16 OR_E (in[95:80], in[79:64], or_e_out);
    or_16 OR_F (in[63:48], in[47:32], or_f_out);
    or_16 OR_G (in[31:16], in[15:0], or_g_out);
endmodule
