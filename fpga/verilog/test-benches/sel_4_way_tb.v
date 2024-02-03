`timescale 1 ns/10 ps

module sel_4_way_tb;
    reg [1:0] sel;
    reg [3:0] out, expected_out;
    localparam period = 20;
    sel_4_way UUT (.sel(sel), .out(out));
    
    initial
    begin
        for (int i=0; i<4; i=i+1) begin
            sel = i;
            #period;
  
            expected_out = 2 ** sel; 
            assert(out == expected_out) else begin
                $display("sel: ", "%b", sel, ", out: ", "%b", out, ", expected out: ", "%b", expected_out);
                $fatal(1);
            end
        end
    end
endmodule
