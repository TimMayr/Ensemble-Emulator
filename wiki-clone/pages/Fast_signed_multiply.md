# Fast signed multiply

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Fast_signed_multiply) | View [other pages](Special_AllPages.xhtml#Fast_signed_multiply)

The following code multiplies two 8-bit signed integers, of range (-64..+64). The output is a signed 16-bit result, in range (-4096..4096). It uses a 256 byte lookup table, but is quite fast. 
    
    
    ;Fast table driven signed multiply routine
    ;It multiplies Y with A, and relies on the mathematical fact (a+b)^2-(a-b)^2 = 4*a*b
    ;Also both inputs and outputs are SIGNED and are never supposed to be outside of the range (-64; +64)
    ;Return with YA = 4*A*Y (result can be shifted as necessary)
    
    Multiply
    	sty Temp
    	pha
    	clc
    	adc Temp	;Add two multiplicands together
    	bpl +
    	eor #$ff
    	clc
    	adc #$01	;If result is negative, force it to be positive
    +	asl A
    	tax		;The square of a negative number is equal to its pos counterpart anyways
    
    	pla
    	sec
    	sbc Temp       ;Compute difference of two multiplicands
    	bpl +
    	eor #$ff       ;Again, force the result to be positive
    	clc
    	adc #$01
    +	asl A
    	tay
    
    	lda SquareTbl,X
    	sec
    	sbc SquareTbl,Y    ;Compute (a+y)^2-(a-y)^2
    	pha
    	lda SquareTbl+1,X
    	sbc SquareTbl+1,Y
    	tay
    	pla
    	rts
    
    ;This is a square table for the multiply routine
    ;Note that this macro is for WLA-DX, but can be adapted
    ;For most assemblers
    
    SquareTbl
    .def i = 0
    .rept 128
    	.dw i^2			;Create a square table with 128 word entries
    	.redef i i+1
    .endr
    .ends
    

Categories: [Arithmetic](Category_Arithmetic.xhtml)
