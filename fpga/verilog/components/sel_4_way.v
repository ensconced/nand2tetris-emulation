module sel_4_way (input [1:0] sel,
                  output [3:0] out);
    wire not_sel_0, not_sel_1;
    not_gate NOTA (sel[0], not_sel_0);
    not_gate NOTB (sel[1], not_sel_1);
    and_gate ANDA (sel[1], sel[0], out[3]);
    and_gate ANDB (sel[1], not_sel_0, out[2]);
    and_gate ANDC (not_sel_1, sel[0], out[1]);
    and_gate ANDD (not_sel_1, not_sel_0, out[0]);
endmodule
