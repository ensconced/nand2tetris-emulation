`timescale 1 ns/10 ps

module is_non_zero_tb;
    reg [15:0] in, rand_in;
    reg out, expected_out;
    localparam period = 20;
    
    is_non_zero UUT (.in(in), .out(out));
    
    initial
    begin
        in <= 16'b0;
        #period
        expected_out = 0;
        assert(out == expected_out) else begin
            $display("in: ", "%b", in, ", out: ", "%b", out, ", expected out: ", "%b", expected_out);
            $fatal(1);
        end;

        for (int i = 0; i < 128; i++) begin
            rand_in = $random();
            in <= rand_in;
            #period
            expected_out = (rand_in == 0) ? 0 : 1;
            assert(out == expected_out) else begin
                $display("in: ", "%b", in, ", out: ", "%b", out, ", expected out: ", "%b", expected_out);
                $fatal(1);
            end;
        end
    end
endmodule
