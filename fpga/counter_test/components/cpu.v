module cpu (input [15:0] in_m,
            input [15:0] instruction,
            input reset,
            input clock,
            output [15:0] out_m,
            output [15:0] address_m,
            output [15:0] pc,
            output memory_load);
    wire [15:0] alu_out, not_alu_out, reg_a_in, reg_d_in, in_x, in_y, reg_a_out;
    wire reg_a_load, reg_d_load, alu_out_is_zero, jump_loader_out;
    reg_loader REG_LOADER_A (alu_out, instruction, reg_a_in, reg_d_in, out_m, reg_a_load, reg_d_load, memory_load);
    alu ALU_A (in_x, in_y, instruction[11], instruction[10], instruction[9], instruction[8], instruction[7], instruction[6], alu_out, not_alu_out, alu_out_is_zero);
    reg_16 REG_A (reg_a_in, reg_a_load, clock, reg_a_out);
    reg_16 REG_D (reg_d_in, reg_d_load, clock, in_x);
    mux_16 MUX_A (reg_a_out, in_m, instruction[12], in_y);
    jump_loader JUMP_LOADER_A (instruction[2], instruction[1], instruction[0], alu_out_is_zero, alu_out[15], instruction[15], jump_loader_out);
    counter COUNTER_A (reg_a_out, 1'b1, jump_loader_out, reset, clock, pc);
    assign address_m = reg_a_out;
endmodule
