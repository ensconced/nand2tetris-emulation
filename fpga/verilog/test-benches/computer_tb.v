`timescale 1 ns/10 ps

module computer_tb;
    reg reset, clock;
    reg [15:0] led_output;
    localparam period = 20;
    computer UUT (.reset(reset), .clock(clock), .led_output(led_output));
    
    task tick;
    begin
        #period;
        clock = 0;
        #period;
        clock = 1;
        #period;
    end
    endtask

    initial
    begin
        // -----------------------------------------------------------------------------------
        // SETUP
        reset = 1;
        tick(clock);
        reset = 0;
        for (int i = 0; i < 1024; i++) begin
            tick(clock);
        end;
        // No assertions here but this testbench should be useful for viewing in gtkwave.
    end
endmodule
