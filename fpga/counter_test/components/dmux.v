module dmux (input in,
             input sel,
             output out_a,
             output out_b);
    wire not_out;
    and_gate ANDA (in, not_out, out_a);
    and_gate ANDB (in, sel, out_b);
    not_gate NOTA (sel, not_out);
endmodule
