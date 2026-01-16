# User:TakuikaNinja/Code snippets

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3ATakuikaNinja/Code_snippets) | View [other pages](Special_AllPages.xhtml#User_TakuikaNinja_Code_snippets)

This page contains various code snippets intended for use in future articles. Snippets without explicit mentions of being used in commercial games are considered to have no licenses, and may be used freely. 

## BCD byte to binary

This takes in a packed BCD byte and converts it to binary form. The result is also a valid [base 100](Base_100.xhtml "Base 100") value. 
    
    
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
    

## 8-bit PRNG

This is an [LCG](https://en.wikipedia.org/wiki/Linear_congruential_generator "wikipedia:Linear congruential generator") of the form x = (5x + 1) % 256, ported from [Z80 code](https://www.cpcwiki.eu/index.php/Programming:Random_Number_Generator#8-bit_random_number_generator). It is fairly small and simple but has terrible entropy. It may be useful if you need random outcomes for infrequent/unimportant events. 
    
    
    ; Crude 8-bit PRNG, exits with output in A
    ; Seed is both input/output, should be in zeropage for best performance
    rand8:
      lda Seed
      asl a
      asl a
      sec
      adc Seed
      sta Seed
      rts
    

If you want something with higher quality output, consider using [this 8-bit PRNG routine](https://foobles.neocities.org/blog/2024-06-06/prng) instead. 

## MMC3 scanline counter init

Commercial [MMC3](MMC3.xhtml "MMC3") games contain code to manually clock the scanline counter in the reset handler (after waiting for the PPU to warm up) by toggling PPU A12. This likely prevents undesirable behaviour caused by reloading the counter on consecutive scanlines. This snippet is present verbatim in most commercial games: 
    
    
      bit $2002
      lda #$10
      tax
    @loop:
      sta $2006
      sta $2006
      eor #$10
      dex
      bne @loop
    

The following snippet does the minimum amount of toggles required, and uses a ring counter to reduce the code size and cycle count. A12 is set to 0, 1, 0, 1, 0. 
    
    
      bit $2002
      lda #%00001010
    @loop:
      sta $2006
      sta $2006
      asl a
      bcc @loop
    

This unrolled snippet does the minimum amount of toggles required (similar code appears in _Super Mario Bros. 3_): 
    
    
      bit $2002
      lda #$00
      sta $2006
      sta $2006
      lda #$10
      sta $2006
      sta $2006
      lda #$00
      sta $2006
      sta $2006
      lda #$10
      sta $2006
      sta $2006
    
