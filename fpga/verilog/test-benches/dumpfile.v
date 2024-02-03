module dumpfile;
    initial
    begin
        $dumpfile("test.vcd");
        $dumpvars(0, computer_tb);
    end
endmodule