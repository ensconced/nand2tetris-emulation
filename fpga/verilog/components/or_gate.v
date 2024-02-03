module or_gate (input in_a,
                input in_b,
                output out);
    wire not_a, not_b;
    not_gate NOTA (in_a, not_a);
    not_gate NOTB (in_b, not_b);
    nand_gate NANDA (not_a, not_b, out);
endmodule
