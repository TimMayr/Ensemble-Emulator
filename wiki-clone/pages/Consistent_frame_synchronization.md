# Consistent frame synchronization

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Consistent_frame_synchronization) | View [other pages](Special_AllPages.xhtml#Consistent_frame_synchronization)

## Contents

  * 1 Introduction
  * 2 PAL timing
  * 3 Basic synchronization
  * 4 Writing to a particular pixel
  * 5 Ideal NMI
  * 6 NMI delay
  * 7 Compensating for NMI delay
  * 8 Sprite DMA always ends on even cycle
  * 9 VBL flag cleared at end of VBL
  * 10 VBL begins on odd cycles
  * 11 Synchronizing with even CPU cycle
  * 12 Simpler synchronization routine



## Introduction

This page describes a method for consistently synchronizing with the [PPU](PPU.xhtml "PPU") every frame from inside an [NMI](NMI.xhtml "NMI") handler, without having to cycle-time everything. This method allows synchronization just as good as is possible with completely cycle-timed code. At the beginning, the PPU is precisely synchronized with, ensuring that the code behaves the same every time it's run, every time the NES is powered up or reset. It's fully predictable. 

Currently only [PAL](PAL_video.xhtml "PAL video") version is covered, since the PAL PPU's frame timing is simpler. The NTSC version operates in a similar manner, and will be covered eventually. 

## PAL timing

    _See also:[Cycle reference chart](Cycle_reference_chart.xhtml "Cycle reference chart")_

The PAL NES has a master clock shared by the PPU and CPU. The CPU divides the master clock by 16 to get its instruction cycle clock, which we'll call _cycle_ for simplicity. For example, a NOP instruction takes 2 cycles. The PPU divides the master clock by 5 to get its pixel clock, which we'll call _pixel_ for simplicity. There are 16/5 = 3.2 pixels per cycle. 

A video frame consists of 312 scanlines, each 341 pixels long. Unlike NTSC, there are no short frames, and rendering being enabled or disabled has no effect on frame length. Thus, _every_ frame is exactly 312*341 = 106396 pixels = 33247.5 cycles long. We'll have pixel 0 refer to the first pixel of a frame, and pixel 106395 refer to the last pixel of a frame. 

A frame begins with the [vertical blanking interval (VBL)](The_frame_and_NMIs.xhtml#VBlank,_Rendering_Time,_and_NMIs "The frame and NMIs"), then the visible scanlines. The notation VBL+N refers to N cycles after the cycle that VBL began within, VBL+0. To talk about pixels since VBL, we simply refer to pixel P, where pixel 0 is the beginning of VBL, and pixel 106395 is the last pixel in the frame. 

## Basic synchronization

If we're going to write at a particular pixel, we must first synchronize the CPU to the beginning of a frame, so that pixel 0 begins at the beginning of a cycle, and we know how many cycles ago that was. Reading [$2002](PPU_registers.xhtml#Status_\(%242002\)_<_read "PPU registers") gives the current status of the VBL flag in bit 7, then clears it. The VBL flag is set at pixel 0 of each frame, and cleared around when VBL ends. We can use the VBL flag to achieve synchronization. 

A frame is 33247.5 cycles long. If we could somehow read $2002 every 33247.5 cycles, we'd read at the same point in each frame. But if we read $2002 every 33248 cycles, we'll be reading 0.5 cycles (1.6 pixels) later each successive frame. If we have a loop do this until it finds the VBL flag set, it will synchronize with the PPU. Each time through, it will read later in the frame, until it reads just as the VBL flag for the next frame is set. 
    
    
            ; Fine synchronize
    :       delay 33241
            bit $2002
            bpl :-
    

Cycle | PPU | CPU   
---|---|---  
0 |  |   
1 |  |   
...   
33246 |  | Read $2002 = 0   
33246.5 |  |   
33247 |  |   
33247.5 | Set VBL flag |   
...   
66494 |  | Read $2002 = 0   
66494.5 |  |   
66495 | Set VBL flag |   
...   
99742 |  | Read $2002 = 0   
99742.5 | Set VBL flag |   
...   
132990 | Set VBL flag | Read $2002 = $80   
  
Looking at it relative to each frame, we more clearly see how the CPU effectively reads later by half a cycle each frame. 

Cycle | Frame 1 | Frame 2 | Frame 3 | Frame 4 | Event   
---|---|---|---|---|---  
-1.5 | read |  |  |  |   
-1.0 |  | read |  |  |   
-0.5 |  |  | read |  |   
0 |  |  |  | read | VBL flag set   
  
The loop must be started so that the first $2002 read is slightly before the end of the frame, otherwise it might start out reading well after the flag has been set. We can do this by starting with a simpler coarse synchronization loop. 
    
    
    sync_ppu:
            ; Coarse synchronize
            bit $2002
    :       bit $2002
            bpl :-
            
            delay 33231
            jmp first
            
            ; Fine synchronize
    :       delay 33241
    first:  bit $2002
            bpl :-
     
            rts
    

The coarse synchronization loop might read $2002 just as the VBL flag was set, or read it nearly 7 cycles after it was set. Then, in the fine synchronization loop, $2002 is read 33240 to 33247 cycles later. In most cases, this will be slightly before the VBL flag is set, so the loop will delay and read $2002 again 33248 cycles later, etc. 

Once done, the CPU will have executed two cycles after the final $2002 read that found the VBL flag just set. 

## Writing to a particular pixel

In order to achieve some graphical effect, we want to write to the PPU at a particular pixel every frame. As an example, we'll write to $2006 at pixel 30400, which is near the upper-center of the screen. To simplify things, we'll not care what value we write. This requires that we write to $2006 at VBL+9500. 
    
    
            ; Synchronize to PPU
            jsr sync_ppu
            
            ; Delay almost a full frame, so that the code below begins on
            ; a frame.
            delay 33238
            
    vbl:    ; VBL begins in this cycle
            
            delay 9497
            sta $2006
    

Pixel | Cycle | Event   
---|---|---  
0 | 0 | VBL begins  
delay 9497   
...   
| 9497 | STA $2006   
| 9498 |   
| 9499 |   
30400 | 9500 | $2006 write   
  
If we try to make this write to the same pixel each frame, we run into a problem: the frame length isn't a whole number of cycles. We'll count frames and treat odd frames as being 33247 cycles long, and even frames 33248 cycles long, which will average to the correct 33247.5 cycles per frame. 
    
    
            ; Synchronize to PPU
            jsr sync_ppu
             
            ; Delay almost a full frame, so that the code below begins on
            ; a frame.
            delay 33233
            
            ; We were on frame 1 after sync_ppu, but vbl will begin on frame 2
            lda #2
            sta frame_count
            
    vbl:    ; VBL begins in this cycle
            
            delay 9497
            sta $2006
            
            delay 23731
            
            ; Delay extra cycle on even frames
            lda frame_count
            and #$01
            beq extra
    extra:  inc frame_count
    
            jmp vbl
    

Now our write time doesn't drift, but it still doesn't write to the same pixel each frame. Since even frames begin in the middle of a cycle, our write is half a cycle/1.6 pixels earlier. 

Odd frame pixel | Even frame pixel | Cycle | Event   
---|---|---|---  
0 |  | 0 | VBL begins  
delay 9497   
| 0 | 0.5   
...   
|  | 9497 | STA $2006   
|  | 9498 |   
|  | 9499 |   
30400 | 30398.4 | 9500 | $2006 write   
  
Our write will thus fall on pixel 30400 on odd frames, and pixel 30398.4 on even frames. That's the best we can do, regardless of how we write our code, as this is a hardware limitation. 

Another similar limitation is that when the NES is powered up or reset, the CPU and PPU master clock dividers start in random states, adding up to 1.6 additional pixels of variance. This offset doesn't change until the NES is powered off or reset. 

## Ideal NMI

Above, all the code had to be cycle-timed to ensure that each write occurred at the correct time. This isn't practical in most programs, which instead use NMI for synchronizing roughly to VBL. In these programs, timing-critical code is at the beginning of the NMI handler, followed by code that isn't carefully timed. Thus, such code relies on NMI occurring shortly after VBL, and not being delayed. 

Ideally, NMI would begin a fixed number of cycles after VBL, without waiting for the current instruction to finish. If that were the case, we'd have it nearly as easy as before. Here, we'll imagine NMI always occurs at VBL+2. NMI takes 7 cycles to vector to our NMI handler, so that our NMI handler begins at VBL+9. To simplify the code and timing diagrams, we won't bother saving any registers as we'd normally do in an NMI handler. 
    
    
    nmi:    ; VBL+9
            delay 9488
            sta $2006         ; write at VBL+9500
    

Even frame pixel | Odd frame pixel | Cycle | Event   
---|---|---|---  
0 |  | 0 | VBL begins   
| 0 | 0.5   
|  | 1 |   
|  | 2 | NMI vectored   
|  | 3 |   
|  | 4 |   
|  | 5 |   
|  | 6 |   
|  | 7 |   
|  | 8 |   
|  | 9 | delay 9488   
...   
|  | 9497 | STA $2006   
|  | 9498 |   
|  | 9499 |   
30400 | 30398.4 | 9500 | $2006 write   
  
## NMI delay

In reality, NMI waits until the current instruction completes before vectoring to the NMI handler, adding an extra delay as compared to the ideal NMI described above. Also, sometimes the NES powers up with the PPU and CPU dividers such that the NMI occurs an additional cycle later. 

By ensuring that a short instruction is executing when VBL occurs, we can minimize the delay before NMI is vectored. For example, if we have a series of NOP instructions executing when VBL occurs, NMI will occur from 2 to 4 cycles after VBL. The table shows the four possible timings, with each column titled with the time NMI vectoring begins. 
    
    
            nop
            nop
            nop
    

Cycle | VBL + 2 | VBL + 3 | VBL + 4 | Event   
---|---|---|---|---  
-1 |  | NOP |  |   
0 | NOP |  | NOP | VBL begins   
1 |  | NOP |  |   
2 | NMI vectored |  | NOP |   
3 |  | NMI vectored |  |   
4 |  |  | NMI vectored |   
  
So, at best, we have 2 to 4 cycles of delay between VBL and our NMI handler. 

Using a long sequence of NOP instructions isn't practical, because it requires either a large number of NOP instructions, or that we know how long the code before them takes so that we can delay entry into the NOP sequence until NMI is about to occur. If we instead have a simple infinite loop made of a single JMP instruction, we only increase the maximum delay by one cycle, to 5. 
    
    
     loop:   jmp loop
    

Cycle | VBL + 2 | VBL + 3 | VBL + 4 | VBL + 5 | Event   
---|---|---|---|---|---  
-1 | JMP |  |  | JMP |   
0 |  | JMP |  |  | VBL begins   
1 |  |  | JMP |  |   
2 | NMI vectored |  |  | JMP |   
3 |  | NMI vectored |  |  |   
4 |  |  | NMI vectored |  |   
5 |  |  |  | NMI vectored |   
  
## Compensating for NMI delay

With a JMP loop to wait for NMI, we have 2 to 5 cycles of delay between VBL and our NMI handler. We want to compensate for this delay D by delaying an additional 5-D cycles. Here, we have the NOP always begin at VBL+12. We can't actually do this, but it shows what we must do the equivalent of. 
    
    
    nmi:	delay 5-D
    	nop
    

Cycle | VBL + 2 | VBL + 3 | VBL + 4 | VBL + 5 | Event   
---|---|---|---|---|---  
-1 | JMP |  |  | JMP |   
0 |  | JMP |  |  | VBL begins   
1 |  |  | JMP |  |   
2 | NMI vectored |  |  | JMP |   
3 |  | NMI vectored |  |  |   
4 |  |  | NMI vectored |  |   
5 |  |  |  | NMI vectored |   
6 |  |  |  |  |   
7 |  |  |  |  |   
8 |  |  |  |  |   
9 | delay 3 |  |  |  |   
10 |  | delay 2 |  |  |   
11 |  |  | delay 1 |  |   
12 | NOP | NOP | NOP | NOP (no delay) |   
  
We just have to find out _how_ to determine the number of cycles of delay to add. 

## Sprite DMA always ends on even cycle

When sprite DMA ($4014) is written to, the next instruction always begins on an odd cycle. If the $4014 write is on an odd cycle, it pauses the CPU for an additional 513 cycles, otherwise 514 cycles. We can use this aspect to partially compensate for NMI's variable delay. 
    
    
    nmi:    lda #$07          ; sprites at $700
            sta $4014
            nop
    

Cycle | VBL + 2 | VBL + 3 | VBL + 4 | VBL + 5   
---|---|---|---|---  
0 | VBL begins   
1 |  |  |  |   
2 | NMI |  |  |   
3 |  | NMI |  |   
4 |  |  | NMI |   
5 |  |  |  | NMI   
6 |  |  |  |   
7 |  |  |  |   
8 |  |  |  |   
9 | LDA #$07 |  |  |   
10 |  | LDA #$07 |  |   
11 | STA $4014 |  | LDA #$07 |   
12 |  | STA $4014 |  | LDA #$07   
13 |  |  | STA $4014 |   
14 | $4014 write |  |  | STA $4014   
15 | 514-cycle DMA | $4014 write |  |   
16 |  | 513-cycle DMA | $4014 write |   
17 |  |  | 514-cycle DMA | $4014 write   
18 |  |  |  | 513-cycle DMA   
...   
527 |  |  |  |   
528 | DMA finishes | DMA finishes |  |   
529 | NOP | NOP |  |   
530 |  |  | DMA finishes | DMA finishes   
531 |  |  | NOP | NOP   
  
This reduces the number of different delays from four to two. The NOP always executes at either VBL+529 or VBL+531. This is an improvement. We just need a way to determine which time DMA finished at, and delay two extra cycles if it was the earlier one. 

## VBL flag cleared at end of VBL

The VBL flag is cleared near the end of VBL. If we read $2002 around the time the flag is cleared, we can determine whether the read occurred before or after the flag was cleared. We will have to avoid reading $2002 elsewhere in the NMI handler, since reading $2002 clears the flag. 

The VBL flag is cleared around pixel 23869, sometimes one less, so we want to read $2002 at VBL+7458 or VBL+7460. It works out nicely that sprite DMA leaves two cycles between the possible ending times, as this ensures that our $2002 read is several pixels before or after when the flag is cleared, giving us a good margin for error. If we find the flag set, we know we are on the earlier of the two DMA ending times, so we delay an extra two cycles. 
    
    
    nmi:    lda #$07          ; sprites at $700
            sta $4014
            delay 6926
            bit $2002         ; read at VBL+7458 or VBL+7460
            bpl skip
            bit 0
    skip:   nop
    

Cycle | VBL + 2 | VBL + 3 | VBL + 4 | VBL + 5   
---|---|---|---|---  
0 | VBL begins   
1 |  |  |  |   
2 | NMI |  |  |   
3 |  | NMI |  |   
4 |  |  | NMI |   
5 |  |  |  | NMI   
6 |  |  |  |   
7 |  |  |  |   
8 |  |  |  |   
9 | LDA #$07 |  |  |   
10 |  | LDA #$07 |  |   
11 | STA $4014 |  | LDA #$07 |   
12 |  | STA $4014 |  | LDA #$07   
13 |  |  | STA $4014 |   
14 | $4014 write |  |  | STA $4014   
15 | 514-cycle DMA | $4014 write |  |   
16 |  | 513-cycle DMA | $4014 write |   
17 |  |  | 514-cycle DMA | $4014 write   
18 |  |  |  | 513-cycle DMA   
...   
527 |  |  |  |   
528 | DMA finishes | DMA finishes |  |   
529 | delay 6926 | delay 6926 |  |   
530 |  |  | DMA finishes | DMA finishes   
531 |  |  | delay 6926 | delay 6926   
...   
7455 | BIT $2002 | BIT $2002 |  |   
7456 |  |  |  |   
7457 |  |  | BIT $2002 | BIT $2002   
7458 | $2002 read = $80 | $2002 read = $80 |  |   
7459 | BPL not taken | BPL not taken | VBL cleared | VBL cleared   
7460 |  |  | $2002 read = 0 | $2002 read = 0   
7461 | BIT 0 | BIT 0 | BPL taken | BPL taken   
7462 |  |  |  |   
7463 |  |  |  |   
7464 | NOP | NOP | NOP | NOP   
  
This achieves our goal, but not in all cases. 

## VBL begins on odd cycles

Unfortunately, VBL doesn't always begin during an even cycle, as we've so far assumed. When VBL begins during an odd cycle, our code doesn't work so well: 

Cycle | VBL + 2 | VBL + 3 | VBL + 4 | VBL + 5   
---|---|---|---|---  
1 | VBL begins   
2 |  |  |  |   
3 | NMI |  |  |   
4 |  | NMI |  |   
5 |  |  | NMI |   
6 |  |  |  | NMI   
7 |  |  |  |   
8 |  |  |  |   
9 |  |  |  |   
10 | LDA #$07 |  |  |   
11 |  | LDA #$07 |  |   
12 | STA $4014 |  | LDA #$07 |   
13 |  | STA $4014 |  | LDA #$07   
14 |  |  | STA $4014 |   
15 | $4014 write |  |  | STA $4014   
16 | 513-cycle DMA | $4014 write |  |   
17 |  | 514-cycle DMA | $4014 write |   
18 |  |  | 513-cycle DMA | $4014 write   
19 |  |  |  | 514-cycle DMA   
...   
527 |  |  |  |   
528 | DMA finishes |  |  |   
529 |  |  |  |   
530 |  | DMA finishes | DMA finishes |   
531 |  |  |  |   
532 |  |  |  | DMA finishes   
  
Now DMA ends at _three_ different times, covering a wider range than the original NMI times did, thus making things worse! 

We need to keep track of when VBL begins during an odd cycle, and compensate _before_ we begin DMA. After our PPU synchronization routine finishes, the last $2002 read it makes will have just found the VBL flag set. In the following table, that is cycle 0. 

Pixel | Cycle | Frame   
---|---|---  
0 | 0 | 1   
106392 | 33247.5 | 2   
212784 | 66495 | 3   
319176 | 99742.5 | 4   
425568 | 132990 | 5   
531960 | 166237.5 | 6   
638352 | 199485 | 7   
744744 | 232732.5 | 8   
  
Looking at which cycle each frame begins on, we see they follow a four-frame pattern: even, odd, odd, even. So we'll just have a variable that starts out at 1 and increments every frame, then examine bit 1 and delay an extra cycle if it's clear. This extra code takes 8 cycles on frames where VBL begins during an even cycle, and 7 cycles otherwise. 

But we also need to insert a complementary delay _after_ DMA, before the $2002 read, since on frames where VBL begins during an odd cycle we'll need to read $2002 one cycle later after DMA than for even frames. 
    
    
    nmi:    lda frame_count
            and #$02
            beq even
    even:   lda #$07          ; sprites at $700
            sta $4014
            delay 6911
            lda frame_count
            and #$02
            bne odd
    odd:    bit $2002
            bpl skip
            bit 0
    skip:   inc frame_count
            delay 2028
            sta $2006
    

Cycle | Frames 1, 4, 5, 8 ... | Frames 2, 3, 6, 7 ...   
---|---|---  
VBL + 2 | VBL + 3 | VBL + 4 | VBL + 5 | VBL + 2 | VBL + 3 | VBL + 4 | VBL + 5   
0 | VBL | VBL | VBL | VBL |  |  |  |   
1 |  |  |  |  | VBL | VBL | VBL | VBL   
2 | NMI |  |  |  |  |  |  |   
3 |  | NMI |  |  | NMI |  |  |   
4 |  |  | NMI |  |  | NMI |  |   
5 |  |  |  | NMI |  |  | NMI |   
6 |  |  |  |  |  |  |  | NMI   
7 |  |  |  |  |  |  |  |   
8 |  |  |  |  |  |  |  |   
9 | LDA frame_count |  |  |  |  |  |  |   
10 |  | LDA frame_count |  |  | LDA frame_count |  |  |   
11 |  |  | LDA frame_count |  |  | LDA frame_count |  |   
12 | AND #$02 |  |  | LDA frame_count |  |  | LDA frame_count |   
13 |  | AND #$02 |  |  | AND #$02 |  |  | LDA frame_count   
14 | BEQ taken |  | AND #$02 |  |  | AND #$02 |  |   
15 |  | BEQ taken |  | AND #$02 | BEQ not taken |  | AND #$02 |   
16 |  |  | BEQ taken |  |  | BEQ not taken |  | AND #$02   
17 | LDA #$07 |  |  | BEQ taken | LDA #$07 |  | BEQ not taken |   
18 |  | LDA #$07 |  |  |  | LDA #$07 |  | BEQ not taken   
19 | STA $4014 |  | LDA #$07 |  | STA $4014 |  | LDA #$07 |   
20 |  | STA $4014 |  | LDA #$07 |  | STA $4014 |  | LDA #$07   
21 |  |  | STA $4014 |  |  |  | STA $4014 |   
22 | $4014 write |  |  | STA $4014 | $4014 write |  |  | STA $4014   
23 | 514-cycle DMA | $4014 write |  |  | 514-cycle DMA | $4014 write |  |   
24 |  | 513-cycle DMA | $4014 write |  |  | 513-cycle DMA | $4014 write |   
25 |  |  | 514-cycle DMA | $4014 write |  |  | 514-cycle DMA | $4014 write   
26 |  |  |  | 513-cycle DMA |  |  |  | 513-cycle DMA   
...   
535 |  |  |  |  |  |  |  |   
536 | DMA finishes | DMA finishes |  |  | DMA finishes | DMA finishes |  |   
537 | delay 6911 | delay 6911 |  |  | delay 6911 | delay 6911 |  |   
538 |  |  | DMA finishes | DMA finishes |  |  | DMA finishes | DMA finishes   
539 |  |  | delay 6911 | delay 6911 |  |  | delay 6911 | delay 6911   
...   
7448 | LDA frame_count | LDA frame_count |  |  | LDA frame_count | LDA frame_count |  |   
7449 |  |  |  |  |  |  |  |   
7450 |  |  | LDA frame_count | LDA frame_count |  |  | LDA frame_count | LDA frame_count   
7451 | AND #$02 | AND #$02 |  |  | AND #$02 | AND #$02 |  |   
7452 |  |  |  |  |  |  |  |   
7453 | BNE not taken | BNE not taken | AND #$02 | AND #$02 | BNE taken | BNE taken | AND #$02 | AND #$02   
7454 |  |  |  |  |  |  |  |   
7455 | BIT $2002 | BIT $2002 | BNE not taken | BNE not taken |  |  | BNE taken | BNE taken   
7456 |  |  |  |  | BIT $2002 | BIT $2002 |  |   
7457 |  |  | BIT $2002 | BIT $2002 |  |  |  |   
7458 | $2002 read = $80 | $2002 read = $80 |  |  |  |  | BIT $2002 | BIT $2002   
7459 | BPL not taken | BPL not taken | VBL cleared | VBL cleared | $2002 read = $80 | $2002 read = $80 |  |   
7460 |  |  | $2002 read = 0 | $2002 read = 0 | BPL not taken | BPL not taken | VBL cleared | VBL cleared   
7461 | BIT 0 | BIT 0 | BPL taken | BPL taken |  |  | $2002 read = 0 | $2002 read = 0   
7462 |  |  |  |  | BIT 0 | BIT 0 | BPL taken | BPL taken   
7463 |  |  |  |  |  |  |  |   
7464 | INC frame_count | INC frame_count | INC frame_count | INC frame_count |  |  |  |   
7465 |  |  |  |  | INC frame_count | INC frame_count | INC frame_count | INC frame_count   
7466 |  |  |  |  |  |  |  |   
7467 |  |  |  |  |  |  |  |   
7468 |  |  |  |  |  |  |  |   
7469 | delay 2028 | delay 2028 | delay 2028 | delay 2028 |  |  |  |   
7470 |  |  |  |  | delay 2028 | delay 2028 | delay 2028 | delay 2028   
...   
9497 | STA $2006 | STA $2006 | STA $2006 | STA $2006 |  |  |  |   
9498 |  |  |  |  | STA $2006 | STA $2006 | STA $2006 | STA $2006   
9499 |  |  |  |  |  |  |  |   
9500 | $2006 write at VBL+9500 | $2006 write at VBL+9500 | $2006 write at VBL+9500 | $2006 write at VBL+9500 |  |  |  |   
9501 |  |  |  |  | $2006 write at VBL+9500 | $2006 write at VBL+9500 | $2006 write at VBL+9500 | $2006 write at VBL+9500   
  
The $2006 write is done at VBL+9500 in all cases. Remember that the right four columns have VBL beginning on cycle 1 (an odd cycle), which is why the final writes appear to be one cycle later than the others. 

## Synchronizing with even CPU cycle

Since our final synchronization method relies on knowing whether a given frame begins during an even or odd cycle, we must initially ensure that our PPU synchronization routine's final $2002 read is also during an even cycle. Since the fine synchronization loop takes an even number of cycles, we merely need to ensure that the first time through that the $2002 read is on an even cycle. We can do this by initiating sprite DMA before the fine synchronization loop. 
    
    
    sync_ppu:
            ; Coarse synchronize
            bit $2002
    :       bit $2002
            bpl :-
            
            sta $4014
            delay 32713
            jmp first
            
            ; Fine synchronize
    :       delay 33241
    first:  bit $2002
            bpl :-
            
            ; NMI won't be fired until frame 2
            lda #2
            sta frame_count
            
            rts
    

The STA $4014 takes up to 518 cycles, so we subtract that from the initial delay. After the STA $4014, the delay begins on an odd cycle. Since also it's an odd number of cycles until the $2002 read, it will occur on an even cycle, as desired. 

## Simpler synchronization routine

The PPU synchronization routine is pretty short, but it requires use of the delay macro, which takes a fair amount of code to implement. It's possible to eliminate that without any negative impact. 

The fine synchronization loop needs to read $2002 every 33248 cycles, so it can find when the VBL flag is set just before the read. This seems to require a long delay between reads. Until the final iteration, it must not find the VBL flag set. If it were like the coarse loop and read the VBL flag every 7 cycles, it would clearly stop somewhere near the beginning of the first frame, but rarely right at the beginning. It might read $2002 one cycle before the VBL flag is set, loop, then read it 7 cycles later and find it now set. This isn't what we want. If we read it slightly more often, like every 33248/2 = 16624 cycles, it would still work, since the VBL flag is automatically cleared near the end of VBL. 
    
    
    sync_ppu:
            ; Coarse synchronize
            bit $2002
    :       bit $2002
            bpl :-
            
            sta $4014
            delay 16089
            jmp first
            
            ; Fine synchronize
    :       delay 16617
    first:  bit $2002
            bpl :-
    
            rts
    

Cycle | PPU | CPU   
---|---|---  
0 | Set VBL flag |   
7459 | Clear VBL flag |   
16622 |  | Read $2002 = 0   
33246 |  | Read $2002 = 0   
33247.5 | Set VBL flag |   
40706.5 | Clear VBL flag |   
49870 |  | Read $2002 = 0   
66494 |  | Read $2002 = 0   
66495 | Set VBL flag |   
73954 | Clear VBL flag |   
83118 |  | Read $2002 = 0   
99742 |  | Read $2002 = 0   
99742.5 | Set VBL flag |   
107201.5 | Clear VBL flag |   
116366 |  | Read $2002 = 0   
132990 | Set VBL flag | Read $2002 = $80   
  
That works, but reducing the delays doesn't eliminate the need for them. The important thing is that the $2002 read only be able to happen just after the VBL flag is set, rather than many cycles after it was set. Rather than rely on the PPU to clear the VBL flag, we can clear it ourselves. 16 is a factor of 33248, so we can have the loop take only 16 cycles and still synchronize properly. 
    
    
    sync_ppu:
            ; Coarse synchronize
            bit $2002
    :       bit $2002
            bpl :-
            
            sta $4014
            bit <0
            
            ; Fine synchronize
    :       bit <0
            nop
            bit $2002
            bit $2002
            bpl :-
    
            rts
    

Cycle | PPU | CPU   
---|---|---  
0 | Set VBL flag |   
10 |  | Dummy read $2002 = $80   
14 |  | Read $2002 = 0   
26 |  | Dummy read $2002 = 0   
30 |  | Read $2002 = 0   
...   
33242 |  | Dummy read $2002 = 0   
33246 |  | Read $2002 = 0   
33247.5 | Set VBL flag |   
33258 |  | Dummy read $2002 = $80   
33262 |  | Read $2002 = 0   
...   
66490 |  | Dummy read $2002 = 0   
66494 |  | Read $2002 = 0   
66495 | Set VBL flag |   
66506 |  | Dummy read $2002 = $80   
66510 |  | Read $2002 = 0   
...   
99738 |  | Dummy read $2002 = 0   
99742 |  | Read $2002 = 0   
99742.5 | Set VBL flag |   
99754 |  | Dummy read $2002 = $80   
99758 |  | Read $2002 = 0   
...   
132986 |  | Dummy read $2002 = 0   
132990 | Set VBL flag | Read $2002 = $80   
  
Essentially there's a four-cycle window that the second $2002 read in the loop is watching for the VBL flag to be set within. On entry to the loop, we ensure that the flag will never be set within this window. Every 33248/16 = 2078 iterations, the second $2002 read is half a cycle later in the frame, just like the original version. On every other iteration, the dummy $2002 read four cycles before has ensured that the VBL flag is cleared. 
