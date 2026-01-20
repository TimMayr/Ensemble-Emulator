# Serial Cable Routines

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Serial_Cable_Routines) | View [other pages](Special_AllPages.xhtml#Serial_Cable_Routines)

This page covers implementation of bit-banged serial read/write routines for use with a [ serial cable](Serial_Cable_Construction.xhtml "Serial Cable Construction"). It's meant if you're implementing your own, or want to understand how the library routines work. 

## Contents

  * 1 Serial
    * 1.1 Encoding
    * 1.2 Decoding
    * 1.3 Stop bit/idle
  * 2 Implementation
    * 2.1 Timing
    * 2.2 Initialization
    * 2.3 Transmit
    * 2.4 Receive
    * 2.5 Timing accuracy



# Serial

## Encoding

Serial data bytes are transferred as a series of bits followed by an idle state until the next byte. Each bit lasts the same amount of time. An 8-bit byte is transferred as 10 bits, with the first and last bits serving as markers of the start and end of the byte. The start bit is encoded as a 0, and the stop bit and idle state as a 1. Data bit 0 is first. 
    
    
    -----       ----- ----- ----- ----- ----- ----- ----- ----- ---------
         \     /     X     X     X     X     X     X     X     /            
          ----- ----- ----- ----- ----- ----- ----- ----- -----
          start   0     1     2     3     4     5     6     7   stop
    

For example, the byte 00110001 ($31) is encoded as 
    
    
    --------       -----                   -----------             ----------
            \     /     \                 /           \           /
             -----       -----------------             -----------
               0     1     0     0     0     1     1     0     0     1
    

## Decoding

To decode a byte, the receiver must read each bit, ideally in its center: 
    
    
    --------       ----- ----- ----- ----- ----- ----- ----- ----- ---------
            \     /     X     X     X     X     X     X     X     /
             ----- ----- ----- ----- ----- ----- ----- ----- -----
                     *     *     *     *     *     *     *     *
    

The transmitter and receiver each have their own clock to time the bits, and these might differ. So the receiver might slowly shift towards one edge due to timing differences. If it weren't aiming for the center, it would be more likely to hit an edge. At that point, it might read an adjacent bit. 

The receiver must re-synchronize on each byte because its clock can't be accurate enough stay in sync with the transmitter's over long periods. The beginning of the start bit provides a known transition it can wait for. Once this occurs, it knows a byte is beginning, and that it should wait 1.5 bits before reading bit 0 (one bit to skip the start bit, and a half bit to get to the middle of bit 0). 
    
    
    --------       ----- ----- ----- ----- ----- ----- ----- ----- ---------
            \     /     X     X     X     X     X     X     X     /
             ----- ----- ----- ----- ----- ----- ----- ----- -----
                     *     *     *     *     *     *     *     *
            ^     ^  ^
          edge    1  .5
    

Above the receiver waits for the edge, then one bit, then a half bit to read bit 0. 

## Stop bit/idle

The idle state between bytes is also a 1, so it's as if the stop bit lasts until the next start bit. It can last any amount longer than a normal bit, and doesn't have to be a multiple of a bit long. That is, two bytes may be back-to-back, have a stop bit that's slightly longer than normal, or have one that's much longer than normal: 
    
    
    --- ----- ----- ----- ----- ----- -----       ----- ----- ----- ----- -----
       X     X     X     X     X     /     \     X     X     X     X     X 
    --- ----- ----- ----- ----- -----       ----- ----- ----- ----- ----- -----
          3     4     5     6     7   stop  start   0     1    2      3     4
    
    
    
    --- ----- ----- ----- ----- ----- -------       ----- ----- ----- ----- ---
       X     X     X     X     X     /       \     X     X     X     X     X 
    --- ----- ----- ----- ----- -----         ----- ----- ----- ----- ----- ---
          3     4     5     6     7    stop   start   0     1    2      3    
    
    
    
    --- ----- ----- ----- ----- ----- -------------------------------       ---
       X     X     X     X     X     /                               \     X  
    --- ----- ----- ----- ----- -----                                 ----- ---
          3     4     5     6     7   stop          idle              start
    

# Implementation

The following implementation is for clarity rather than usability or efficiency. 

## Timing

A serial rate of 57600 bits per second is the most useful on the NES. This gives the following timings: 

| NTSC | PAL   
---|---|---  
CPU Clock | 1789773Hz | 1662607Hz   
Clocks per bit | 31.07 | 28.86   
Rounded clocks | 31 | 29   
Error | -0.2% | +0.5%   
  
Rounding to 31 (29 PAL) cycles per bit gives less than half a percent timing error, well within RS-232 tolerances. This allows the code to be written as simple loops. 

## Initialization

Before transmitting or receiving for the first time, write $03 to $4016 and delay: 
    
    
            ldx #$03
            stx $4016
            ldx #350/5      ; delay 350 cycles, more than 10 bit lengths on NTSC
    wait:   dex
            bne wait
    

This puts an idle state on the output lines so that the first byte sent will not be corrupt. It also prevents Famicom or other controllers also connected from interfering with receiving (as long as the A button isn't being pressed). 

## Transmit

To transmit a byte, output each of the 10 bits for 31 cycles each (29 for PAL). 

| NTSC | PAL   
---|---|---  
Output start bit (0)   
Delay | 31 | 29   
Output bit 0   
Delay | 31 | 29   
...   
Output bit 7   
Delay | 31 | 29   
  
  

    
    
            clc             ; start bit
            ldx #10
    loop:   tay
            lda #$ff        ; replicate carry in low two bits and output
            adc #0
            eor #$ff
            and #%11
            sta $4016
            tya
            sec             ; stop bit
            ror a           ; next bit into carry, stop bit into shift reg
            nop             ; delay 6 cycles
            nop
            nop             ; remove for PAL timing
            dex
            bne loop
    

Remove the indicated NOP for PAL timing. 

This outputs the start bit (0), 8 data bits, and the stop bit (1), each lasting 31 cycles. Note how it only outputs on bits 0 and 1 of $4016, and clears the others. Some serial connections may be using bit 1 in the future, so a routine should write to both bits if possible. Other devices may be using higher bits, so these should always be clear. 

## Receive

The NES inverts serial input, but not output, so when receiving data, things are inverted: a start bit is 1, a stop bit 0, and data bits are inverted. 

To receive a byte, wait for the start bit, delay 1.5 bits = 46.5 cycles (42.5 for PAL), read bit 0, delay 31 cycles (29 for PAL), read bit 1, etc. 

| NTSC | PAL   
---|---|---  
Wait for beginning of start bit   
Delay | 46.5 | 42.5   
Input bit 0   
Delay | 31 | 29   
...   
Input bit 7   
      
    
            lda #%10111 ; wait for start bit
    start:  bit $4017
            beq start
                        ; LDA $4017 here would be 9.5 cycles after start bit
            nop         ; delay 17 cycles
            nop
            nop
            nop
            nop
            pha
            pla
            ldx #8      ; read 8 data bits
            nop         ; remove for PAL timing
    loop:   nop         ; remove for PAL timing
            nop         ; delay 10 cycles
            nop
            nop
            nop
            nop
            tay
            lda #%10111 ; mask input lines and put into carry
            and $4017
            cmp #1
            tya
            ror a       ; shift into output byte
            dex
            bne loop
            eor #$FF    ; invert final byte
    

Remove both indicated NOPs for PAL timing. 

The receive loop masks the bits serial data might come in on, and sets the carry if any of these is not zero. It shifts this bit into the shift register, and un-inverts everything in the end. 

The start bit loop waits for the beginning of the start bit. It reads $4017 every 7 cycles: 
    
    
    bit $4017 ; 4 read
    beq start ; 3
    bit $4017 ; 4 read
    beq start ; 2 (not taken)
    nop
    ...
    

The start bit could occur anywhere between two of these reads, so the next read to notice it will be seeing it from 0 to almost 7 cycles later. It might read just as the start bit begins and see it immediately, or it might read just before it begins and not see it until 7 cycles later when it reads again. On average it thus notices it 3.5 cycles after it began. So we must add 3.5 cycles to our calculations of how long it's been since the start bit began. This means that if we put an LDA $4017 just after the BEQ, it would read on average 9.5 cycles after the start bit began (3.5+4+2): 
    
    
    bit $4017 ; 4 read
    beq start ; 3
    bit $4017 ; 4 read
    beq start ; 2 (not taken)
    lda $4017 ; 4 reads 9.5 cycles on average after start bit began
    

## Timing accuracy

The 31/29 and 46.5/42.5 delays give the following timings, in cycles relative to the beginning of the start bit. The actual NTSC/PAL time is listed, then the ideal time, and the error (difference). 

Transmit timing is only a quarter of a percent off for NTSC and half a percent for PAL. Listed is the starting time for each bit. 

Bit | NTSC | Ideal | Error | PAL | Ideal | Error   
---|---|---|---|---|---|---  
Start | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0   
0 | 31.0 | 31.1 | -0.1 | 29.0 | 28.9 | 0.1   
1 | 62.0 | 62.1 | -0.1 | 58.0 | 57.7 | 0.3   
2 | 93.0 | 93.2 | -0.2 | 87.0 | 86.6 | 0.4   
3 | 124.0 | 124.3 | -0.3 | 116.0 | 115.5 | 0.5   
4 | 155.0 | 155.4 | -0.4 | 145.0 | 144.3 | 0.7   
5 | 186.0 | 186.4 | -0.4 | 174.0 | 173.2 | 0.8   
6 | 217.0 | 217.5 | -0.5 | 203.0 | 202.1 | 0.9   
7 | 248.0 | 248.6 | -0.6 | 232.0 | 230.9 | 1.1   
Stop | 279.0 | 279.7 | -0.7 | 261.0 | 259.8 | 1.2   
  
Receive timing is good on average, though all the times can be +/- 3.5 cycles shifted due to the start bit loop. Listed is the times the routine reads each bit in the middle, along with the ideal and error. 

Bit | NTSC | Ideal | Error | PAL | Ideal | Error   
---|---|---|---|---|---|---  
0 | 46.5 | 46.6 | -0.1 | 42.5 | 43.3 | -0.8   
1 | 77.5 | 77.7 | -0.2 | 71.5 | 72.2 | -0.7   
2 | 108.5 | 108.8 | -0.3 | 100.5 | 101.0 | -0.5   
3 | 139.5 | 139.8 | -0.3 | 129.5 | 129.9 | -0.4   
4 | 170.5 | 170.9 | -0.4 | 158.5 | 158.8 | -0.3   
5 | 201.5 | 202.0 | -0.5 | 187.5 | 187.6 | -0.1   
6 | 232.5 | 233.0 | -0.5 | 216.5 | 216.5 | 0.0   
7 | 263.5 | 264.1 | -0.6 | 245.5 | 245.3 | 0.2   
  
In the above tables, the best we can do in software is to keep the error within -0.5 to +0.5 cycles, so we can't improve on most of the above timings. To correct the rest, we'd do the following. Transmit, NTSC: extra cycle during bit 6, PAL: one fewer cycle during bit 3. Receive, NTSC: extra cycle after bit 5, PAL: extra cycle before bit 0, one fewer cycle after bit 2. This requires unrolled loops so these special actions can be done on particular iterations. 
