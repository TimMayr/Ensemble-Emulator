# 6502 cycle times

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/6502_cycle_times) | View [other pages](Special_AllPages.xhtml#6502_cycle_times)

I put this spreadsheet together because I didn't like the way the timing information was organized in other places. Some mnemonic/addressing mode combinations do not have an opcode and are left blank because they are not legal instructions. I may add the illegal opcodes later. Cells marked with "+" mean add one cycle if a page boundary is crossed. 

Mnemonic  | Description  | IMP  | IMM  | ZP  | ZP,X  | ZP,Y  | ABS  | ABS,X  | ABS,Y  | IND  | IND,X  | IND,Y  | ACC   
---|---|---|---|---|---|---|---|---|---|---|---|---|---  
ADC  | ADd with Carry  |  | 2  | 3  | 4  |  | 4  | 4+  | 4+  |  | 6  | 5+  |   
AND  | bitwise AND with accumulator  |  | 2  | 3  | 4  |  | 4  | 4+  | 4+  |  | 6  | 5+  |   
ASL  | Arithmetic Shift Left  |  |  | 5  | 6  |  | 6  | 7  |  |  |  |  |   
BIT  | test BITs  |  |  | 3  |  |  | 4  |  |  |  |  |  |   
BRK  | BreaK  | 7  |  |  |  |  |  |  |  |  |  |  |   
CMP  | CoMPare accumulator  |  | 2  | 3  | 4  |  | 4  | 4+  | 4+  |  | 6  | 5+  |   
CPX  | ComPare X register  |  | 2  | 3  |  |  | 4  |  |  |  |  |  |   
CPY  | ComPare Y register  |  | 2  | 3  |  |  | 4  |  |  |  |  |  |   
DEC  | DECrement memory  |  |  | 5  | 6  |  | 6  | 7  |  |  |  |  |   
EOR  | bitwise Exclusive OR  |  | 2  | 3  | 4  |  | 4  | 4+  | 4+  |  | 6  | 5+  |   
INC  | INCrement memory  |  |  | 5  | 6  |  | 6  | 7  |  |  |  |  |   
JMP  | JuMP  |  |  |  |  |  | 3  |  |  | 5  |  |  |   
JSR  | Jump to SubRoutine  |  |  |  |  |  | 6  |  |  |  |  |  |   
LDA  | LoaD Accumulator  |  | 2  | 3  | 4  |  | 4  | 4+  | 4+  |  | 6  | 5+  |   
LDX  | LoaD X register  |  | 2  | 3  |  | 4  | 4  |  | 4+  |  |  |  |   
LDY  | LoaD Y register  |  | 2  | 3  | 4  |  | 4  | 4+  |  |  |  |  |   
LSR  | Logical Shift Right  |  |  | 5  | 6  |  | 6  | 7  |  |  |  |  | 2   
NOP  | No OPeration  | 2  |  |  |  |  |  |  |  |  |  |  |   
ORA  | bitwise OR with Accumulator  |  | 2  | 3  | 4  |  | 4  | 4+  | 4+  |  | 6  | 5+  |   
ROL  | Rotate Left  |  |  | 5  | 6  |  | 6  | 7  |  |  |  |  | 2   
ROR  | Rotate Right  |  |  | 5  | 6  |  | 6  | 7  |  |  |  |  | 2   
RTI  | ReTurn from Interrupt  | 6  |  |  |  |  |  |  |  |  |  |  |   
RTS  | ReTurn from Subroutine  | 6  |  |  |  |  |  |  |  |  |  |  |   
SBC  | SuBtract with Carry  |  | 2  | 3  | 4  |  | 4  | 4+  | 4+  |  | 6  | 5+  |   
STA  | Store Accumulator  |  |  | 3  | 4  |  | 4  | 5  | 5  |  | 6  | 6  |   
STX  | Store X register  |  |  | 3  |  | 4  | 4  |  |  |  |  |  |   
STY  | Store Y register  |  |  | 3  | 4  |  | 4  |  |  |  |  |  |   
TAX  | Transfer A to X  | 2  |  |  |  |  |  |  |  |  |  |  |   
TXA  | Transfer X to A  | 2  |  |  |  |  |  |  |  |  |  |  |   
DEX  | DEcrement X  | 2  |  |  |  |  |  |  |  |  |  |  |   
INX  | INcrement X  | 2  |  |  |  |  |  |  |  |  |  |  |   
TAY  | Transfer A to Y  | 2  |  |  |  |  |  |  |  |  |  |  |   
TYA  | Transfer Y to A  | 2  |  |  |  |  |  |  |  |  |  |  |   
DEY  | Decrement Y  | 2  |  |  |  |  |  |  |  |  |  |  |   
INY  | Increment Y  | 2  |  |  |  |  |  |  |  |  |  |  |   
CLC  | CLear Carry  | 2  |  |  |  |  |  |  |  |  |  |  |   
SEC  | SEt Carry  | 2  |  |  |  |  |  |  |  |  |  |  |   
CLI  | CLear Interrupt  | 2  |  |  |  |  |  |  |  |  |  |  |   
SEI  | SEt Interrupt  | 2  |  |  |  |  |  |  |  |  |  |  |   
CLV  | CLear oVerflow  | 2  |  |  |  |  |  |  |  |  |  |  |   
CLD  | CLear Decimal  | 2  |  |  |  |  |  |  |  |  |  |  |   
SED  | SEt Decimal  | 2  |  |  |  |  |  |  |  |  |  |  | 
