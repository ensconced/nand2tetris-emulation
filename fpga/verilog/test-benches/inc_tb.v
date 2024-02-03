`timescale 1 ns/10 ps

module inc_tb;
    reg [15:0] in, out;
    reg [15:0] rand_in, expected_out;
    localparam period = 20;
    
    inc UUT (.in(in), .out(out));
    
    initial
    begin
        for (int i = 0; i < 128; i++) begin
            rand_in = $random();
            in <= rand_in;
            #period
            expected_out = rand_in + 1;
            assert(out == expected_out) else begin
                $display("in: ", "%b", in, ", out: ", "%b", out, ", expected out: ", "%b", expected_out);
                $fatal(1);
            end;
        end
    end
endmodule
