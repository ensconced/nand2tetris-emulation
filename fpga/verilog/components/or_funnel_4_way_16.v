module or_funnel_4_way_16 (input [63:0] in,
                           output [15:0] out);
    wire [15:0] or_b_out, or_c_out;
    // top layer
    or_16 OR_A (or_b_out, or_c_out, out);
    // bottom layer
    or_16 OR_B (in[63:48], in[47:32], or_b_out);
    or_16 OR_C (in[31:16], in[15:0], or_c_out);
endmodule
