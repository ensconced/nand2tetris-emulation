module mux_8_way_16 (input [127:0] in,
                     input [2:0] sel,
                     output [15:0] out);
    wire [7:0] sel_8_way_out;
    wire [127:0] muxes_out;
    sel_8_way SEL_A (sel, sel_8_way_out);
    or_funnel_8_way_16 OR_FUNNEL (muxes_out, out);
    generate
    genvar i;
    for (i = 0; i<8; i++)
        mux_16 MUX_I (16'b0, in[16 * i+:16], sel_8_way_out[i], muxes_out[16 * i+:16]);
        endgenerate
endmodule
