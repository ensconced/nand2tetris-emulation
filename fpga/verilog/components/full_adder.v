module full_adder (input [2:0] in,
                   output [1:0] out);
    wire a_sum, a_carry, b_carry;
    half_adder HALFADDERA (in[1:0], { a_carry, a_sum });
    half_adder HALFADDERB ({ in[2], a_sum }, { b_carry, out[0] });
    or_gate ORA (b_carry, a_carry, out[1]);
endmodule
