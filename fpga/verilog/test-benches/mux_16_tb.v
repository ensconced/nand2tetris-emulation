`timescale 1 ns/10 ps

module mux_16_tb;
    reg [15:0] in_a, in_b, out, rand_in_a, rand_in_b, expected_out;
    reg sel, rand_sel;
    localparam period = 20;
    
    mux_16 UUT (.in_a(in_a), .in_b(in_b), .sel(sel), .out(out));
    
    initial
    begin
        for (int i = 0; i < 128; i++) begin
            rand_in_a = $random();
            rand_in_b = $random();
            rand_sel = $random();
            in_a <= rand_in_a;
            in_b <= rand_in_b;
            sel <= rand_sel;
            #period
            expected_out = sel ? rand_in_b : rand_in_a;
            assert(out == expected_out) else begin
                $display("in_a: ", "%b", in_a, ", in_b: ", "%b", in_b, ", sel: ", sel, ", out: ", "%b", out, ", expected out: ", "%b", expected_out);
                $fatal(1);
            end;
        end
    end
endmodule
