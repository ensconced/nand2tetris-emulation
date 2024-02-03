`timescale 1 ns/10 ps

module or_8_way_tb;
    reg [7:0] in, rand_in;
    reg out, expected_out;
    localparam period = 20;
    
    or_8_way UUT (.in(in), .out(out));
    
    initial
    begin
        in <= 8'b0;
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
