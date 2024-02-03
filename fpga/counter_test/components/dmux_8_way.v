module dmux_8_way (input in,
                   input [2:0] sel,
                   output [7:0] out);
    wire [7:0] sel_out;
    sel_8_way SELA (sel, sel_out);

    generate
    genvar i;
    for (i = 0; i<8; i++)
        and_gate AND_I (sel_out[i], in, out[i]);
    endgenerate
endmodule
