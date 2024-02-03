module counter (input [15:0] in,
                input inc,
                input load,
                input reset,
                input clock,
                output [15:0] out);
    wire [15:0] load_val, inc_val, reg_in, reg_out, inc_out;
    wire reg_load;
    mux_16 MUXA (reg_out, inc_out, inc, inc_val);
    mux_16 MUXB (inc_val, in, load, load_val);
    mux_16 MUXC (load_val, 16'b0, reset, reg_in);
    reg_16 REGA (reg_in, reg_load, clock, reg_out);
    or_3_way ORA ({ inc, load, reset }, reg_load);
    inc INCA (reg_out, inc_out);
    assign out = reg_out;
endmodule
