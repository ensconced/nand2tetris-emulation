`timescale 1 ns/10 ps

module reg_loader_tb;
    reg [15:0] alu_out, instruction, reg_a_in, reg_d_in, memory_in;
    reg reg_a_load, reg_d_load, memory_load;

    localparam period = 20;
    reg_loader UUT (
        .alu_out(alu_out), 
        .instruction(instruction), 
        .reg_a_in(reg_a_in), 
        .reg_d_in(reg_d_in), 
        .memory_in(memory_in),
        .reg_a_load(reg_a_load), 
        .reg_d_load(reg_d_load),
        .memory_load(memory_load)
    );
    
    initial
    begin
        // initialise
        alu_out = 16'd12345;

        // an A instruction gets loaded into the A register
        instruction = 16'b0101_0101_0101_0101;
        #period
        assert({ reg_a_load, reg_d_load, memory_load } == 3'b100) else begin
            $display("reg_a_load: ", reg_a_load, ", reg_d_load: ", reg_d_load, ", memory_load: ", memory_load);
            $fatal(1);
        end;
        assert(reg_a_in == instruction) else begin
            $display("reg_a_in: ", "%b", reg_a_in);
            $fatal(1);
        end;

        // C instructions
        // null instruction doesn't do anything
        instruction = 16'b1110_000000_000_000;
        #period
        assert({ reg_a_load, reg_d_load, memory_load } == 3'b000) else begin
            $display("reg_a_load: ", reg_a_load, ", reg_d_load: ", reg_d_load, ", memory_load: ", memory_load);
            $fatal(1);
        end;

        // setting d3 loads into memory
        instruction = 16'b1110_000000_001_000;
        #period
        assert({ reg_a_load, reg_d_load, memory_load } == 3'b001) else begin
            $display("reg_a_load: ", reg_a_load, ", reg_d_load: ", reg_d_load, ", memory_load: ", memory_load);
            $fatal(1);
        end;
        assert(memory_in == alu_out) else begin
            $display("memory_in: ", "%b", memory_in);
            $fatal(1);
        end;
        
        // setting d2 loads into reg D
        instruction = 16'b1110_000000_010_000;
        #period
        assert({ reg_a_load, reg_d_load, memory_load } == 3'b010) else begin
            $display("reg_a_load: ", reg_a_load, ", reg_d_load: ", reg_d_load, ", memory_load: ", memory_load);
            $fatal(1);
        end;
        assert(reg_d_in == alu_out) else begin
            $display("reg_d_in: ", "%b", reg_d_in);
            $fatal(1);
        end;


        // setting d2 and d3 loads into reg D and memory
        instruction = 16'b1110_000000_011_000;
        #period
        assert({ reg_a_load, reg_d_load, memory_load } == 3'b011) else begin
            $display("reg_a_load: ", reg_a_load, ", reg_d_load: ", reg_d_load, ", memory_load: ", memory_load);
            $fatal(1);
        end;
        assert(reg_d_in == alu_out) else begin
            $display("reg_d_in: ", "%b", reg_d_in);
            $fatal(1);
        end;
        assert(memory_in == alu_out) else begin
            $display("memory_in: ", "%b", memory_in);
            $fatal(1);
        end;

        // setting d1 loads into reg A
        instruction = 16'b1110_000000_100_000;
        #period
        assert({ reg_a_load, reg_d_load, memory_load } == 3'b100) else begin
            $display("reg_a_load: ", reg_a_load, ", reg_d_load: ", reg_d_load, ", memory_load: ", memory_load);
            $fatal(1);
        end;
        assert(reg_a_in == alu_out) else begin
            $display("reg_a_in: ", "%b", reg_a_in);
            $fatal(1);
        end;


        // setting d1 and d3 loads into reg A and memory
        instruction = 16'b1110_000000_101_000;
        #period
        assert({ reg_a_load, reg_d_load, memory_load } == 3'b101) else begin
            $display("reg_a_load: ", reg_a_load, ", reg_d_load: ", reg_d_load, ", memory_load: ", memory_load);
            $fatal(1);
        end;
        assert(reg_a_in == alu_out) else begin
            $display("reg_a_in: ", "%b", reg_a_in);
            $fatal(1);
        end;
        assert(memory_in == alu_out) else begin
            $display("memory_in: ", "%b", memory_in);
            $fatal(1);
        end;

        // setting d1 and d2 loads into reg A and reg D
        instruction = 16'b1110_000000_110_000;
        #period
        assert({ reg_a_load, reg_d_load, memory_load } == 3'b110) else begin
            $display("reg_a_load: ", reg_a_load, ", reg_d_load: ", reg_d_load, ", memory_load: ", memory_load);
            $fatal(1);
        end;
        assert(reg_a_in == alu_out) else begin
            $display("reg_a_in: ", "%b", reg_a_in);
            $fatal(1);
        end;
        assert(reg_d_in == alu_out) else begin
            $display("reg_d_in: ", "%b", reg_d_in);
            $fatal(1);
        end;

        // setting d1, d2 and d3 loads into reg A, reg D, and memory
        instruction = 16'b1110_000000_111_000;
        #period
        assert({ reg_a_load, reg_d_load, memory_load } == 3'b111) else begin
            $display("reg_a_load: ", reg_a_load, ", reg_d_load: ", reg_d_load, ", memory_load: ", memory_load);
            $fatal(1);
        end;
        assert(reg_a_in == alu_out) else begin
            $display("reg_a_in: ", "%b", reg_a_in);
            $fatal(1);
        end;
        assert(reg_d_in == alu_out) else begin
            $display("reg_d_in: ", "%b", reg_d_in);
            $fatal(1);
        end;
        assert(memory_in == alu_out) else begin
            $display("memory_in: ", "%b", memory_in);
            $fatal(1);
        end;
    end
endmodule
