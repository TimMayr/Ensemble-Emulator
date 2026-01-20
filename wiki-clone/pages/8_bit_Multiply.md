# 8-bit Multiply

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/8-bit_Multiply) | View [other pages](Special_AllPages.xhtml#8_bit_Multiply)

Since the 6502 CPU has no multiplication instruction, this feature has to be written in software. 

See also: [Fast signed multiply](Fast_signed_multiply.xhtml "Fast signed multiply")

## Contents

  * 1 tepples
  * 2 tepples unrolled
  * 3 Bregalad
  * 4 frantik
  * 5 MMC5
  * 6 External links



## tepples

This 8x8 multiply routine from _[Thwaite](User_Tepples_Thwaite.xhtml "User:Tepples/Thwaite")_ does binary long multiplication (a.k.a. "the [Russian Peasant method](https://en.wikipedia.org/wiki/Ancient_Egyptian_multiplication#Russian_peasant_multiplication "wikipedia:Ancient Egyptian multiplication")"). 
    
    
    ;;
    ; Multiplies two 8-bit factors to produce a 16-bit product
    ; in about 153 cycles.
    ; @param A one factor
    ; @param Y another factor
    ; @return high 8 bits in A; low 8 bits in $0000
    ;         Y and $0001 are trashed; X is untouched
    .proc mul8
    prodlo  = $0000
    factor2 = $0001
    
      ; Factor 1 is stored in the lower bits of prodlo; the low byte of
      ; the product is stored in the upper bits.
      lsr a  ; prime the carry bit for the loop
      sta prodlo
      sty factor2
      lda #0
      ldy #8
    loop:
      ; At the start of the loop, one bit of prodlo has already been
      ; shifted out into the carry.
      bcc noadd
      clc
      adc factor2
    noadd:
      ror a
      ror prodlo  ; pull another bit out for the next iteration
      dey         ; inc/dec don't modify carry; only shifts and adds do
      bne loop
      rts
    .endproc
    

This is a similar technique to Bregalad's method below, but is further optimized to keep part of the running total in the accumulator. 

Assuming no page crossings and zero page, this routine takes 137-169 cycles, not counting the JSR to call it. (Each non-zero bit in A adds 4 cycles.) 

## tepples unrolled

The above code can be made to run slightly faster by both unrolling the loop and pre-decrementing factor2 so that CLC isn't required. Note that the low byte is now returned in A, the high byte in Y, and that CA65 syntax is used. 
    
    
    ; @param A one factor
    ; @param Y another factor
    ; @return low 8 bits in A; high 8 bits in Y
    mul8_multiply:
        lsr
        sta prodlo
        tya
        beq mul8_early_return
        dey
        sty factor2
        lda #0
    .repeat 8, i
        .if i > 0
            ror prodlo
        .endif
        bcc :+
        adc factor2
    :
        ror
    .endrepeat
        tay
        lda prodlo
        ror
    mul8_early_return:
        rts
    

The code takes up 70 bytes and runs in 120 cycles or less. 

## Bregalad

This routine by Bregalad is similar to tepples' method above, but is less optimized. 
    
    
    ;8-bit multiply
    ;by Bregalad
    ;Enter with A,Y, numbers to multiply
    ;Output with YA = 16-bit result (X is unchanged)
    Multiply:
    	sty Factor  ;Store input factor
    	ldy #$00
    	sty Res
    	sty Res2    ;Clear result
    	ldy #$08    ;Number of shifts needed
    
    -	lsr A       ;Shift right input number
    	bcc +       ;Check if bit is set
    	pha
    	lda Res2
    	clc
    	adc Factor
    	sta Res2    ;If so add number to result
    	pla
    +	lsr Res2    ;Shift result right
    	ror Res
    	dey
    	bne -
    	lda Res
    	ldy Res2
    	rts
    

An optimization for efficiency is made here; binary long multiplication requires adding one multiplicand to the result at various bit-shifts (i.e. multiply by each power of 2). The naive approach might maintain the value to add as a 16-bit value, left shifting it once each iteration to reach the next power of 2. This one, however, takes advantage of the input being only 8-bits wide, and instead pre-multiplies the result by 256 (8 bits), and each iteration instead right-shifts the result. After 8 iterations the pre-multiply is undone, and the advantage gained is that only the shift is 16-bit; adding the multiplicand remains an efficient 8-bit add. 

Assuming no page crossings and zero page, this routine takes 184-320 cycles, not counting the JSR to call it. (Each non-zero bit in A adds 17 cycles.) 

## frantik

This routine by frantik is another binary long multiplication. 
    
    
    ; Multiply two bytes in memory using Russian peasant algorithm
    ; by frantik
    
    ; Accepts: value1 and value2, labels for bytes in memory
    ;   value2 should ideally be the lesser of the two input values 
    ;   for increased efficiency
    ; Uses: $00, $01, $02 for temporary variables
    ; Returns: 16 bit value in $00 and $01
    
    .macro multiply value1, value2
    
    ret  = $00              ; return value
    temp = $02              ; temp storage
    
    	lda #$00        ; clear temporary variables
    	sta ret
    	sta ret+1
    	sta temp
    	lda value2
    	bne +end"
    	jmp +start:
    
    -loop:
    	asl value1      ; double first value
    	rol temp        ; using 16bit precision
    	lsr value2      ; halve second vale
    +start:
    	lda value2      ;
    	and #01         ; is new 2nd value an odd number?
    	beq -loop:      ; 
    	clc             ; if so, add new 1st value to running total
    	lda ret         ;
    	adc value1      ;
    	sta ret         ;
    	lda ret+1       ;
    	adc temp        ;
    	sta ret+1       ;
    	lda value2      ;
    	cmp #01         ; is 2nd value 1?  if so, we're done
    	bne -loop:      ; otherwise, loop
    +end:
    .endm
    

Assuming no page crossings and zero page, this routine takes 17-403 cycles, though it is an in-place macro generating 46 bytes of new code each time it is used. 

## MMC5

The [MMC5](MMC5.xhtml "MMC5") contains an 8x8-bit multiplier at $5205-$5206. 

## External links

  * [Forum post:](https://forums.nesdev.org/viewtopic.php?f=2&t=16571) Fast multi, ... - Faster multiplication routine using lookup tables, by keldon.
  * [Forum post:](https://forums.nesdev.org/viewtopic.php?f=2&t=16616) What's the best (fastest) multiplication/division code?
  * [AtariAge forum post:](https://atariage.com/forums/topic/71120-6502-killer-hacks/page-2#entry896028) 6502 Killer hacks



Categories: [Arithmetic](Category_Arithmetic.xhtml)
