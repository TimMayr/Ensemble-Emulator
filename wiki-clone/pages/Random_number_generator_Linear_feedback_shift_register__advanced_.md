# Random number generator/Linear feedback shift register (advanced)

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Random_number_generator/Linear_feedback_shift_register_%28advanced%29) | View [other pages](Special_AllPages.xhtml#Random_number_generator_Linear_feedback_shift_register__advanced_)

Further commentary on the [linear feedback shift register](Random_number_generator.xhtml#Linear_feedback_shift_register "Random number generator") example at [random number generator](Random_number_generator.xhtml "Random number generator"). 

These are excerpted from the following source on github: [prng_6502](https://github.com/bbbradsmith/prng_6502)

## Contents

  * 1 Basic version
  * 2 Sacrifice entropy for speed
  * 3 Simple 24 and 32 bit LFSR
  * 4 Overlapped 24 and 32 bit LFSR
  * 5 References
  * 6 Additional Resource



## Basic version
    
    
    prng:
    	ldy #8     ; iteration count
    	lda seed+0
    :
    	asl        ; shift the register
    	rol seed+1
    	bcc :+
    	eor #$39   ; apply XOR feedback whenever a 1 bit is shifted out
    :
    	dey
    	bne :--
    	sta seed+0
    	cmp #0     ; reload flags
    	rts
    

## Sacrifice entropy for speed

The iteration count stored in Y can be reduced to speed up the generator, at the expense of quality of randomness. Each iteration effectively generators one more bit of entropy, so 8 iterations are needed for an 8-bit random number. If you intend to use fewer bits of the result (e.g. use AND to mask the result), or if you are satisfied with less randomness, you can reduce Y, or even parameterize it: 
    
    
    ; Y as a parameter specifies number of random bits to generate (1 to 8)
    prng:
    	lda seed+0
    @bitloop:
    	asl
    	rol seed+1
    	bcc :+
    	eor #$39
    :
    	dey
    	bne @bitloop
    	sta seed+0
    	cmp #0
    	rts
    

Alternatively this loop could be unrolled with 8 entry points, saving the need to use Y or load it as a parameter. 

## Simple 24 and 32 bit LFSR

By adding an extra byte or two to the seed variable, and choosing an appropriate polynomial to XOR with, we can extend the sequence length significantly with only one additonal ROL per byte (+40 cycles). 

This 24-bit version has a sequence length of 16777215: 21 bytes, 173-181 cycles. 
    
    
    .zeropage
    seed: .res 3 ; 24-bit
    
    .code
    prng:
    	ldy #8
    	lda seed+0
    :
    	asl
    	rol seed+1
    	rol seed+2
    	bcc :+
    	eor #$1B
    :
    	dey
    	bne :--
    	sta seed+0
    	cmp #0
    	rts
    

This 32-bit version has a sequence length of 4294967295: 23 bytes, 213-221 cycles. 
    
    
    .zeropage
    seed: .res 4 ; 32-bit
    
    .code
    prng:
    	ldy #8
    	lda seed+0
    :
    	asl
    	rol seed+1
    	rol seed+2
    	rol seed+3
    	bcc :+
    	eor #$C5
    :
    	dey
    	bne :--
    	sta seed+0
    	cmp #0
    	rts
    

Even longer sequences are possible, but it's not likely to be practical, as it would already take approximately 7 days for an NTSC NES to complete this 32 bit LFSR cycle if doing nothing else. 

## Overlapped 24 and 32 bit LFSR

With an XOR-feedback that contains only four bits, we can shift and feed back 8 bits at once in a more complex overlapped operation that essentially applies 4 16-bit XOR operations to the lower two bytes of the seed. (One XOR for each feedback bit.) With some careful rearrangement this can do 8 iterations at once very efficiently. 

24-bit overlapped: 38 bytes, 73 cycles. 
    
    
    prng:
    	; rotate the middle byte left
    	ldy seed+1 ; will move to seed+2 at the end
    	; compute seed+1 ($1B>>1 = %1101)
    	lda seed+2
    	lsr
    	lsr
    	lsr
    	lsr
    	sta seed+1 ; reverse: %1011
    	lsr
    	lsr
    	eor seed+1
    	lsr
    	eor seed+1
    	eor seed+0
    	sta seed+1
    	; compute seed+0 ($1B = %00011011)
    	lda seed+2
    	asl
    	eor seed+2
    	asl
    	asl
    	eor seed+2
    	asl
    	eor seed+2
    	sty seed+2 ; finish rotating byte 1 into 2
    	sta seed+0
    	rts

32-bit overlapped: 44 bytes, 83 cycles. 
    
    
    prng:
    	; rotate the middle bytes left
    	ldy seed+2 ; will move to seed+3 at the end
    	lda seed+1
    	sta seed+2
    	; compute seed+1 ($C5>>1 = %1100010)
    	lda seed+3 ; original high byte
    	lsr
    	sta seed+1 ; reverse: 100011
    	lsr
    	lsr
    	lsr
    	lsr
    	eor seed+1
    	lsr
    	eor seed+1
    	eor seed+0 ; combine with original low byte
    	sta seed+1
    	; compute seed+0 ($C5 = %11000101)
    	lda seed+3 ; original high byte
    	asl
    	eor seed+3
    	asl
    	asl
    	asl
    	asl
    	eor seed+3
    	asl
    	asl
    	eor seed+3
    	sty seed+3 ; finish rotating byte 2 into 3
    	sta seed+0
    	rts
    

A note about the chosen polynomials: several XOR-feedback values are available that will produce a maximal-length LFSR period[1]. $39, $2D, and $C5 are chosen because they contain the minimum number of bits, in a compact arrangement that allows a fast overlapped computation. 

## References

  1. ↑ [List of possible 1-byte XOR values for maximal-length Galois LFSR](https://github.com/bbbradsmith/prng_6502/blob/master/utils/polyfind.txt)



  * [32-bit LFSR PRNG](http://forums.nesdev.org/viewtopic.php?p=196569#p196569) by jroatch



## Additional Resource

  * <http://users.ece.cmu.edu/~koopman/lfsr/index.html> \- a source for maximal polynomials for LFSRs of many lengths.


