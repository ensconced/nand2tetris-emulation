`timescale 1 ns/10 ps

module ram_16k_tb;
    reg [15:0] in, out, val_a, val_b, expected_out;
    reg [13:0] addr, addr_a, addr_b; 
    reg load, clock;
    ram_16k UUT (.in(in), .addr(addr), .load(load), .clock(clock), .out(out));
    initial
    begin
        for (int i = 0; i < 128; i++) begin
            val_a = $random();
            val_b = $random();
            addr_a = $random();
            addr_b = $random();

            // load a value in to the first address
            clock = 0;
            load = 1;
            in = val_a;
            addr = addr_a;
            #10
            clock = 1;
            #10
            assert(out == val_a) else begin
                $display("i: ", i, ", val_a: ", val_a, " , val_b: ", val_b, ", out: ", out);
                $fatal(1);
            end;

            // load another value into another address
            in = val_b;
            addr = addr_b;
            clock = 0;
            #10
            clock = 1;
            #10

            // synchronously read value from the first address
            load = 0;
            addr = addr_a;
            #10
            expected_out = (addr_a == addr_b) ? val_b : val_a;
            assert(out == expected_out) else begin
                $display("addr_a: ", addr_a, ", addr_b: ", addr_b, ", val_a: ", val_a, " , val_b: ", val_b, ", out: ", out);
                $fatal(1);
            end;
            
            // synchronously read value from the second address
            addr = addr_b;
            #10
            assert(out == val_b) else begin
                $display("addr_a: ", addr_a, ", addr_b: ", addr_b, ", val_a: ", val_a, " , val_b: ", val_b, ", out: ", out);
                $fatal(1);
            end;
        end
    end
endmodule
