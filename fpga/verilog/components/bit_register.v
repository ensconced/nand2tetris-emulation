module bit_register(input in,
                    input load,
                    input clock,
                    output out);
    wire flip_flop_out, mux_out;
    mux MUXA (flip_flop_out, in, load, mux_out);
    flip_flop FF (mux_out, clock, flip_flop_out);
    assign out = flip_flop_out;
endmodule
