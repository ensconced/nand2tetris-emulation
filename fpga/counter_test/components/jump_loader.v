module jump_loader (input j1,
                    input j2,
                    input j3,
                    input is_zero,
                    input is_negative,
                    input is_c_instruction,
                    output should_jump);
wire is_not_zero, is_not_negative, is_positive, jump1, jump2, jump3, some_jump;
    not_gate NOT_A (is_zero, is_not_zero);
    not_gate NOT_B (is_negative, is_not_negative);
    and_gate AND_A (is_not_zero, is_not_negative, is_positive);
    and_gate AND_B (is_negative, j1, jump1);
    and_gate AND_C (is_zero, j2, jump2);
    and_gate AND_D (is_positive, j3, jump3);
    or_3_way OR_3 ({ jump1, jump2, jump3 }, some_jump);
    and_gate AND_E (some_jump, is_c_instruction, should_jump);
endmodule
