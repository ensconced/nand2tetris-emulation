`timescale 1 ns/10 ps

module cpu_tb;
    reg [15:0] instruction, in_m, out_m, address_m, pc;
    reg reset, clock, memory_load;
    localparam period = 20;
    
    cpu UUT (.in_m(in_m),
             .instruction(instruction),
             .reset(reset),
             .clock(clock),
             .out_m(out_m),
             .address_m(address_m),
             .pc(pc),
             .memory_load(memory_load)
    );
    
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
        in_m = 16'b0;
        instruction = 16'b0;
        reset = 1;
        tick(clock);
        assert (pc == 16'b0) else begin
            $display("pc: ", "%b", pc);
            $fatal(1);
        end;
        // -----------------------------------------------------------------------------------
        // INCREMENTING PC
        reset = 0;
        tick(clock);
        assert (pc == 16'b1) else begin
            $display("pc: ", "%b", pc);
            $fatal(1);
        end;
        tick(clock);
        assert (pc == 16'b10) else begin
            $display("pc: ", "%b", pc);
            $fatal(1);
        end;
        // -----------------------------------------------------------------------------------
        // A + D
        // load value "1" into register A
        instruction = 16'b1;
        tick(clock);
        assert (pc == 16'b11) else begin
            $display("pc: ", "%b", pc);
            $fatal(1);
        end;
        assert (address_m == 16'b1) else begin
            $display("address_m: ", "%b", address_m);
            $fatal(1);
        end;
        // load contents of register A into register D
        instruction = 16'b1110_110000_010_000;
        tick(clock);
        assert (out_m == 16'b1) else begin
            $display("out_m: ", "%b", out_m);
            $fatal(1);
        end;
        // load value "10" into register A
        instruction = 16'b10;
        tick(clock);
        assert (address_m == 16'b10) else begin
            $display("address_m: ", "%b", address_m);
            $fatal(1);
        end;
        // add the values of registers A and D, and on next tick load result into register A
        instruction = 16'b1110_000010_100_000;
        #period
        assert (out_m == 16'b11) else begin
            $display("out_m: ", "%b", out_m);
            $fatal(1);
        end;
        tick(clock);
        assert (address_m == 16'b11) else begin
            $display("address_m: ", "%b", address_m);
            $fatal(1);
        end;
        // -----------------------------------------------------------------------------------
        // D + M
        // load value "3" into register A
        instruction = 16'b11;
        tick(clock);
        assert (address_m == 16'b11) else begin
            $display("address_m: ", "%b", address_m);
            $fatal(1);
        end;
        // load contents of register A into register D
        instruction = 16'b1110_110000_010_000;
        tick(clock);
        assert (out_m == 16'b11) else begin
            $display("out_m: ", "%b", out_m);
            $fatal(1);
        end;
        // set value "4" as in_m
        in_m = 16'b100;
        // add the values of D and in_m, and load result into memory
        instruction = 16'b1111_000010_001_000;
        #period
        assert (out_m == 16'b111) else begin
            $display("out_m: ", "%b", out_m);
            $fatal(1);
        end;
        assert (memory_load == 1) else begin
            $display("memory_load: ", "%b", memory_load);
            $fatal(1);
        end;
        // -----------------------------------------------------------------------------------
        // 10 x 10
        // load 10 into register A
        instruction = 16'd10; // note we're using decimal!
        tick(clock);
        // load contents of register A into register D
        instruction = 16'b1110_110000_010_000;
        tick(clock);
        assert (out_m == 16'd10) else begin
            $display("out_m: ", "%d", out_m);
            $fatal(1);
        end;
        // repeat 9 x (compute A + D and load result into register D)
        instruction = 16'b1110_000010_010_000;
        #period

        for (int i = 2; i <= 10; i++) begin
            assert (out_m == i * 10) else begin
                $display("i: ", i, ", out_m: ", out_m);
                $fatal(1);
            end;
            tick(clock);
        end
        // -----------------------------------------------------------------------------------
        // Jump iff output is negative
        reset = 1;
        tick(clock);
        reset = 0;
        // load address "7" into register A
        instruction = 16'b111;
        tick(clock);
        assert (pc == 16'b1) else begin
            $display("pc: ", pc);
            $fatal(1);
        end;
        //  output value "0" from alu - this should not result in a jump
        instruction = 16'b1110_101010_000_100;
        tick(clock);
        assert (pc == 16'b10) else begin
            $display("pc: ", pc);
            $fatal(1);
        end;
        //  output value "1" from alu - this should not result in a jump
        instruction = 16'b1110_111111_000_100;
        tick(clock);
        assert (pc == 16'b11) else begin
            $display("pc: ", pc);
            $fatal(1);
        end;
        //  output value "-1" from alu - this should result in a jump
        instruction = 16'b1110_111010_000_100;
        tick(clock);
        assert (pc == 16'b111) else begin
            $display("pc: ", pc);
            $fatal(1);
        end;
        // -----------------------------------------------------------------------------------
        // Jump iff output is zero
        reset = 1;
        tick(clock);
        reset = 0;
        // load address "7" into register A
        instruction = 16'b111;
        tick(clock);
        assert (pc == 16'b1) else begin
            $display("pc: ", pc);
            $fatal(1);
        end;
        //  output value "-1" from alu - this should not result in a jump
        instruction = 16'b1110_111010_000_010;
        tick(clock);
        assert (pc == 16'b10) else begin
            $display("pc: ", pc);
            $fatal(1);
        end;
        //  output value "1" from alu - this should not result in a jump
        instruction = 16'b1110_111111_000_010;
        tick(clock);
        assert (pc == 16'b11) else begin
            $display("pc: ", pc);
            $fatal(1);
        end;
        //  output value "0" from alu - this should result in a jump
        instruction = 16'b1110_101010_000_010;
        tick(clock);
        assert (pc == 16'b111) else begin
            $display("pc: ", pc);
            $fatal(1);
        end;
        // -----------------------------------------------------------------------------------
        // Jump iff output is positive
        reset = 1;
        tick(clock);
        reset = 0;
        // load address "7" into register A
        instruction = 16'b111;
        tick(clock);
        assert (pc == 16'b1) else begin
            $display("pc: ", pc);
            $fatal(1);
        end;
        //  output value "-1" from alu - this should not result in a jump
        instruction = 16'b1110_111010_000_001;
        tick(clock);
        assert (pc == 16'b10) else begin
            $display("pc: ", pc);
            $fatal(1);
        end;
        //  output value "0" from alu - this should not result in a jump
        instruction = 16'b1110_101010_000_001;
        tick(clock);
        assert (pc == 16'b11) else begin
            $display("pc: ", pc);
            $fatal(1);
        end;
        //  output value "1" from alu - this should result in a jump
        instruction = 16'b1110_111111_000_001;
        tick(clock);
        assert (pc == 16'b111) else begin
            $display("pc: ", pc);
            $fatal(1);
        end;
    end
endmodule
