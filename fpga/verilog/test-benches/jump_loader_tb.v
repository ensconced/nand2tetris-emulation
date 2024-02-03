`timescale 1 ns/10 ps

module jump_loader_tb;
    reg j1, j2, j3, is_zero, is_negative, is_positive, is_c_instruction, should_jump, expected_out;
    localparam period = 20;
    jump_loader UUT (
        .j1(j1), 
        .j2(j2), 
        .j3(j3), 
        .is_zero(is_zero), 
        .is_negative(is_negative),
        .is_c_instruction(is_c_instruction), 
        .should_jump(should_jump)
    );
    
    initial
    begin
        for (int i = 0; i < 64; i++) begin
            { j1, j2, j3, is_zero, is_negative, is_c_instruction } = i;
            #period
            is_positive = ~(is_negative | is_zero);
            expected_out = is_c_instruction & ((j1 & is_negative) | (j2 & is_zero) | (j3 & is_positive));
            assert(should_jump == expected_out) else begin
                $display("should_jump: ", should_jump, ", expected: ", expected_out);
                $fatal(1);
            end;
        end
    end
endmodule
