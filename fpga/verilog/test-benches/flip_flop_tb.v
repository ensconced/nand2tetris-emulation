`timescale 1 ns/10 ps

module flip_flop_tb;
    reg in, out, clock;
    flip_flop UUT (.in(in), .clock(clock), .out(out));
    initial
    begin
        // initial posedge to set out to 1
        clock = 0;
        in = 1;
        #10
        clock = 1;
        #10
        assert(out == 1) else begin
            $display("out: ", "%b", out);
            $fatal(1);
        end;

        // out stays at 1 when clock falls back to 0
        clock = 0;
        #10
        assert(out == 1) else begin
            $display("out: ", "%b", out);
            $fatal(1);
        end;

        // if in falls to 0, out still stays at 1
        in = 0;
        #10
        assert(out == 1) else begin
            $display("out: ", "%b", out);
            $fatal(1);
        end;

        // but on posedge, out will now fall to 0
        clock = 1;
        #10;
        assert(out == 0) else begin
            $display("out: ", "%b", out);
            $fatal(1);
        end;
    end
endmodule
