`timescale 1 ns/10 ps

module not_16_tb;
    reg [15:0] in, out;
    reg [15:0] rand_in = 0;
    localparam period = 20;
    
    not_16 UUT (.in(in), .out(out));
    
    initial
    begin
        for (int i = 0; i < 128; i++) begin
            rand_in = $random();
            in <= rand_in;
            #period
            assert(out == ~in) else begin
                $display("in: ", "%b", in, ", out: ", "%b", out, "expected out: ", "%b", ~in);
                $fatal(1);
            end;
        end
    end
endmodule
