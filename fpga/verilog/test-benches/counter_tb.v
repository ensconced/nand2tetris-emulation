`timescale 1 ns/10 ps

module counter_tb;
    reg [15:0] in, out, expected_out;
    reg inc, load, reset, clock;
    localparam period = 20;
    
    counter UUT (.in(in), .inc(inc), .load(load), .reset(reset), .clock(clock), .out(out));
    
    initial
    begin
        // load an initial value
        in = 16'b0;
        inc = 0;
        load = 1;
        reset = 0;
        clock = 0;
        #period
        clock = 1;
        #period

        expected_out = 16'b0;
        assert(out == expected_out) else begin
            $display("out, ", out, ", expected out: ", expected_out);
            $fatal(1);
        end;
        
        // increment the value
        load = 0;
        inc = 1;
        clock = 0;
        #period
        clock = 1;
        #period
        expected_out = 16'b1;
        assert(out == expected_out) else begin
            $display("out, ", out, ", expected out: ", expected_out);
            $fatal(1);
        end;

        // increment again
        clock = 0;
        #period
        clock = 1;
        #period
        expected_out = 16'b10;
        assert(out == expected_out) else begin
            $display("out, ", out, ", expected out: ", expected_out);
            $fatal(1);
        end;

        // reset
        reset = 1;
        clock = 0;
        #period
        clock = 1;
        #period
        expected_out = 16'b0;
        assert(out == expected_out) else begin
            $display("out, ", out, ", expected out: ", expected_out);
            $fatal(1);
        end;

        // load new value
        in = 123;
        reset = 0;
        load = 1;
        clock = 0;
        #period
        clock = 1;
        #period
        expected_out = 16'd123;
        assert(out == expected_out) else begin
            $display("out, ", out, ", expected out: ", expected_out);
            $fatal(1);
        end;

        // increment again
        load = 0;
        clock = 0;
        #period
        clock = 1;
        #period
        expected_out = 16'd124;
        assert(out == expected_out) else begin
            $display("out, ", out, ", expected out: ", expected_out);
            $fatal(1);
        end;
    end
endmodule
