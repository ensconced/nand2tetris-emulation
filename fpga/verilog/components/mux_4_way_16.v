module mux_4_way_16 (input [63:0] in,
                     input [1:0] sel,
                     output [15:0] out);
    wire [3:0] sel_4_way_out;
    wire [63:0] muxes_out;
    sel_4_way SEL_A (sel, sel_4_way_out);
    or_funnel_4_way_16 OR_FUNNEL (muxes_out, out);
    generate
    genvar i;
    for (i = 0; i<4; i++)
        mux_16 MUX_I (16'b0, in[16 * i+:16], sel_4_way_out[i], muxes_out[16 * i+:16]);
        endgenerate
endmodule
