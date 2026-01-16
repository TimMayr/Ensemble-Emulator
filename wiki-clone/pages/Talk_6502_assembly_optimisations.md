# Talk:6502 assembly optimisations

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3A6502_assembly_optimisations) | View [other pages](Special_AllPages.xhtml#Talk_6502_assembly_optimisations)

### Examples

    **Split word tables in high and low components**
    This optimisation is not human friendly, makes the source code much bigger, but still makes the compiled size smaller and faster:

One doesn't compile assembly language programs. They are assembled. Please use correct terminology so those of us who know the difference between assembling and compiling programs don't dismiss this article as the work of a dilettante. 

_`PointerTable .dw Pointer1, Pointer2, ....`_

The `.dw` pseudo-op is non-standard syntax that should not be used in code examples. Best if examples stick to official MOS Technology syntax and pseudo-ops, such as: 

`pointertable .byte pointer1, pointer2, ...` ([unsigned](https://en.wikipedia.org/wiki/Wikipedia:Signatures "wikipedia:Wikipedia:Signatures") post by ~~38.69.12.5~~)

    With respect to "compile" vs. "assemble" pedantry: The distinction between assembling and compiling becomes blurred when macros (in assembly language) or blocks of inline assembly language (in a language that is not primarily assembly language) come into play. Besides, we use "compile" because among words meaning "translate source code into object code", I assume it will be understood by the largest number of readers. What better word encompasses both "compile" and "assemble" and means "perform a translation of source code, in either assembly language or a language that is not assembly language, into object code"?
    With respect to syntax: What assembler distributed under a [free software license](https://en.wikipedia.org/wiki/free_software_license "wikipedia:free software license") should we cite as an example of an assembler that accepts official MOS Technology syntax and pseudo-ops? --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 21:18, 7 June 2013 (MDT)

## RTS Trick cycle use

Denine [pointed out a perceived miscategorization of the RTS trick](http://forums.nesdev.org/viewtopic.php?p=179912#p179912), but its categorization could be more complex than the forum post suggests. 

There are three options. The base case is a jump target variable outside zero page (which on the NES means $0100-$07FF or $6000-$7FFF). Here, I use `a:`, which is ca65's notation for forcing a 2-byte address: 
    
    
    lda table+1,x       ; 3b 4c
    sta a:jumptarget+1  ; 3b 4c
    lda table,x         ; 3b 4c
    sta a:jumptarget    ; 3b 4c
    jmp (jumptarget)    ; 3b 5c
    

If the jump target variable is in $0000-$00FF, 2 bytes and 2 cycles are saved: 
    
    
    lda table+1,x       ; 3b 4c
    sta z:jumptarget+1  ; 2b 3c
    lda table,x         ; 3b 4c
    sta z:jumptarget    ; 2b 3c
    jmp (jumptarget)    ; 3b 5c
    

Putting the jump target variable on the stack (the RTS trick) saves 6 bytes and 1 cycle compared to a jump target variable outside zero page, and it saves 4 bytes but costs 1 cycle compared to a jump target variable within zero page. 
    
    
    lda table+1,x     ; 3b 4c
    pha               ; 1b 3c
    lda table,x       ; 3b 4c
    pha               ; 1b 3c
    rts               ; 1b 6c
    

In addition, unlike the indirect jump, the RTS trick is reentrant. This reentrancy can be considered a RAM saving because the NMI and IRQ handlers don't need separate jump target variables permanently allocated to them. Instead it uses two bytes of stack for a handful of cycles, which one can amortize in the program's overall stack use. So is this a saving, or is it a size-for-speed tradeoff? --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 09:43, 21 September 2016 (MDT) 
