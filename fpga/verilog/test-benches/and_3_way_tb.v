`timescale 1 ns/10 ps

module and_3_way_tb;
    reg [2:0] in;
    reg out, expected_out;
    localparam period = 20;
    and_3_way UUT (.in(in), .out(out));
    
    initial
    begin
        for (int i=0; i<8; i=i+1) begin
            in = i;
            #period;
  
            expected_out = (in == 3'b111) ? 1 : 0;
            assert(out == expected_out) else begin
                $display("in: ", "%b", in, ", out: ", "%b", out, ", expected out: ", "%b", expected_out);
                $fatal(1);
            end;
        end
    end
endmodule
