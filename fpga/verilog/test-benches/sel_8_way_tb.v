`timescale 1 ns/10 ps

module sel_8_way_tb;
    reg [2:0] sel;
    reg [7:0] out, expected_out;
    localparam period = 20;
    sel_8_way UUT (.sel(sel), .out(out));
    
    initial
    begin
        for (int i=0; i<8; i=i+1) begin
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
