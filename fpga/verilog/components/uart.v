module uart (input rx,
             output led);
    assign led = rx;
endmodule
