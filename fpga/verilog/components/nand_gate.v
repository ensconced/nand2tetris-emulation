module nand_gate (input in_a,
                  input in_b,
                  output out);
    wire nand_tmp;
    assign out = in_a ~& in_b;
endmodule
