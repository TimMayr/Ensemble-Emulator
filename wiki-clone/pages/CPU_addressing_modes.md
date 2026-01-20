# CPU addressing modes

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/CPU_addressing_modes) | View [other pages](Special_AllPages.xhtml#CPU_addressing_modes)

The NES CPU is a second-source version of MOS Technology 6502, manufactured by Ricoh. Addressing modes and instruction timings are the same as those in the standard 6502. 

This page summarizes the 6502 addressing modes and explains some cases where certain modes might be useful. 

## Indexed addressing

Indexed addressing modes use the X or Y register to help determine the address. The 6502 has six main indexed addressing modes: 

Abbr | Name | Formula | Cycles   
---|---|---|---  
d,x | Zero page indexed | val = PEEK((arg + X) % 256) | 4   
d,y | Zero page indexed | val = PEEK((arg + Y) % 256) | 4   
a,x | Absolute indexed | val = PEEK(arg + X) | 4+   
a,y | Absolute indexed | val = PEEK(arg + Y) | 4+   
(d,x) | Indexed indirect | val = PEEK(PEEK((arg + X) % 256) + PEEK((arg + X + 1) % 256) * 256) | 6   
(d),y | Indirect indexed | val = PEEK(PEEK(arg) + PEEK((arg + 1) % 256) * 256 + Y) | 5+   
  
Notes: 

  * Abbreviations for addressing modes are those used in WDC's 65C816 data sheets.
  * \+ means add a cycle for write instructions or for page wrapping on read instructions, called the "oops" cycle below.



The 6502 has one 8-bit ALU and one 16-bit upcounter (for PC). To calculate a,x or a,y addressing in an instruction other than sta, stx, or sty, it uses the 8-bit ALU to first calculate the low byte while it fetches the high byte. If there's a carry out, it goes "oops", applies the carry using the ALU, and repeats the read at the correct address. Store instructions always have this "oops" cycle: the CPU first reads from the partially added address and then writes to the correct address. The same thing happens on (d),y indirect addressing. 

The (d),y mode is used far more often than (d,x). The latter implies a table of addresses on zero page. On computers like the Apple II, where the BASIC interpreter uses up most of zero page, (d,x) is rarely used. But the NES has far more spare room in zero page, and music engine might use X = 0, 4, 8, or 12 to match [APU](APU.xhtml "APU") register offsets. 

## Other addressing

Abbr | Name | Notes   
---|---|---  
| Implicit | Instructions like _RTS_ or _CLC_ have no address operand, the destination of results are implied.   
A | Accumulator | Many instructions can operate on the accumulator, e.g. _LSR A_. Some assemblers will treat no operand as an implicit _A_ where applicable.   
#v | Immediate | Uses the 8-bit operand itself as the value for the operation, rather than fetching a value from a memory address.   
d | Zero page | Fetches the value from an 8-bit address on the zero page.   
a | Absolute | Fetches the value from a 16-bit address anywhere in memory.   
label | Relative | Branch instructions (e.g. _BEQ_ , _BCS_) have a relative addressing mode that specifies an 8-bit signed offset relative to the current PC.   
(a) | Indirect | The _JMP_ instruction has a special indirect addressing mode that can jump to the address stored in a 16-bit pointer anywhere in memory. 
