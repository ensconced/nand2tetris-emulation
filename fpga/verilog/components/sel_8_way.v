module sel_8_way (input [2:0] sel,
                  output [7:0] out);
    wire not_sel_0, not_sel_1, not_sel_2;
    not_gate NOTA (sel[0], not_sel_0);
    not_gate NOTB (sel[1], not_sel_1);
    not_gate NOTC (sel[2], not_sel_2);
    and_3_way ANDA ({ sel[2], sel[1], sel[0] }, out[7]);
    and_3_way ANDB ({ sel[2], sel[1], not_sel_0 }, out[6]);
    and_3_way ANDC ({ sel[2], not_sel_1, sel[0] }, out[5]);
    and_3_way ANDD ({ sel[2], not_sel_1, not_sel_0 }, out[4]);
    and_3_way ANDE ({ not_sel_2, sel[1], sel[0] }, out[3]);
    and_3_way ANDF ({ not_sel_2, sel[1], not_sel_0 }, out[2]);
    and_3_way ANDG ({ not_sel_2, not_sel_1, sel[0] }, out[1]);
    and_3_way ANDH ({ not_sel_2, not_sel_1, not_sel_0 }, out[0]);
endmodule
