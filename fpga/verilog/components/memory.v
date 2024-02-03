module memory (input [15:0] in,
               input [14:0] address,
               input load,
               input clock,
               output [15:0] out,
               output [15:0] led_output);
    wire load_ram, load_led_reg;
    wire [15:0] ram_out, led_reg_out;
    dmux DMUX_A (load, address[14], load_ram, load_led_reg);
    mux_16 MUX_A (ram_out, led_reg_out, address[14], out);
    ram_16k RAM_A (in, address[13:0], load_ram, clock, ram_out);
    reg_16 REG_A (in, load_led_reg, clock, led_reg_out);
    assign led_output = led_reg_out; 
endmodule
