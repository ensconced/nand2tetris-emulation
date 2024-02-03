module reg_loader (input [15:0] alu_out,
                   input [15:0] instruction,
                   output [15:0] reg_a_in,
                   output [15:0] reg_d_in,
                   output [15:0] memory_in,
                   output reg_a_load,
                   output reg_d_load,
                   output memory_load);
    wire alu_out_into_reg_a, instruction_into_reg_a;
    and_gate ANDA(instruction[5], instruction[15], alu_out_into_reg_a);
    and_gate ANDB(instruction[4], instruction[15], reg_d_load);
    and_gate ANDC(instruction[3], instruction[15], memory_load);
    not_gate NOTA (instruction[15], instruction_into_reg_a);
    or_gate ORA (alu_out_into_reg_a, instruction_into_reg_a, reg_a_load);
    mux_16 MUXA (instruction, alu_out, alu_out_into_reg_a, reg_a_in);
    assign reg_d_in = alu_out;
    assign memory_in = alu_out;
endmodule
