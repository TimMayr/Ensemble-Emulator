# Jump table

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Jump_table) | View [other pages](Special_AllPages.xhtml#Jump_table)

A jump table is a table of code addresses, meant to be indexed by a selector value. The program uses the selector to look up an address in the table, then jumps to that address. 

The alternative to a jump table is a long string of comparisons with each possible selector value. This approach is tedious to set up and slow in comparison to jump tables. 

Jump tables are similar to "switch" statements found in other programming languages. 

## Indirect jumping

The NES supports JMP (addr), an indirect jump instruction, so a jump table can be implemented by copying the address to a temporary variable and jumping to it: 
    
    
    ; Jumps to the subroutine indexed by 'A'.
    do_action:
           asl
           tax
           lda table,x
           sta ptr
           lda table+1,x
           sta ptr+1
           jmp (ptr)
    

While there is no indirect version of JSR, the behavior can be imitated by combining regular JSR with JMP (addr): 
    
    
    do_action:
           asl
           tax
           lda table,x
           sta ptr
           lda table+1,x
           sta ptr+1
           jsr callSubroutineInPtr
           ; Do other stuff here once the called subroutine returns.
           rts
    
    callSubroutineInPtr:
           jmp (ptr)
    

Two things to ensure: 

  * _ptr_ must not lie on the edge of a page boundary (`$xxFF`), as a [bug in the original 6502](Errata.xhtml#CPU "Errata") prevents it from being fetched properly. This is easy to avoid, especially if _ptr_ is on the zero-page, but most assemblers should at least have a warning to catch the accidental case.
  * _ptr_ must only be used by a single thread. If you need a jump table in both your main thread code, and within an interrupt/NMI, a separate variable must be used for the interrupt thread to prevent conflicting use.



A stack-based alternative can avoid the need to use a _ptr_ variable, at the expense of 1 extra cycle for RTS vs JMP (assuming _ptr_ was zero-page). See below. 

## Stack-based dispatch

    _Main article:[RTS Trick](RTS_Trick.xhtml "RTS Trick")_

Like JMP (addr), the RTS and RTI instructions also perform indirect jumps. Rather than jumping to a pointer variable stored in zero page memory, RTS and RTI jump to the address on top of the stack. 

To use RTI for indirect jumps, first push the address and then push the processor flags. Executing RTI will pop these values and jump. 
    
    
    do_action:
           asl a
           tax
           lda table+1,x ; high byte first
           pha
           lda table,x
           pha
           php ; RTI expects processor flags on top.
           rti
    

RTS is slightly more tricky, because it adds one to the address it pulls from the stack. This requires that every entry in the jump table have one subtracted from it. This could be done by the code, but it's tedious because the low byte must be decremented first, while the high byte needs to be pushed first. Thus, it's preferable to simply subtract one from each entry in the assembly source text: 
    
    
    do_action:
           asl a
           tax
           lda table+1,x
           pha
           lda table,x
           pha
           rts
    
    table:
           .word action0-1, action1-1, action2-1 ; ...
    

The benefit of the RTS version is that it's three clock cycles faster than the RTI version, due to not having to push the flags. The disadvantage is that you must adjust every table entry by -1. 

## Split Tables

The previous examples have used a single table storing two-byte addresses, but on the 6502 it is slightly more efficient to split the table into a table of low bytes and a table of high bytes: 
    
    
    table_lo:
        .byt .lobyte(addr1)
        .byt .lobyte(addr2)
        .byt .lobyte(addr3)
    table_hi:
        .byt .hibyte(addr1)
        .byt .hibyte(addr2)
        .byt .hibyte(addr3)
    
    ; Jumps to the subroutine indexed by 'A'.
    do_action:
           tax
           lda table_lo,x
           sta ptr
           lda table_hi,x
           sta ptr+1
           jmp (ptr)
    

256 addresses can be contained in both tables this way as opposed to 128 using a single table. 
