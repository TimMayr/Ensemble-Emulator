# FCEUX debugger

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/FCEUX_debugger) | View [other pages](Special_AllPages.xhtml#FCEUX_debugger)

Tips for using the [FCEUX](http://www.fceux.com) debugger. 

## Contents

  * 1 Memory
  * 2 Inline assembler
  * 3 Breakpoints
  * 4 Conditional breakpoints
    * 4.1 Example conditions
    * 4.2 A story
  * 5 References



## Memory

The memory pane displays the current contents of the NES' RAM and ROM. The following memory ranges may display useful data for inspection: 
    
    
    0000-00FF - zero page
    0100-01FF - stack
    0200-07FF - ram
    4018-FFFF - mapper controlled
    

Memory contents are displayed in this form: 
    
    
    0F:C0A8:24 1F     BIT $001F = #$80
    bb:mmmm:dd dd dd  iiiiiiiiiiiiiiii
    
    
    
    bb   - 16k iNES bank, designates which 16k bank from the iNES file is mapped here
    mmmm - physical address on the NES CPU data bus
    dd   - data bytes belonging to the instruction beginning at this address
    iiii - description of the instruction
    

An instruction description with "= #$xx" at the end indicates the value currently in memory at the address referenced by the instruction (before that instruction executes). 

When debugging an NSF program, the bank designation will be a 4k NSF bank instead of the 16k iNES bank. 

There is an empty column to the left of the memory view. Hovering the mouse here will display at the bottom of the window more detailed information about the location of this code in the iNES file. Left clicking on this column will open the inline assembler, which allows you to patch the ROM at runtime. Right clicking on this column will open the hex editor, which allows you to directly edit the ROM. 

## Inline assembler

Open the inline assembler by clicking in the empty column to the left of the memory view. 

The starting memory address is displayed in the PC field at the top of the inline assembler window. Type a line of assembly to add in the empty field just below this, and hit enter. The assembled code of your patch will appear line by line below. 

Click apply to apply your patch to the ROM in memory. Click undo to remove the last assembled line. After applying a patch, clicking Save will allow you to save this patch directly to the ROM file. 

## Breakpoints

To create a breakpoint, click the Add button in the BreakPoints frame in the upper right corner of the debugger. 

Each breakpoint has an address range to watch (use only the left address field if you wish to watch a single byte address). Check one or more of the options to watch for Read, Write, or Execute at an address. Note that fetching of code from an address will not break as a Read; use the Execute box for this. 

Double click on a breakpoint in the BreakPoints frame to quickly disable or enable it. 

The Forbid option creates an anti-breakpoint that will prevent the specified address from triggering any of the other breakpoints. These can also be enabled and disabled as needed. 

Breakpoints are listed in the following form: 
    
    
    aaaa: EmRWXF : nnnn
    aaaa-aaaa: EmRWXF : nnnn
    
    
    
    aaaa - address of breakpoint
    E    - enabled
    m    - memory area C = CPU, P = PPU, S = sprite
    R    - read
    W    - write
    X    - execute
    F    - forbid
    nnnn - name of breakpoint
    

When entering the address of a breakpoint, a small number of convenient strings may be used instead of the hexadecimal memory address: 

NES special addresses: 

  * NMI/VBL - non-maskable interrupt vector at FFFA
  * RST - reset vector at FFFC
  * IRQ - interrupt vector at FFFE



FDS special addresses: 

  * NMI1 - non-maskable interrupt vector at DFF6
  * NMI2 - non-maskable interrupt vector at DFF8
  * NMI3 - non-maskable interrupt vector at DFFA
  * RST - reset vector at DFFC
  * IRQ - interrupt vector at DFFE



NSF special addresses: 

  * LOAD - NSF LOAD address
  * INIT - NSF INIT address
  * PLAY - NSF PLAY address



## Conditional breakpoints

Breakpoints may have a conditional statement that cause them to execute only if that statement evaluates to true. They can be used as runtime assertions: put the opposite of your assertion in the condition, and it will trigger only when your assertion fails. The conditional breakpoint grammar has this form: 
    
    
    * P         -> Connect
    * Connect   -> Compare {('||' | '&&') Compare}
    * Compare   -> Sum {('==' | '!=' | '<=' | '>=' | '<' | '>') Sum}
    * Sum       -> Product {('+' | '-') Product}
    * Product   -> Primitive {('*' | '/') Primitive}
    * Primitive -> Number | Address | Register | Flag | PC Bank | '(' Connect ')'
    * Number    -> '#' [1-9A-F]*
    * Address   -> '$' [1-9A-F]* | '$' '[' Connect ']'
    * Register  -> 'A' | 'X' | 'Y' | 'P'
    * Flag      -> 'N' | 'C' | 'Z' | 'I' | 'B' | 'V'
    * PC Bank   -> 'K'
    

The parser is very strict. All numbers are hexadecimal. Always prefix a number with # for an immediate value, or $ for a memory address. If a memory address needs to be calculated, use $[] with the calculation inside the brackets. After entering a condition in the condition field, you should click Ok and then reopen the breakpoint with the Edit button. If there was a problem parsing your condition, it will have been erased. 

### Example conditions

break if register A is less than value at memory address $0005: 
    
    
    A < $0005
    

break if the program counter is 8123: 
    
    
    P == #8123
    

break if the value at the indirect address on zeropage $10 is not equal to FF: 
    
    
    #FF != $[$10+($11*#100)]
    

break if flag N is clear or A is not equal to 00: 
    
    
    (N==#0 || A!=#0)
    

### A story

Once upon a time, an NES programmer was trying to determine where in the code a surprising value was being written to a specific memory location. Specifically, when the value $EE was being written to location $0028. To figure out what can be put into the "condition" text input field in the breakpoint dialog box, they dug into the source code for FCEUX and found [the parser](http://fceux.sourcearchive.com/documentation/2.1.4aplus-prepack-0ubuntu1/conddebug_8cpp-source.html), whose grammar is listed near the top of the source file. 

After the programmer had been staring at the problem for too long, a fox came and helped them. 

  1. Address: `28`
  2. Click "Write"
  3. Memory: `CPU` (the default)
  4. Condition: `A == #ee || X == #ee || Y == #ee` if you don't know what CPU register is used for the write or `X == #ee` if you know it's X. The # is important; leaving it out will screw it up.



## References

  * [FCEUX online debug documentation](http://fceux.com/web/help/Debug.html)
  * [FCEUX debugger usage guide](http://fceux.com/web/help/Debugger.html)
  * [Forum post about conditional breakpoints](http://forums.nesdev.org/viewtopic.php?p=75485#p75485)


