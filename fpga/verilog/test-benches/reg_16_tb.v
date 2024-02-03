`timescale 1 ns/10 ps

module reg_16_tb;
    reg [15:0] in, out, val_a, val_b;
    reg load, clock;
    reg_16 UUT (.in(in), .load(load), .clock(clock), .out(out));
    initial
    begin
        for (int i = 0; i < 128; i++) begin
            val_a = $random();
            val_b = $random();

            // load a value in to the register
            clock = 0;
            load = 1;
            in = val_a;
            #10
            clock = 1;
            #10
            assert(out == val_a) else begin
                $display("out: ", "%b", out);
                $fatal(1);
            end;
            
            // if load is zero, the output doesn't change even when the input does
            load = 0;
            in = val_b;
            clock = 0;
            #10
            clock = 1;
            #10
            assert(out == val_a) else begin
                $display("out: ", "%b", out);
                $fatal(1);
            end;

            // but when we set load back to one, the output will change...
            load = 1;
            clock = 0;
            #10
            clock = 1;
            #10
            assert(out == val_b) else begin
                $display("out: ", "%b", out);
                $fatal(1);
            end;
        end
    end
endmodule
