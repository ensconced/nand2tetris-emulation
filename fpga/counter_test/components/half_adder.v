module half_adder (input [1:0] in,
                   output [1:0] out);
    xor_gate XORA (in[1], in[0], out[0]);
    and_gate ANDA (in[1], in[0], out[1]);
endmodule
