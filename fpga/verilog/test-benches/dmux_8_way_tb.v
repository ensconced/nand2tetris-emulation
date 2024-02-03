`timescale 1 ns/10 ps

module dmux_8_way_tb;
    reg in;
    reg [2:0] sel;
    reg [7:0] out, expected_out;
    localparam period = 20;
    dmux_8_way UUT (.in(in), .sel(sel), .out(out));
    
    initial
    begin
        for (int i=0; i<16; i=i+1) begin
            { in, sel } = i;
            #period;
  
            expected_out = (in == 1) ? 2 ** sel : 0; 
            assert(out == expected_out) else begin
                $display("in:", "%b", in, ", sel: ", "%b", sel, ", out: ", "%b", out, ", expected out: ", "%b", expected_out);
                $fatal(1);
            end
        end
    end
endmodule
