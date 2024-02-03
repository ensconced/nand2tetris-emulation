module and_3_way (input [2:0] in,
                  output out);
    wire and_a_out;
    and_gate ANDA (in[2], in[1], and_a_out);
    and_gate ANDB (and_a_out, in[0], out);
endmodule
