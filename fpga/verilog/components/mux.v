module mux (input in_a,
            input in_b,
            input sel,
            output out);
    wire anda_out, andb_out, not_out;
    and_gate ANDA (in_a, not_out, anda_out);
    and_gate ANDB (in_b, sel, andb_out);
    not_gate NOTA (sel, not_out);
    or_gate ORA (anda_out, andb_out, out);
endmodule
