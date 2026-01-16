# Base 100

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Base_100) | View [other pages](Special_AllPages.xhtml#Base_100)

Unlike the regular 6502, the [2A03](2A03.xhtml "2A03") does not have decimal mode. One workaround for this is to keep numbers in binary, and use a [BCD conversion routine](16_bit_BCD.xhtml "16-bit BCD") to convert to binary-coded decimal as needed. Base 100 is another workaround that simplifies displaying the numbers onscreen. 

In base 100, a number consists of a series of bytes that range from 0-99 (or $00-$63 in hexadecimal). Unlike in regular 8-bit math, numbers wrap around at 100 instead of at 256, so $0063 + $0001 is $0100 instead of $0064. Given a base 100 number, you can use a 100-byte table to convert each byte to BCD, which is easy to display. 
    
    
    base_100_to_bcd:
      .byte $00, $01, $02, $03, $04, $05, $06, $07, $08, $09, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19
      .byte $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36, $37, $38, $39
      .byte $40, $41, $42, $43, $44, $45, $46, $47, $48, $49, $50, $51, $52, $53, $54, $55, $56, $57, $58, $59
      .byte $60, $61, $62, $63, $64, $65, $66, $67, $68, $69, $70, $71, $72, $73, $74, $75, $76, $77, $78, $79
      .byte $80, $81, $82, $83, $84, $85, $86, $87, $88, $89, $90, $91, $92, $93, $94, $95, $96, $97, $98, $99
    

Base 100 is good for numbers that you want to display and do addition and subtraction on, but don't need for more complicated math. It can be good for things like a score, an amount of currency, or a timer or countdown. 

## Contents

  * 1 Code examples
    * 1.1 BCD conversion
    * 1.2 Byte to base 100 encoding
    * 1.3 16-bit increment
    * 1.4 16-bit decrement
    * 1.5 16-bit addition
    * 1.6 16-bit subtraction
    * 1.7 Multiplication/Division by powers of 2
      * 1.7.1 Multiplication by 2
      * 1.7.2 Division by 2
    * 1.8 Multiplication/Division by powers of 100
      * 1.8.1 Multiplication by 100
      * 1.8.2 Division by 100
  * 2 Games using base 100



## Code examples

### BCD conversion

The BCD table above makes it trivial to calculate the decimal digits (div/mod by 10) to be displayed: 
    
    
      ldx base_100_number
      lda base_100_to_bcd,x
      tay ; alternatively, use a zeropage variable
      lsr a
      lsr a
      lsr a
      lsr a
      sta PPUDATA ; tens digit
      tya
      and #%00001111
      sta PPUDATA ; ones digit
    

For even more speed, you can have two tables that separately provide the ones digit and the tens digit of the resulting BCD number: 
    
    
      ldx base_100_number
      lda base_100_tens,x
      sta PPUDATA ; tens digit
      lda base_100_ones,x
      sta PPUDATA ; ones digit
    

Note: These examples write directly to [PPUDATA](PPU_registers.xhtml "PPUDATA") for simplicity. A more robust approach would be to [ buffer](The_frame_and_NMIs.xhtml#Buffer_Formats "The frame and NMIs") the data to be transferred instead. 

### Byte to base 100 encoding

This takes in an unsigned 8-bit value in A and returns its base 100 encoding in X (high byte) and A (low byte): 
    
    
    ; Parameter: A = byte to encode
    ; Returns base 100 encoding as: X = high byte, A = low byte
    ByteToBase100:
      ldx #0 ; Init high byte of result
      :
      cmp #100 ; 0~99 is valid
      bcc @ret
        inx
        sbc #100 ; Carry already set
        bcs :- ; Carry is guaranteed to be set
    @ret:
      rts
    

### 16-bit increment

This increments a 16-bit base 100 number by 1, while preventing it from going over 9999: 
    
    
    AddOneCoin:
      lda #99 ; Check for cap of 9999
      cmp Money+1
      bne :+
      cmp Money+0
      bne :+
        rts ; Exit if cap already reached
      :
      ; Otherwise increment the amount
      inc Money
      lda Money
      cmp #100
      bcc :+
        lda #0 ; Base 100 overflow
        sta Money
        inc Money+1
      :
      rts
    

### 16-bit decrement

This decrements a 16-bit base 100 number by 1, while preventing it from going under 0: 
    
    
    SubOneCoin:
      lda Money ; Check if amount is 0
      ora Money+1
      beq :+
      ; Otherwise decrement the amount
      lda #99
      dec Money
      bpl :+
        sta Money ; Base 100 underflow
        dec Money+1
      :
      rts 
    

### 16-bit addition

Here's an example that demonstrates base 100 addition, using a pair of 16-bit numbers. The result is clamped to 9999: 
    
    
    AddPrizeMoney:
      lda Money
      clc
      adc Prize
      cmp #100
      bcc :+
        ; Carry is set if the code ends up in here
        sbc #100 ; Guaranteed to set carry
      :
      sta Money
      ; Carry will correctly reflect base 100 overflow here
      lda Money+1
      adc Prize+1
      cmp #100
      bcc :+
        lda #99 ; Cap amount at 9999
        sta Money
      :
      ; Write the new amount
      sta Money+1
      rts
    

### 16-bit subtraction

Here's an example that demonstrates base 100 subtraction, using a pair of 16-bit numbers: 
    
    
    PurchaseItem:
      lda Money
      sec
      sbc Price
      bpl :+
        ; Carry is clear if the code ends up in here
        adc #100
        clc
      :
      sta Temp
      ; Carry will correctly reflect base 100 underflow here
      lda Money+1
      sbc Price+1
      bmi NotEnoughFunds
      ; Write the new amount
      sta Money+1
      lda Temp
      sta Money
      rts
    

### Multiplication/Division by powers of 2

Multiplication and division by powers of 2 are straightforward to do in base 100 through the use of shift and rotate operations. 

#### Multiplication by 2

Multiply a 16-bit base 100 value by 2, clamping the result to 9999: 
    
    
    Mult2:
      lda Val ; Double low byte
      asl a
      cmp #100
      bcc :+
        ; Carry is set if the code ends up in here
        sbc #100 ; Guaranteed to set carry
    :
      ; Carry will correctly reflect base 100 overflow here
      sta Val
      lda Val+1 ; Double high byte while also adding carry
      rol a
      cmp #100
      bcc :+
        lda #99 ; Cap amount at 9999
        sta Val
    :
      sta Val+1
      rts
    

#### Division by 2

Divide a 16-bit base 100 value by 2: 
    
    
    Div2:
      lsr Val+1 ; Halve high byte
      lda Val
      bcc :+
        ; Carry is set if the code ends up in here
        adc #99 ; Add 100 to low byte so it can be halved to 50
    :
      lsr a ; Halve low byte
      sta Val
      rts
    

### Multiplication/Division by powers of 100

Multiplication and division by powers of 100 are trivial to do in base 100, analogous to powers of 10 in base 10 (BCD) or powers of 2 in binary - shift the number over by one byte and place a zero at the opposite end for each power of 100. 

#### Multiplication by 100

Multiply an 8-bit base 100 value by 100, storing the result in 16 bits: 
    
    
    Mult100:
      lda #0 ; Put a 0 into the low byte of the result
      sta Res
      lda Value ; Put the input into the high byte of the result to multiply by 100
      sta Res+1
      rts
    

#### Division by 100

Divide a 16-bit base 100 value by 100, storing the result in 16 bits: 
    
    
    Div100:
      lda #0 ; Put a 0 into the high byte of the result
      sta Res+1
      lda Value+1 ; Put the input high byte into the low byte of the result to divide by 100
      sta Res
      rts
    

## Games using base 100

Homebrew games: 

  * _[Thwaite](https://github.com/pinobatch/thwaite-nes/)_ , in [`addScore`](https://github.com/pinobatch/thwaite-nes/blob/00e36745188bc165990f60eed6b093c3ce6ad0e3/src/bg.s#L667-L686)



Hacks: 

  * _[TetrisNESBugfix](https://github.com/TakuikaNinja/TetrisNESBugfix)_ , in the routines used by [`calcAndAddLineClearPoints`](https://github.com/TakuikaNinja/TetrisNESBugfix/blob/02504909b12e04faa5d93633a79c6269f4588e2a/main.asm#L3431) (optimised multiplication logic for line clear points). Base 100 is also used for variables such as the line clear count and level number.



Categories: [Arithmetic](Category_Arithmetic.xhtml)
