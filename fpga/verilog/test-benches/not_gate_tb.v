`timescale 1 ns/10 ps

module not_gate_tb;
    reg in;
    localparam period = 20;
    not_gate UUT (.in(in), .out(out));
    
    initial
    begin
        in = 0;
        #period;

        assert(out == 1) else begin
            $display("output should be on when input is off");
            $fatal(1);
        end;

        in = 1;
        #period;
        
        assert(out == 0) else begin
            $display("output should be off when input is on");
            $fatal(1);
        end;
    end
endmodule
