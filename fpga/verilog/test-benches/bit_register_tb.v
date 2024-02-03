`timescale 1 ns/10 ps

module bit_register_tb;
    reg in, out, load, clock;
    bit_register UUT (.in(in), .load(load), .clock(clock), .out(out));
    initial
    begin
        // load a value in to the register
        clock = 0;
        load = 1;
        in = 1;
        #10
        clock = 1;
        #10
        assert(out == 1) else begin
            $display("out: ", "%b", out);
            $fatal(1);
        end;
        
        // if load is zero, the output doesn't change even when the input does
        load = 0;
        in = 0;
        clock = 0;
        #10
        clock = 1;
        #10
        assert(out == 1) else begin
            $display("out: ", "%b", out);
            $fatal(1);
        end;

        // but when we set load back to one, the output will change...
        load = 1;
        clock = 0;
        #10
        clock = 1;
        #10
        assert(out == 0) else begin
            $display("out: ", "%b", out);
            $fatal(1);
        end;
    end
endmodule
