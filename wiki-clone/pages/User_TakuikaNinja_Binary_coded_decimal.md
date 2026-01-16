# User:TakuikaNinja/Binary-coded decimal

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3ATakuikaNinja/Binary-coded_decimal) | View [other pages](Special_AllPages.xhtml#User_TakuikaNinja_Binary_coded_decimal)

**[Binary-coded decimal](https://en.wikipedia.org/wiki/Binary-coded_decimal "wikipedia:Binary-coded decimal") (BCD)** is a format used to store decimal numbers in binary systems, usually so it can be easily displayed on a graphical interface. On most processors implementing BCD arithmetic, BCD values are added/subtracted in binary and the result is corrected to BCD using a special instruction (e.g. DAA/DAS on x86). On the 6502 and its successors, however, a special flag called the [decimal flag](Status_flags.xhtml#D:_Decimal "Status flags") is used to toggle automatic BCD arithmetic on the ADC/SBC instructions (and nothing else).[1] This flag allows one to accomplish BCD arithmetic trivially using code such as the following: 
    
    
      sed ; set decimal flag
      lda #$23
      clc
      adc #$50 ; = $73
      sec
      sbc #$04 ; = $69
      sta Sum ; ONLY the carry flag is valid after BCD arithmetic on the original 6502, 
      lda Sum ; so reload the value to get the zero/negative flags! (fixed on 65C02/65816)
      cld ; don't forget to clear the decimal flag once we're done!
    

Unfortunately, the NES [CPU](CPU.xhtml "CPU") does not have that luxury as the BCD logic is disconnected within the die. The remainder of this article will discuss workarounds for BCD arithmetic without a working decimal flag. These come at the cost of increased code size, cycle counts, and memory usage. Software BCD arithmetic is often implemented using a set of generic routines and/or macros to assist with abstraction. 

Programmers may prefer to keep numbers in binary format and manually convert them to BCD format for display purposes instead: 

  * [16-bit BCD](16_bit_BCD.xhtml "16-bit BCD") \- An efficient 16-bit binary to decimal converter
  * [Base 100](Base_100.xhtml "Base 100") \- Alternate method of storing numbers, with simpler logic for arithmetic and BCD conversion



## Contents

  * 1 Unpacked BCD
  * 2 Packed BCD
    * 2.1 Base 100 conversion
    * 2.2 Unpacking
  * 3 Multiply/Divide by powers of 10
    * 3.1 Unpacked
    * 3.2 Packed
  * 4 References



## Unpacked BCD

Unpacked BCD stores each BCD digit ($00~$09) as individual bytes. This has the advantage of simplifying the logic for doing arithmetic on these numbers and displaying them. However, this wastes some amount of memory as the upper nybble will be unused for each digit. _Super Mario Bros._ , _Dr. Mario_ , and many other games use this format. The logic for unpacked BCD arithmetic is identical to that of [base 100](Base_100.xhtml "Base 100"), simply requiring a change to base 10. 

## Packed BCD

Packed BCD stores 2 BCD digits as nybbles within a single byte ($00~$99, no A~F nybbles), and is the format normally used in BCD arithmetic. This reduces memory usage but requires careful logic when it comes to doing arithmetic on these numbers without the decimal flag. _Tetris_ (Nintendo) uses this format but notably suffers from inaccuracies and performance issues due to a flawed implementation of packed BCD arithmetic.[2] There are two methods of implementing BCD arithmetic in this scheme. 

### Base 100 conversion

One option is to convert the BCD bytes to [base 100](Base_100.xhtml "Base 100"), do arithmetic on them, and convert them back to BCD format. However, it would be better to use base 100 all of the time instead. 

This routine takes in a packed BCD byte and converts it to binary form: 
    
    
    ; Parameter: A = BCD value to decode
    ; Stores the hexadecimal value in Res
    ; Returns the hexadecimal value in A
    ; (result is also a valid base 100 value)
    BCDToByte:
      pha ; save value for later
      and #$F0 ; letting x = 10's digit, convert 16x to 10x
      lsr a
      sta Res ; 16x/2
      lsr a
      lsr a ; 16x/8
      adc Res ; 16x/8 + 16x/2
      sta Res ; = 10x
      pla ; retrieve and add low nybble to result
      and #$0F
      adc Res ; carry is always clear
      sta Res
      rts
    

The following lookup table (using the packed BCD value as an index) can be used instead if speed is desired: 
    
    
    bcd_to_bin:
      .byte $00, $01, $02, $03, $04, $05, $06, $07, $08, $09, 0, 0, 0, 0, 0, 0
      .byte $0a, $0b, $0c, $0d, $0e, $0f, $10, $11, $12, $13, 0, 0, 0, 0, 0, 0 
      .byte $14, $15, $16, $17, $18, $19, $1a, $1b, $1c, $1d, 0, 0, 0, 0, 0, 0 
      .byte $1e, $1f, $20, $21, $22, $23, $24, $25, $26, $27, 0, 0, 0, 0, 0, 0 
      .byte $28, $29, $2a, $2b, $2c, $2d, $2e, $2f, $30, $31, 0, 0, 0, 0, 0, 0 
      .byte $32, $33, $34, $35, $36, $37, $38, $39, $3a, $3b, 0, 0, 0, 0, 0, 0 
      .byte $3c, $3d, $3e, $3f, $40, $41, $42, $43, $44, $45, 0, 0, 0, 0, 0, 0 
      .byte $46, $47, $48, $49, $4a, $4b, $4c, $4d, $4e, $4f, 0, 0, 0, 0, 0, 0 
      .byte $50, $51, $52, $53, $54, $55, $56, $57, $58, $59, 0, 0, 0, 0, 0, 0
      .byte $5a, $5b, $5c, $5d, $5e, $5f, $60, $61, $62, $63
    

### Unpacking

Another option is to unpack the digits of each BCD byte into nybbles, do arithmetic on them, and pack the result back into bytes. This way, similar logic as unpacked BCD can be used. 

This snippet demonstrates the unpacking process: 
    
    
      lda Val
      and #%11110000
      sta HiDigit ; = %xxxx0000
      eor Val
      sta LoDigit ; = %0000xxxx
    

Repacking the value is a simple bitwise OR operation: 
    
    
      lda HiDigit ; must be %xxxx0000
      ora LoDigit ; must be %0000xxxx
      sta Res
    

## Multiply/Divide by powers of 10

One advantage of BCD is that multiplication and division by powers of 10 are trivial to implement, and does not require a working decimal flag. 

### Unpacked

To multiply/divide packed BCD bytes by powers of 10, simply shift them left/right by one byte and place a zero at the opposite end for each power of 10. 

### Packed

To multiply/divide packed BCD bytes by 10, simply shift them left/right by four bits (one nybble) using a sequence of shift and rotate instructions: 
    
    
    ; multiply 16-bit packed BCD value by 10
      asl Val ; shift d7 into carry
      rol Val+1 ; rotate carry into d0
      asl Val ; repeat
      rol Val+1
      asl Val
      rol Val+1
      asl Val
      rol Val+1
    
    ; divide 16-bit packed BCD value by 10
      lsr Val+1 ; shift d0 into carry
      ror Val ; rotate carry into d7
      lsr Val+1 ; repeat
      ror Val
      lsr Val+1
      ror Val
      lsr Val+1
      ror Val
    

To multiply/divide packed BCD bytes by 100, simply shift them left/right by one byte and place a zero at the opposite end for each power of 100, similar to unpacked BCD. Arbitrary powers of 10 can be accounted for by applying byte shifts for each power of 100, then applying a nybble shift if a power of 10 remains. 

## References

  1. ↑ [6502 decimal mode tutorial](http://www.6502.org/tutorials/decimal_mode.html)
  2. ↑ [Applying Artificial Intelligence to Nintendo Tetris](https://meatfighter.com/nintendotetrisai/#The_Kill_Screen): Breakdown of the flawed BCD addition routine in _Tetris_ (Nintendo).


