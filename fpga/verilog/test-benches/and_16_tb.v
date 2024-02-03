`timescale 1 ns/10 ps

module and_16_tb;
    reg [15:0] in_a, in_b, out;
    reg [15:0] rand_in_a, rand_in_b, expected_out;
    localparam period = 20;
    
    and_16 UUT (.in_a(in_a), .in_b(in_b), .out(out));
    
    initial
    begin
        for (int i = 0; i < 128; i++) begin
            rand_in_a = $random();
            rand_in_b = $random();
            in_a <= rand_in_a;
            in_b <= rand_in_b;
            #period
            expected_out = rand_in_a & rand_in_b;
            assert(out == expected_out) else begin
                $display("in_a: ", "%b", in_a, ", in_b: ", "%b", in_b, ", out: ", "%b", out, ", expected out: ", "%b", expected_out);
                $fatal(1);
            end;
        end
    end
endmodule
