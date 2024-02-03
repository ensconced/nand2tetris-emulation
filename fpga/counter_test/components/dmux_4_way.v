module dmux_4_way (input in,
                   input [1:0] sel,
                   output [3:0] out);
    wire [3:0] sel_out;
    sel_4_way SELA (sel, sel_out);

    generate
    genvar i;
    for (i = 0; i<4; i++)
        and_gate AND_I (sel_out[i], in, out[i]);
    endgenerate
endmodule
