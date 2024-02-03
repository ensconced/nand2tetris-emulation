module computer (input reset,
                 input clock,
                 output [15:0] led_output);
    wire memory_load;
    wire [15:0] word_from_ram, instruction, word_to_ram, address_m, pc;
    rom_32k ROM_A (pc[14:0], instruction);
    cpu CPU_A (word_from_ram, instruction, reset, clock, word_to_ram, address_m, pc, memory_load);
    memory MEMORY_A (word_to_ram, address_m[14:0], memory_load, clock, word_from_ram, led_output);
endmodule
