module xor_gate (input in_a,
                 input in_b,
                 output out);
    wire a_out, b_out, c_out;
    nand_gate NANDA (in_a, in_b, a_out);
    nand_gate NANDB (in_a, a_out, b_out);
    nand_gate NANDC (in_b, a_out, c_out);
    nand_gate NANDD (b_out, c_out, out);
endmodule
