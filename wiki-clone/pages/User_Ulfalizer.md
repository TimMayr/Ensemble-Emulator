# User:Ulfalizer

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AUlfalizer) | View [other pages](Special_AllPages.xhtml#User_Ulfalizer)

## Contents

  * 1 Misc. timing stuff brought together in one place
    * 1.1 Reads and writes
      * 1.1.1 Read (LDA $1234, NOP)
      * 1.1.2 Write (LDA #$AB, STA $1234, NOP)
      * 1.1.3 Read/write observations
      * 1.1.4 Read/write timing
    * 1.2 Interrupts
      * 1.2.1 Detection intervals for various instructions
      * 1.2.2 Interrupt hijacking
      * 1.2.3 Reset
    * 1.3 M2 duty cycle
    * 1.4 PPU interface
    * 1.5 Timing charts
  * 2 Sprite overflow bug



# Misc. timing stuff brought together in one place

## Reads and writes

### Read (LDA $1234, NOP)

[http://www.qmtpro.com/~nes/chipimages/visual2a03/?graphics=false&logmore=Execute,phi2&a=0&d=ad3412ea&steps=25](http://www.qmtpro.com/~nes/chipimages/visual2a03/?graphics=false&logmore=Execute,phi2&a=0&d=ad3412ea&steps=25)

Tick-by-tick from <http://nesdev.org/6502_cpu.txt> interleaved with steps from Visual 2A03: 
    
    
     #  address R/W description
    --- ------- --- -------------------------------------------------
     1    PC     R  fetch opcode, increment PC
        ab    db  rw  pc    phi2
        0000  ad  1   0000  0
        0000  ad  1   0000  1
     2    PC     R  fetch low byte of address, increment PC
        ab    db  rw  pc    phi2
        0001  34  1   0001  0
        0001  34  1   0001  1
     3    PC     R  fetch high byte of address, increment PC
        ab    db  rw  pc    phi2
        0002  12  1   0002  0
        0002  12  1   0002  1
     4  address  R  read from effective address
        ab    db  rw  pc    phi2
        1234  00  1   0003  0
        1234  00  1   0003  1
    

### Write (LDA #$AB, STA $1234, NOP)

[http://www.qmtpro.com/~nes/chipimages/visual2a03/?graphics=false&logmore=Execute,phi2&a=0&d=a9ab8d3412ea&steps=29](http://www.qmtpro.com/~nes/chipimages/visual2a03/?graphics=false&logmore=Execute,phi2&a=0&d=a9ab8d3412ea&steps=29)

Tick-by-tick from <http://nesdev.org/6502_cpu.txt> interleaved with steps from Visual 2A03: 
    
    
     #  address R/W description
    --- ------- --- ------------------------------------------
     1    PC     R  fetch opcode, increment PC
        ab    db  rw  pc    phi2
        0002  8d  1   0002  0
        0002  8d  1   0002  1
     2    PC     R  fetch low byte of address, increment PC
        ab    db  rw  pc    phi2
        0003  34  1   0003  0
        0003  34  1   0003  1
     3    PC     R  fetch high byte of address, increment PC
        ab    db  rw  pc    phi2
        0004  12  1   0004  0
        0004  12  1   0004  1
     4  address  W  write register to effective address
        ab    db  rw  pc    phi2
        1234  12  0   0005  0
        1234  ab  0   0005  1
    

### Read/write observations

  * Address bus and **rw** changes right away (during **φ1**).
  * Values appear to be read during **φ2** (**db _x_** pins buffered on **cclk**), and **db** changes during **φ2** for writes too.
  * [Clocks section of tutorial](Visual_circuit_tutorial.xhtml#Clocks "Visual circuit tutorial").



### Read/write timing

"The address is guaranteed to be stable 300 nanoseconds after the leading edge of Phase One, and the data must be stable 100 nanoseconds before the trailing edge of Phase Two. At 1.0 MHz operation, this allows the memory devices approximately 575 ns to make data available on the data bus.", from <http://users.telenet.be/kim1-6502/6502/hwman.html>. 

## [Interrupts](CPU_interrupts.xhtml "CPU interrupts")

Sampled on the falling edge of **φ2**. IRQ detection depends on nodes nnT2BR (branch-related) and 646 (maybe a "sampling points" signal) being low. 

### Detection intervals for various instructions

Smallest IRQ assertion interval that will trigger an IRQ for LDA #FF: [http://visual6502.org/JSSim/expert.html?logmore=Execute,irq,IRQP,State,nnT2BR,646&a=0&d=58A9FF1890FE&irq0=5&irq1=6&steps=20](http://visual6502.org/JSSim/expert.html?logmore=Execute,irq,IRQP,State,nnT2BR,646&a=0&d=58A9FF1890FE&irq0=5&irq1=6&steps=20)

Ditto for LSR $AB: [http://visual6502.org/JSSim/expert.html?logmore=Execute,irq,IRQP,State,nnT2BR,646&a=0&d=5846AB1890FE&irq0=11&irq1=12&steps=20](http://visual6502.org/JSSim/expert.html?logmore=Execute,irq,IRQP,State,nnT2BR,646&a=0&d=5846AB1890FE&irq0=11&irq1=12&steps=20)

Ditto for LSR $AB with NMI instead of IRQ: [http://visual6502.org/JSSim/expert.html?logmore=Execute,nmi,~NMIG,State&a=0&d=58461890FE&nmi0=11&nmi1=12&steps=20](http://visual6502.org/JSSim/expert.html?logmore=Execute,nmi,~NMIG,State&a=0&d=58461890FE&nmi0=11&nmi1=12&steps=20)

IRQ triggered by assertion during first cycle of BCC - no page crossing, offset 0, pad with SEDs: [http://visual6502.org/JSSim/expert.html?logmore=Execute,irq,IRQP,State,nnT2BR,646&a=0&d=58189000F8F8&irq0=9&irq1=10&steps=30](http://visual6502.org/JSSim/expert.html?logmore=Execute,irq,IRQP,State,nnT2BR,646&a=0&d=58189000F8F8&irq0=9&irq1=10&steps=30)

IRQ triggered by assertion during first cycle of BCC - page crossing, offset E0 (backwards jump), pad with SEDs: [http://visual6502.org/JSSim/expert.html?logmore=Execute,irq,IRQP,State,nnT2BR,646&a=0&d=581890E0&a=ffe4&d=F8F8&irq0=9&irq1=10&steps=30](http://visual6502.org/JSSim/expert.html?logmore=Execute,irq,IRQP,State,nnT2BR,646&a=0&d=581890E0&a=ffe4&d=F8F8&irq0=9&irq1=10&steps=30)

IRQ triggered by assertion before fixup cycle of BCC - page crossing, offset E0 (backwards jump), pad with SEDs: [http://visual6502.org/JSSim/expert.html?logmore=Execute,irq,IRQP,State,nnT2BR,646&a=0&d=581890E0&a=ffe4&d=F8F8&irq0=13&irq1=14&steps=30](http://visual6502.org/JSSim/expert.html?logmore=Execute,irq,IRQP,State,nnT2BR,646&a=0&d=581890E0&a=ffe4&d=F8F8&irq0=13&irq1=14&steps=30)

IRQ during JMP Absolute (to after JMP instruction): [http://visual6502.org/JSSim/expert.html?logmore=Execute,irq,IRQP,State,nnT2BR,646&a=0&d=584C0400F8F8&irq0=7&irq1=8&steps=30](http://visual6502.org/JSSim/expert.html?logmore=Execute,irq,IRQP,State,nnT2BR,646&a=0&d=584C0400F8F8&irq0=7&irq1=8&steps=30)

IRQ during JMP Relative (address stored after JMP instruction, to next location after that): [http://visual6502.org/JSSim/expert.html?logmore=Execute,irq,IRQP,State,nnT2BR,646&a=0&d=586C04000600F8F8&irq0=11&irq1=12&steps=30](http://visual6502.org/JSSim/expert.html?logmore=Execute,irq,IRQP,State,nnT2BR,646&a=0&d=586C04000600F8F8&irq0=11&irq1=12&steps=30)

RTI - interrupt flag changes *before* poll location: [http://visual6502.org/JSSim/expert.html?logmore=Execute,irq,IRQP,State,nnT2BR,646&a=0&d=40&irq0=9&irq1=10&steps=30](http://visual6502.org/JSSim/expert.html?logmore=Execute,irq,IRQP,State,nnT2BR,646&a=0&d=40&irq0=9&irq1=10&steps=30)

CLI+SEI - interrupt flag changes *after* poll location for both: [http://visual6502.org/JSSim/expert.html?logmore=Execute,irq,IRQP,State,nnT2BR,646&a=0&d=5878&irq0=5&irq1=6&steps=30](http://visual6502.org/JSSim/expert.html?logmore=Execute,irq,IRQP,State,nnT2BR,646&a=0&d=5878&irq0=5&irq1=6&steps=30)

PLP - interrupt flag changes *after* poll location: [http://visual6502.org/JSSim/expert.html?logmore=Execute,irq,IRQP,State,nnT2BR,646&a=0&d=28F8F8&irq0=5&irq1=6&steps=30](http://visual6502.org/JSSim/expert.html?logmore=Execute,irq,IRQP,State,nnT2BR,646&a=0&d=28F8F8&irq0=5&irq1=6&steps=30)

### Interrupt hijacking

<http://en.wikipedia.org/wiki/Interrupts_in_65xx_processors>

BRK hijacked by NMI: [http://visual6502.org/JSSim/expert.html?logmore=Execute,nmi,~NMIG,State,nnT2BR,646&a=0&d=5800&nmi0=11&nmi1=12&steps=30](http://visual6502.org/JSSim/expert.html?logmore=Execute,nmi,~NMIG,State,nnT2BR,646&a=0&d=5800&nmi0=11&nmi1=12&steps=30)

IRQ hijacked by NMI: [http://visual6502.org/JSSim/expert.html?logmore=Execute,irq,IRQP,nmi,~NMIG,State,nnT2BR,646&a=0&d=58F8F8&irq0=5&irq1=6&nmi0=15&nmi1=16&steps=30](http://visual6502.org/JSSim/expert.html?logmore=Execute,irq,IRQP,nmi,~NMIG,State,nnT2BR,646&a=0&d=58F8F8&irq0=5&irq1=6&nmi0=15&nmi1=16&steps=30)

### Reset

Seems to take 1+7 cycles, where the last 7 is the ordinary interrupt sequence. 

[http://visual6502.org/JSSim/expert.html?logmore=Execute,irq,IRQP,nmi,~NMIG,State,nnT2BR,646,Reset0,res&a=0&d=58F8F8F8F8&reset0=5&reset1=6&steps=30](http://visual6502.org/JSSim/expert.html?logmore=Execute,irq,IRQP,nmi,~NMIG,State,nnT2BR,646,Reset0,res&a=0&d=58F8F8F8F8&reset0=5&reset1=6&steps=30)

## M2 duty cycle

The M2 duty cycle is 5/8 - [forum post](http://forums.nesdev.org/viewtopic.php?f=3&t=10029&start=15#p111322) and [CPU pin-out page](CPU_pinout.xhtml "CPU pin out and signal description"). Low for 9/8 PPU cycles, high for 15/8 PPU cycles. Low for 4.5 master cycles, high for 7.5 master cycles. Low for ~210 ns, high for ~349 ns. 

## PPU interface

  * [Wiring diagram](https://www.nesdev.org/w/images/default/f/f3/Neswires.jpg "Neswires.jpg")
  * [Address decoder datasheet](http://www.datasheetcatalog.org/datasheet/motorola/SN54LS139J.pdf)



Input map: 
    
    
    Binding         | Interpretation
    ----------------+--------------------------------------
    0   -> 15       | 0     -> /Enable second demultiplexer
    a15 -> 13       | a15   -> A_1b
    a14 -> 3        | a14   -> A_1a
    a13 -> 2        | a13   -> A_0a
    M2  -> 14       | M2    -> A_0b
    11  -> 1        | /O_1b -> /Enable first demultiplexer
    9   -> /ROM SEL | /O_3b -> /ROM SEL
    5   -> /DBE     | /O_1a -> /DBE
    4   -> RAM CS   | /O_0a -> RAM CS
    

Input/output map: 
    
    
    Inputs   | Outputs
    ---------+----------------
    E  A0 A1 | O_0 O_1 O_2 O_3
    ---------+----------------
    1  x  x  | 1   1   1   1
    0  0  0  | 0   1   1   1
    0  1  0  | 1   0   1   1
    0  0  1  | 1   1   0   1
    0  1  1  | 1   1   1   0
    

Hence: 
    
    
    ROM SEL = M2 && a15
    <Enable first demultiplexer> = M2 && !a15
    DBE = <Enable first demultiplexer> && !a14 && a13
    RAM CS = <Enable first demultiplexer> && !a14 && !a13
    

Summary: 

When M2 high, 
    
    
    A15-13 | Signal
    -------+--------
    1xx    | ROM SEL
    001    | DBE
    000    | RAM CS
    

  * **rw** signal together with ab lines directly generates signals like **/r2002**.
  * **_io_ce** follows _inverted_ **M2**.
  * **vbl_flag** is set via a _vpos was 241 and vpos is not 241_ comparison. Goes high at hpos = 1 during at pclk0.
  * The **set_vbl_flag** signal is high during **pclk0** of **vpos** =241/**hpos** =1.
  * **spr0_hit** goes high at tick x+2, during **pclk1**. Reading uses same **_io_ce** behavior as **vbl_flag**.
  * **spr_overflow** reading behavior just like **spr0_hit**. _TODO:_ set timing.
  * $2005/$2006 and $2000 nametable bits writes use the same timing.



## Timing charts
    
    
    Around VBL flag setting:
    
                   set_vbl_flag signal active during this interval
                                      |
                                     [++]
    Master: 0101010101010101010101010101010101010101010101010101010101010101010101010
    PPU:     [p0][p1][p0][p1][p0][p1][p0][p1][p0][p1][p0][p1][p0][p1][p0][p1][p0][p1]
    CPU:      [  Low  ][     High    ]                                 -> 0, NMI (-8)
    CPU:        [  Low  ][     High    ]                               -> 0, NMI (-7)
    CPU:          [  Low  ][     High    ]                             -> 0, no NMI (-6)
    CPU:            [  Low  ][     High    ]                           -> 0, no NMI (-5)
    CPU:              [  Low  ][     High    ]                         -> 0, no NMI (-4)
    CPU:                [  Low  ][     High    ]                       -> 0, no NMI (-3)
    CPU:                  [  Low  ][     High    ]                     -> 0, no NMI (-2)
    CPU:                    [  Low  ][     High    ]                   -> 0, no NMI (-1)
    CPU:                      [  Low  ][     High    ]                 -> 1, no NMI (0)
    CPU:                        [  Low  ][     High    ]               -> 1, no NMI (1)
    CPU:                          [  Low  ][     High    ]             -> 1, no NMI (2)
    CPU:                            [  Low  ][     High    ]           -> 1, no NMI (3)
    CPU:                              [  Low  ][     High    ]         -> 1, no NMI (4)
    CPU:                                [  Low  ][     High    ]       -> 1, no NMI (5)
    CPU:                                  [  Low  ][     High    ]     -> 1, no NMI (6)
    CPU:                                    [  Low  ][     High    ]   -> 1, NMI (7)
    CPU:                                      [  Low  ][     High    ] -> 1, NMI (8)
    
    (NOTE: Read buffer latches the data, so value when high phase starts matters. Value at end probably matters for the rest.)
    
    Around VBL flag clearing:        cleared during this interval
                                     [+++++++++++...
    Master: 0101010101010101010101010101010101010101010101010101010101010101010101010
    PPU:     [p0][p1][p0][p1][p0][p1][p0][p1][p0][p1][p0][p1][p0][p1][p0][p1][p0][p1]
    CPU:      [  Low  ][     High    ]                                 -> 1
    CPU:        [  Low  ][     High    ]                               -> 1
    CPU:          [  Low  ][     High    ]                             -> 1
    CPU:            [  Low  ][     High    ]                           -> 1
    CPU:              [  Low  ][     High    ]                         -> 1
    CPU:                [  Low  ][     High    ]                       -> 1
    CPU:                  [  Low  ][     High    ]                     -> 1
    CPU:                    [  Low  ][     High    ]                   -> 0 (?)
    CPU:                      [  Low  ][     High    ]                 -> 0
    CPU:                        [  Low  ][     High    ]               -> 0
    CPU:                          [  Low  ][     High    ]             -> 0
    CPU:                            [  Low  ][     High    ]           -> 0
    CPU:                              [  Low  ][     High    ]         -> 0
    CPU:                                [  Low  ][     High    ]       -> 0
    CPU:                                  [  Low  ][     High    ]     -> 0
    CPU:                                    [  Low  ][     High    ]   -> 0
    CPU:                                      [  Low  ][     High    ] -> 0
    
    Sprite zero/overflow flag clearing:      cleared during this interval
                                             [+++++++++++.... 
    Master: 0101010101010101010101010101010101010101010101010101010101010101010101010
    PPU:     [p0][p1][p0][p1][p0][p1][p0][p1][p0][p1][p0][p1][p0][p1][p0][p1][p0][p1]
    CPU:      [  Low  ][     High    ]                                 -> 1
    CPU:        [  Low  ][     High    ]                               -> 1
    CPU:          [  Low  ][     High    ]                             -> 1
    CPU:            [  Low  ][     High    ]                           -> 1
    CPU:              [  Low  ][     High    ]                         -> 0 (?)
    CPU:                [  Low  ][     High    ]                       -> 0
    CPU:                  [  Low  ][     High    ]                     -> 0
    CPU:                    [  Low  ][     High    ]                   -> 0
    CPU:                      [  Low  ][     High    ]                 -> 0
    CPU:                        [  Low  ][     High    ]               -> 0
    CPU:                          [  Low  ][     High    ]             -> 0
    CPU:                            [  Low  ][     High    ]           -> 0
    CPU:                              [  Low  ][     High    ]         -> 0
    CPU:                                [  Low  ][     High    ]       -> 0
                              
                          Pixel location      Flag goes high
    Sprite 0 setting:        --------            -----------------...
    Master: 0101010101010101010101010101010101010101010101010101010101010101010101010
    PPU:     [p0][p1][p0][p1][p0][p1][p0][p1][p0][p1][p0][p1][p0][p1][p0][p1][p0][p1]
    CPU:      [  Low  ][     High    ]                                 -> 0
    CPU:        [  Low  ][     High    ]                               -> 0
    CPU:          [  Low  ][     High    ]                             -> 0
    CPU:            [  Low  ][     High    ]                           -> 0
    CPU:              [  Low  ][     High    ]                         -> 0
    CPU:                [  Low  ][     High    ]                       -> 0
    CPU:                  [  Low  ][     High    ]                     -> 1 (?)
    CPU:                    [  Low  ][     High    ]                   -> 1
    CPU:                      [  Low  ][     High    ]                 -> 1
    CPU:                        [  Low  ][     High    ]               -> 1
    CPU:                          [  Low  ][     High    ]             -> 1
    CPU:                            [  Low  ][     High    ]           -> 1
    CPU:                              [  Low  ][     High    ]         -> 1
    CPU:                                [  Low  ][     High    ]       -> 1
    CPU:                                  [  Low  ][     High    ]     -> 1
    CPU:                                    [  Low  ][     High    ]   -> 1
    CPU:                                      [  Low  ][     High    ] -> 1
    

# [Sprite overflow bug](PPU_sprite_evaluation.xhtml "PPU sprite evaluation")

Appears to be caused by a timing issue. 

Signal trace for a "good" sprite skip (eight sprites not found yet, current sprite not in range): 
    
    
    hpos  spr_addr_clear_low_bump_high_setup  spr_addr_load_next_value  /spr_load_next_value_or_write_2003_reg
    04c   0                                   0                         1
    04c   0                                   0                         1
    04c   0                                   0                         1
    04c   0                                   0                         1
    04c   0                                   0                         1
    04c   0                                   0                         1
    04c   0                                   0                         1
    04c   0                                   0                         1
    04b   1                                   0                         1
    04b   1                                   0                         1
    04b   1                                   0                         1
    04b   1                                   0                         1
    04b   1-----------------------------------1                         0
    04b   1             Overlap               1                         0
    04b   1   Clears low and bumps high       1                         0
    04b   1-----------------------------------1                         0
    04a   1                                   0                         1
    04a   1                                   0                         1
    04a   1                                   0                         1
    04a   1                                   0                         1
    04a   1                                   0                         1
    04a   1                                   0                         1
    04a   1                                   0                         1
    04a   1                                   0                         1
    049   0                                   0                         1
    049   0                                   0                         1
    049   0                                   0                         1
    049   0                                   0                         1
    049   0                                   1                         0
    049   0                                   1                         0
    049   0                                   1                         0
    049   0                                   1                         0
    048   0                                   0                         1
    

Signal trace for a glitchy sprite skip (eight sprites found, current sprite not in range): 
    
    
    hpos  spr_addr_clear_low_bump_high_setup  spr_addr_load_next_value  /spr_load_next_value_or_write_2003_reg
    08e   1                                   0                         1
    08e   1                                   0                         1
    08e   1                                   0                         1
    08e   1                                   0                         1
    08e   1                                   0                         1
    08e   1                                   0                         1
    08e   1                                   0                         1
    08e   1                                   0                         1
    08d   0                                   0                         1
    08d   0                                   0                         1
    08d   0                                   0                         1
    08d   0                                   0                         1
    08d   0-----------------------------------1                         0
    08d   0           No overlap              1                         0
    08d   0       Bumps low and high          1                         0
    08d   0-----------------------------------1                         0
    08c   1                                   0                         1
    08c   1                                   0                         1
    08c   1                                   0                         1
    08c   1                                   0                         1
    08c   1                                   0                         1
    08c   1                                   0                         1
    08c   1                                   0                         1
    08c   1                                   0                         1
    08b   0                                   0                         1
    08b   0                                   0                         1
    08b   0                                   0                         1
    08b   0                                   0                         1
    08b   0                                   1                         0
    08b   0                                   1                         0
    08b   0                                   1                         0
    08b   0                                   1                         0
    08a   1                                   0                         1
    
