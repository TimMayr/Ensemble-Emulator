# Visual6502wiki/6502 datapath

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Visual6502wiki/6502_datapath) | View [other pages](Special_AllPages.xhtml#Visual6502wiki_6502_datapath)

This page discusses the 6502 datapath, using the terminology from [Visual6502wiki/Hanson's Block Diagram](Visual6502wiki_Hanson_s_Block_Diagram.xhtml "Visual6502wiki/Hanson's Block Diagram") and is probably best understood by [reference to it](http://www.pagetable.com/?p=39)

We're interested in which datapath control signals are active in each of the two phases. 

A full cycle consists of phi1 and phi2. When we say a signal is "effective", we mean it actually does something. 

All datapath control signals are latched during phi2; they are set mostly from opcode and timing data, but also some internal state. We work broadly from left to right. (Which is right to left on [Balazs' schematic](Visual6502wiki_Balazs__schematic_and_documents.xhtml "Visual6502wiki/Balazs' schematic and documents")) 

## Contents

  * 1 External busses and signals
  * 2 Address values
  * 3 The register file
  * 4 ALU inputs
  * 5 ALU operation selection
  * 6 ALU output register
  * 7 The Program Counter
  * 8 Precharge
  * 9 A note on signal naming



### External busses and signals

DOR is latched from DB during phi1, and driven onto the data pins in phi2, if a write is done (and, on the 6501, only if the asynchronous DBE signal is on). 

DL is latched during phi2, and then put on ADL, ADH, or DB on the next phi1; during phi2, the old value in DL is put on that bus. 

ABL and ABH can be loaded from ADL and ADH respectively during phi1; they are put on the address pins in that same phi1, and stay there until changed again. 

R/#W is latched during phi2, and then delayed until phi1, where it is output. 

### Address values

ADL/ABL, ADH/ABH
    
    We already saw these. Effective on the next phi1.

0/ADL0, 0/ADL1, 0/ADL2, 0/ADH0, 0/ADH(1-7)
    These set the interrupt vector fetch address, and the zero page and stack high address. Effective on phi2 and the next phi1.

  


### The register file

Y/SB, X/SB, SB/Y, SB/X
    Move the X and Y registers from/to the SB. Latched on phi2, just like everything else; effective on the next phi1.

SB/S, S/S, effective on the next phi1.
S/SB, S/ADL, effective on phi2 and the next phi1.
    The S register is actually two latches in series. This makes it possible to read a value from SB and write a value to ADL at the same time. On phi2, the value from the "in" latch is forwarded to the "out" latch (and onto the driven bus, if any).

(Note the [two "tuning fork" structures](Visual6502wiki_6502_datapath_control_timing_fix.xhtml "Visual6502wiki/6502 datapath control timing fix"), which have contacts either on the top or bottom, which select whether X, Y, A write SB and DB only during phi1, or slightly longer, during "not phi2". We think this might be a timing fix, or an option left open until after silicon showed which choice worked best) 

### ALU inputs

SB/ADD, 0/ADD, nDB/ADD, DB/ADD, ADL/ADD
    Two options for one side, three for the other. Effective on the next phi1.

### ALU operation selection

ANDS, EORS, ORS, 1/ADDC, SRS, SUMS, DAA, DSA
    Select the ALU operation. Effective on the next phi1 and phi2.

(The overflow and carry out signals AVR and ACR are output from the ALU back to the control logic, latched during phi2, used in phi1. The decimal carries are picked up at phi2 as well). 

### ALU output register

The ALU output register (ADD) is written during phi2. The value can be used the next cycle: 

ADD/SB7, ADD/SB(0-6), ADD/ADL, effective on phi2 and the next phi1.
    The ADL output is for address calculations. For output to SB, the top bit is handled separately for rotate right instructions: the ALU always computes a zero there; by not driving it to the bus a one will be read.

SB/AC, effective on the next phi1.
    Lines 1-3,5-7 are fed through the decimal adjust first, to finish the proper BCD add/subtract result if necessary, before writing it to the accumulator.

AC/SB, AC/DB, effective on the next phi1.
    Write the A reg back to one of the busses.

### The Program Counter

ADH/PCH, PCH/PCH, PCL/PCL, ADL/PCL
    select whether to use the current PC, or take a new value from the internal address busses. Effective on the next phi1.

PCH/DB, PCL/DB, PCH/ADH, PCL/ADL
    write the PC to one of the busses. Effective on phi2 and the next phi1.

I/PC, effective during the next phi1 and phi2.
    Increment the PC, or not. When incrementing, the new value is put on ADL,ADH because there are no internal latches in the PC incrementer. For every instruction, the first two bytes are fetched (during execution of the previous instruction); I/PC peeks ahead (or back, if you want to look at it that way) to the next instruction that is predecoded, so it can skip incrementing PC if that is a one-byte instruction.

P/DB
    Write the flag values to the DB; effective on phi2 and the next phi1. The DB can be read to set the flag values as well; it is read during phi2, and then latched in the flag register on the next phi1.

SB/DB, SB/ADH
    Connect two busses together. Effective on phi2 and the next phi1.

### Precharge

All internal busses (SB, DB, ADL, ADH) are driven high during phi2, as a sort of precharge. In fact commonly they are also driven by data signals during phi2, causing an intermediate voltage to appear on the bus. 

### A note on signal naming

In our [Javascript simulation](http://visual6502.org/JSSim/expert.html?graphics=f&logmore=DPControl&steps=10) the datapath control signals are tabulated according to Hanson's names, but [in the layout](http://visual6502.org/JSSim/expert.html?nosim=t&find=dpc4_SSB,dpc5_SADL,dpc6_SBS,dpc7_SS&panx=166.0&pany=357.3&zoom=6.2) they are named with a prefix according to their position across the chip. So 

  * SSB, SADL, SBS, SS



will be found as 

  * dpc4_SSB,dpc5_SADL,dpc6_SBS,dpc7_SS



in [the source](https://github.com/trebonian/visual6502/blob/master/nodenames.js). See also the table below. 

As Balazs used another naming scheme in his very useful but incomplete schematic, we should also cross-reference his names: 

Balazs | Hanson | JSSim | note   
---|---|---|---  
R1x7  | Y/SB  | dpc0_YSB | drive sb from y   
R1x6  | SB/Y  | dpc1_SBY | load y from sb   
R1x5  | X/SB  | dpc2_XSB | drive sb from x   
R1x4  | SB/X  | dpc3_SBX | load x from sb   
R1x2  | S/SB  | dpc4_SSB | drive sb from stack pointer   
R1x1  | S/ADL  | dpc5_SADL | drive adl from stack pointer   
R1x3  | SB/S  | dpc6_SBS | load stack pointer from sb   
?  | S/S  | dpc7_SS | recirculate stack pointer   
R2x1  | notDB/ADD  | dpc8_nDBADD | alu b side: select not-idb input   
R2x2  | DB/ADD  | dpc9_DBADD | alu b side: select idb input   
R2x3  | ADL/ADD  | dpc10_ADLADD | alu b side: select adl input   
R2x4 (??)  | SB/ADD  | dpc11_SBADD | alu a side: select sb   
R2x5  | 0/ADD  | dpc12_0ADD | alu a side: select zero   
R2x6  | ORS  | dpc13_ORS | alu op: a or b   
R2x7  | SRS  | dpc14_SRS | alu op: logical right shift   
R2x8  | ANDS  | dpc15_ANDS | alu op: a and b   
R2x9  | EORS  | dpc16_EORS | alu op: a xor b (?)   
R2x12  | SUMS  | dpc17_SUMS | alu op: a plus b (?)   
?  | DAA  | dpc18_#DAA | decimal related (inverted)   
R2x14,7  | ADD/SB(7)  | dpc19_ADDSB7 | alu to sb bit 7 only   
R2x14  | ADD/SB(0-6)  | dpc20_ADDSB06 | alu to sb bits 6-0 only   
R2x15  | ADD/ADL  | dpc21_ADDADL | alu to adl   
R2x20,6  | DSA  | dpc22_#DSA | decimal related/SBC only (inverted)   
R3x4  | SB/AC  | dpc23_SBAC | (optionally decimal-adjusted) sb to acc   
R3x1  | AC/SB  | dpc24_ACSB | acc to sb   
R3x3  | SB/DB  | dpc25_SBDB | sb pass-connects to idb (bi-directionally)   
R3x2  | AC/DB  | dpc26_ACDB | acc to idb   
R3x0  | SB/ADH  | dpc27_SBADH | sb pass-connects to adh (bi-directionally)   
R3x5,0  | 0/ADH0  | dpc28_0ADH0 | zero to adh0 bit0 only   
R3x5  | 0/ADH(1-7)  | dpc29_0ADH17 | zero to adh bits 7-1 only   
R4x2  | ADH/PCH  | dpc30_ADHPCH | load pch from adh   
R4x3  | PCH/PCH  | dpc31_PCHPCH | load pch from pch incremented   
R4x4  | PCH/ADH  | dpc32_PCHADH | drive adh from pch incremented   
R4x1  | PCH/DB  | dpc33_PCHDB | drive idb from pch incremented   
!!  | PCLC  | dpc34_PCLC | pch carry in and pcl FF detect?   
Carry  | PCHC  | dpc35_PCHC | pcl 0x?F detect - half-carry   
notCarry  | I/PC  | dpc36_#IPC | pcl carry in (inverted)   
R5x1  | PCL/DB  | dpc37_PCLDB | drive idb from pcl incremented   
R5x4  | PCL/ADL  | dpc38_PCLADL | drive adl from pcl incremented   
R5x3  | PCL/PCL  | dpc39_PCLPCL | load pcl from pcl incremented   
R5x2  | ADL/PCL  | dpc40_ADLPCL | load pcl from adl   
Dkx2  | DL/ADL  | dpc41_DL/ADL | pass-connect adl to mux node driven by idl   
Dkx3  | DL/ADH  | dpc42_DL/ADH | pass-connect adh to mux node driven by idl   
Dkx1  | DL/DB  | dpc43_DL/DB | pass-connect idb to mux node driven by idl 
