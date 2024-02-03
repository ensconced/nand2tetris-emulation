`timescale 1 ns/10 ps

module or_gate_tb;
    reg in_a, in_b;
    localparam period = 20;
    or_gate UUT (.in_a(in_a), .in_b(in_b), .out(out));
    
    initial
    begin
        in_a = 0;
        in_b = 0;
        #period;

        assert(out == 0) else begin
            $display("output should be off when both inputs are off");
            $fatal(1);
        end;

        in_a = 0;
        in_b = 1;
        #period;
        
        assert(out == 1) else begin
            $display("output should be on when in_a is off and in_b is on");
            $fatal(1);
        end;

        in_a = 1;
        in_b = 0;
        #period;
        
        assert(out == 1) else begin
            $display("output should be on when in_a is on and in_b is off");
            $fatal(1);
        end;

        in_a = 1;
        in_b = 1;
        #period;

        assert(out == 1) else begin
            $display("output should be on when both inputs are on");
            $fatal(1);
        end;
    end
endmodule
