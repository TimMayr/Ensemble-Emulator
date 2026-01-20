# User:Zzo38/6502 programming tricks

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AZzo38/6502_programming_tricks) | View [other pages](Special_AllPages.xhtml#User_Zzo38_6502_programming_tricks)

## Contents

  * 1 Toggle the zero flag
  * 2 Rotate accumulator left 8-bits
  * 3 Signed compare
  * 4 Tail recursion
  * 5 Fall-through tail recursion
  * 6 Compressed data in CHR ROM
  * 7 Storing high byte and low byte of an address in separate tables
  * 8 Dealing with carry flag
  * 9 Clear accumulator and carry flag and reset bank
  * 10 Self-modifying codes
  * 11 See also



## Toggle the zero flag
    
    
    PHP
    PLA
    AND #2
    

  * Bytes: 4
  * Cycles: 9



Main-effect: 

  * Zero flag: toggled.



Side-effect: 

  * Accumulator: set to 0 or 2.
  * Negative flag: cleared.



## Rotate accumulator left 8-bits
    
    
    ASL A
    ADC #0
    

  * Bytes: 3
  * Cycles: 4



Main-effect: 

  * Accumulator: shifted left, with shifted-out bit shifted-in.



Side-effect: 

  * Carry flag: cleared.



## Signed compare

One way to do this is by toggling the high bit of both numbers before comparison. For example, if 8-bit numbers: 
    
    
    EOR #$80
    

## Tail recursion

  * Bytes: -1
  * Cycles: -9



This can be done by replacing a JSR/RTS with just a JMP, since the subroutine it jumps to will have its own RTS. 

## Fall-through tail recursion

  * Bytes: -4
  * Cycles: -12



Instead of jumping to another subroutine, you can just fall-through into it. In fact you can even JSR to it and then fall through to it (possibly doing something else in between if wanted), if you want to run it twice; _Super Mario Bros._ uses this pattern. 

Note: This should be done itself inside of a subroutine which finally does the same effect as another, so it still needs to be called. 

## Compressed data in CHR ROM

Compressed level data may be stored in CHR ROM, rather than PRG ROM, if it helps. One thing that can help is the automatic address increment in the PPU, and faster addressing modes. The data can start at an arbitrary address without much difficulty. 

## Storing high byte and low byte of an address in separate tables

For the RTS trick and other things, you can use separate 256 byte pages for different pieces of a record, such as low and high half of an address. 

Here is an example of a macro for use with [Unofficial MagicKit](https://www.nesdev.org/w/index.php?title=Unofficial_MagicKit&action=edit&redlink=1 "Unofficial MagicKit \(page does not exist\)") which will automatically do this for you (here, it is called `player_tile`, and uses the tables `pmrttl` and `pmrtth`; you can, in fact, use exactly this macro and just change the names around): 
    
    
    	macro player_tile
    	if ?X=1
    	macset 2,4,*
    	macset 3,4,?B
    	bank 3
    	org pmrttl+(\1)
    	db low((\2)-1)
    	org pmrtth+(\1)
    	db high((\2)-1)
    	bank \3
    	org \2
    	endif
    	endm
    

## Dealing with carry flag

Several things can be done with the carry flag, such as: 

  * If you want to use a AND instruction and clear the carry flag at the same time, you can use the unofficial ANC instruction, if the high bit of the operand is cleared.
  * Often you can tell that the carry flag will have a specific state (such as after a conditional branch), which can help when using a ADC or SBC instruction.



## Clear accumulator and carry flag and reset bank
    
    
    CLC
    RLA ident
    

  * Bytes: 4
  * Cycles: 8



In this program, "`ident`" should point to a zero byte in ROM. The "official" way would be: 
    
    
    CLC
    LDA #0
    STA ident
    

  * Bytes: 6
  * Cycles: 8



## Self-modifying codes

## See also

  * [RTS Trick](RTS_Trick.xhtml "RTS Trick")
  * [Synthetic instructions](Synthetic_instructions.xhtml "Synthetic instructions")
  * [Programming with unofficial opcodes](Programming_with_unofficial_opcodes.xhtml "Programming with unofficial opcodes")


