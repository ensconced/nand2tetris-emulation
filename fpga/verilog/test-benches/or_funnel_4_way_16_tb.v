`timescale 1 ns/10 ps

module or_funnel_4_way_16_tb;
    reg [63:0] in, rand_in;
    reg [15:0] a, b, c, d, out, expected_out;
    localparam period = 20;
    
    or_funnel_4_way_16 UUT (.in(in), .out(out));
    
    initial
    begin
        for (int i = 0; i < 128; i++) begin
            rand_in = $random();
            in <= rand_in;
            #period
            {a, b, c, d} = rand_in;
            expected_out = a | b | c | d;
            assert(out == expected_out) else begin
                $display("in: ", "%b", in, ", out: ", "%b", out, ", expected out: ", "%b", expected_out);
                $fatal(1);
            end;
        end
    end
endmodule
