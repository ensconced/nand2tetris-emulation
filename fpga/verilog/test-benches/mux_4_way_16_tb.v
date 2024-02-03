`timescale 1 ns/10 ps

module mux_4_way_16_tb;
    reg [63:0] in;
    reg [15:0] out, expected_out;
    reg [1:0] sel;
    localparam period = 20;
    
    mux_4_way_16 UUT (.in(in), .sel(sel), .out(out));
    
    initial
    begin
        for (int i = 0; i < 128; i++) begin
            in = $random();
            sel = $random();
            #period;
            expected_out = in[sel * 16+:16];
            assert(out == expected_out) else begin
                $display("in: ", "%b", in, ", out: ", "%b", out, ", expected out: ", "%b", expected_out);
                $fatal(1);
            end;
        end
    end
endmodule
