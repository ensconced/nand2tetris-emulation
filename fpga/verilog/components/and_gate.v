module and_gate (input in_a,
                input in_b,
                output out);
    wire nand_tmp;
    nand_gate NANDA (in_a, in_b, nand_tmp);
    nand_gate NANDB (nand_tmp, nand_tmp, out);
endmodule
