# HexToDecimal.8

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/HexToDecimal.8) | View [other pages](Special_AllPages.xhtml#HexToDecimal_8)

Celius wrote 8-, 16-, and 24-bit binary to decimal converters. They work by storing the BCD version of 1, 2, 4, 8, 16, ..., 32768, adding them all up digit-wise, and performing the carry through big lookup tables. 
    
    
    HexToDecimal.8:
    ;Given: Hex value in Hex0
    ;Returns decimal value in DecOnes, DecTens, and DecHundreds.
    
    	lda #$00
    	sta DecOnes
    	sta DecTens
    	sta DecHundreds
    
    	lda Hex0
    	and #$0F
    	tax
    	lda HexDigit00Table,x
    	sta DecOnes
    	lda HexDigit01Table,x
    	sta DecTens
    
    	lda Hex0
    	lsr a
    	lsr a
    	lsr a
    	lsr a
    	tax
    	lda HexDigit10Table,x
    	clc
    	adc DecOnes
    	sta DecOnes
    	lda HexDigit11Table,x
    	adc DecTens
    	sta DecTens
    	lda HexDigit12Table,x
    	sta DecHundreds
    
    	clc
    	ldx DecOnes
    	lda DecimalSumsLow,x
    	sta DecOnes
    	
    
    	lda DecimalSumsHigh,x
    	adc DecTens
    	tax
    	lda DecimalSumsLow,x
    	sta DecTens
    
    	lda DecimalSumsHigh,x
    	adc DecHundreds
    	tax
    	lda DecimalSumsLow,x
    	sta DecHundreds			;118
    
    	rts
    
    HexToDecimal.16:
    ;Given: Hex value in Hex0 and Hex1.
    ;Returns decimal value in DecOnes, DecTens, DecHundreds, DecThousands, DecTenThousands.
    
    	lda #$00
    	sta DecOnes
    	sta DecTens
    	sta DecHundreds
    	sta DecThousands
    	sta DecTenThousands
    
    	lda Hex0
    	and #$0F
    	tax
    	lda HexDigit00Table,x
    	sta DecOnes
    	lda HexDigit01Table,x
    	sta DecTens
    
    	lda Hex0
    	lsr a
    	lsr a
    	lsr a
    	lsr a
    	tax
    	lda HexDigit10Table,x
    	clc
    	adc DecOnes
    	sta DecOnes
    	lda HexDigit11Table,x
    	adc DecTens
    	sta DecTens
    	lda HexDigit12Table,x
    	sta DecHundreds
    
    	lda Hex1
    	and #$0F
    	tax
    	lda HexDigit20Table,x
    	clc
    	adc DecOnes
    	sta DecOnes
    	lda HexDigit21Table,x
    	adc DecTens
    	sta DecTens
    	lda HexDigit22Table,x
    	adc DecHundreds
    	sta DecHundreds
    	lda HexDigit23Table,x
    	sta DecThousands
    
    	lda Hex1
    	lsr a
    	lsr a
    	lsr a
    	lsr a
    	tax
    	clc
    	lda HexDigit30Table,x
    	adc DecOnes
    	sta DecOnes
    	lda HexDigit31Table,x
    	adc DecTens
    	sta DecTens
    	lda HexDigit32Table,x
    	adc DecHundreds
    	sta DecHundreds
    	lda HexDigit33Table,x
    	adc DecThousands
    	sta DecThousands
    	lda HexDigit34Table,x
    	sta DecTenThousands
    
    	clc
    	ldx DecOnes
    	lda DecimalSumsLow,x
    	sta DecOnes
    	
    
    	lda DecimalSumsHigh,x
    	adc DecTens
    	tax
    	lda DecimalSumsLow,x
    	sta DecTens
    
    	lda DecimalSumsHigh,x
    	adc DecHundreds
    	tax
    	lda DecimalSumsLow,x
    	sta DecHundreds
    
    	lda DecimalSumsHigh,x
    	adc DecThousands
    	tax
    	lda DecimalSumsLow,x
    	sta DecThousands
    
    	lda DecimalSumsHigh,x
    	adc DecTenThousands
    	tax
    	lda DecimalSumsLow,x
    	sta DecTenThousands			;263
    
    	rts
    
    
    HexToDecimal.24:
    ;Given hex value in Hex0, Hex1, and Hex2
    ;Returns decimal value in DecOnes, DecTens, DecHundreds, DecThousands, DecTenThousands, DecHundredThousands, DecMillions,
    ;and DecTenMillions.
    
    	lda #$00
    	sta DecOnes
    	sta DecTens
    	sta DecHundreds
    	sta DecThousands
    	sta DecTenThousands
    	sta DecHundredThousands
    	sta DecMillions
    	sta DecTenMillions
    	
    
    	lda Hex0
    	and #$0F
    	tax
    	lda HexDigit00Table,x
    	sta DecOnes
    	lda HexDigit01Table,x
    	sta DecTens
    
    	lda Hex0
    	lsr a
    	lsr a
    	lsr a
    	lsr a
    	tax
    	lda HexDigit10Table,x
    	clc
    	adc DecOnes
    	sta DecOnes
    	lda HexDigit11Table,x
    	adc DecTens
    	sta DecTens
    	lda HexDigit12Table,x
    	sta DecHundreds
    
    
    	lda Hex1
    	and #$0F
    	tax
    	lda HexDigit20Table,x
    	clc
    	adc DecOnes
    	sta DecOnes
    	lda HexDigit21Table,x
    	adc DecTens
    	sta DecTens
    	lda HexDigit22Table,x
    	adc DecHundreds
    	sta DecHundreds
    	lda HexDigit23Table,x
    	sta DecThousands
    
    
    	lda Hex1
    	lsr a
    	lsr a
    	lsr a
    	lsr a
    	tax
    	clc
    	lda HexDigit30Table,x
    	adc DecOnes
    	sta DecOnes
    	lda HexDigit31Table,x
    	adc DecTens
    	sta DecTens
    	lda HexDigit32Table,x
    	adc DecHundreds
    	sta DecHundreds
    	lda HexDigit33Table,x
    	adc DecThousands
    	sta DecThousands
    	lda HexDigit34Table,x
    	sta DecTenThousands
    
    	lda Hex2
    	and #$0F
    	tax
    	clc
    	lda HexDigit40Table,x
    	adc DecOnes
    	sta DecOnes
    	lda HexDigit41Table,x
    	adc DecTens
    	sta DecTens
    	lda HexDigit42Table,x
    	adc DecHundreds
    	sta DecHundreds
    	lda HexDigit43Table,x
    	adc DecThousands
    	sta DecThousands
    	lda HexDigit44Table,x
    	adc DecTenThousands
    	sta DecTenThousands
    	lda HexDigit45Table,x
    	sta DecHundredThousands
    
    	lda Hex2
    	lsr a
    	lsr a
    	lsr a
    	lsr a
    	tax
    	clc
    	lda HexDigit50Table,x
    	adc DecOnes
    	sta DecOnes
    	lda HexDigit51Table,x
    	adc DecTens
    	sta DecTens
    	lda HexDigit52Table,x
    	adc DecHundreds
    	sta DecHundreds
    	lda HexDigit53Table,x
    	adc DecThousands
    	sta DecThousands
    	lda HexDigit54Table,x
    	adc DecTenThousands
    	sta DecTenThousands
    	lda HexDigit55Table,x
    	adc DecHundredThousands
    	sta DecHundredThousands
    	lda HexDigit56Table,x
    	sta DecMillions
    	lda HexDigit57Table,x
    	sta DecTenMillions
    
    	clc
    	ldx DecOnes
    	lda DecimalSumsLow,x
    	sta DecOnes
    	
    
    	lda DecimalSumsHigh,x
    	adc DecTens
    	tax
    	lda DecimalSumsLow,x
    	sta DecTens
    
    	lda DecimalSumsHigh,x
    	adc DecHundreds
    	tax
    	lda DecimalSumsLow,x
    	sta DecHundreds
    
    	lda DecimalSumsHigh,x
    	adc DecThousands
    	tax
    	lda DecimalSumsLow,x
    	sta DecThousands
    
    	lda DecimalSumsHigh,x
    	adc DecTenThousands
    	tax
    	lda DecimalSumsLow,x
    	sta DecTenThousands
    
    	lda DecimalSumsHigh,x
    	adc DecHundredThousands
    	tax
    	lda DecimalSumsLow,x
    	sta DecHundredThousands
    
    	lda DecimalSumsHigh,x
    	adc DecMillions
    	tax
    	lda DecimalSumsLow,x
    	sta DecMillions
    
    	lda DecimalSumsHigh,x
    	adc DecTenMillions
    	tax
    	lda DecimalSumsLow,x
    	sta DecTenMillions
    
    								;475
    
    	rts
    
    ;************ Pre-Converted Hex to Decimal Tables *************
    
    ;******
    
    ;1 byte
    HexDigit32Table:
    	.db $0
    
    HexDigit00Table:
    HexDigit56Table:
    DecimalSumsLow:
    ;55 bytes
    	.db $0,$1,$2,$3,$4,$5,$6,$7,$8,$9,$0,$1,$2,$3,$4,$5
    	.db $6,$7,$8,$9,$0,$1,$2,$3,$4,$5,$6,$7,$8,$9,$0,$1
    	.db $2,$3,$4,$5,$6,$7,$8,$9,$0,$1,$2,$3,$4,$5,$6,$7
    	.db $8,$9,$0,$1,$2,$3,$4
    
    HexDigit01Table:
    HexDigit57Table:
    DecimalSumsHigh:
    ;55 bytes
    	.db $0,$0,$0,$0,$0,$0,$0,$0,$0,$0,$1,$1,$1,$1,$1,$1
    	.db $1,$1,$1,$1,$2,$2,$2,$2,$2,$2,$2,$2,$2,$2,$3,$3
    	.db $3,$3,$3,$3,$3,$3,$3,$3,$4,$4,$4,$4,$4,$4,$4,$4
    	.db $4,$4,$5,$5,$5,$5,$5
    
    ;111 bytes
    ;******
    HexDigit50Table:
    HexDigit40Table:
    HexDigit30Table:
    HexDigit20Table:
    HexDigit10Table:
    	.db $0,$6,$2,$8,$4,$0,$6,$2,$8,$4,$0,$6,$2,$8,$4,$0
    
    HexDigit11Table:
    	.db $0,$1,$3,$4,$6,$8,$9,$1,$2,$4,$6,$7,$9,$0,$2,$4
    
    HexDigit12Table:
    	.db $0,$0,$0,$0,$0,$0,$0,$1,$1,$1,$1,$1,$1,$2,$2,$2
    ;******
    HexDigit21Table:
    	.db $0,$5,$1,$6,$2,$8,$3,$9,$4,$0,$6,$1,$7,$2,$8,$4
    
    HexDigit22Table:
    	.db $0,$2,$5,$7,$0,$2,$5,$7,$0,$3,$5,$8,$0,$3,$5,$8
    
    HexDigit23Table:
    	.db $0,$0,$0,$0,$1,$1,$1,$1,$2,$2,$2,$2,$3,$3,$3,$3
    ;******
    HexDigit31Table:
    	.db $0,$9,$9,$8,$8,$8,$7,$7,$6,$6,$6,$5,$5,$4,$4,$4
    
    HexDigit33Table:
    	.db $0,$4,$8,$2,$6,$0,$4,$8,$2,$6,$0,$5,$9,$3,$7,$1
    
    HexDigit34Table:
    	.db $0,$0,$0,$1,$1,$2,$2,$2,$3,$3,$4,$4,$4,$5,$5,$6
    
    ;******
    HexDigit41Table:
    	.db $0,$3,$7,$0,$4,$8,$1,$5,$8,$2,$6,$9,$3,$6,$0,$4
    
    HexDigit42Table:
    	.db $0,$5,$0,$6,$1,$6,$2,$7,$2,$8,$3,$8,$4,$9,$5,$0
    
    HexDigit43Table:
    	.db $0,$5,$1,$6,$2,$7,$3,$8,$4,$9,$5,$0,$6,$1,$7,$3
    
    HexDigit44Table:
    	.db $0,$6,$3,$9,$6,$2,$9,$5,$2,$8,$5,$2,$8,$5,$1,$8
    
    HexDigit45Table:
    	.db $0,$0,$1,$1,$2,$3,$3,$4,$5,$5,$6,$7,$7,$8,$9,$9
    ;******
    HexDigit51Table:
    	.db $0,$7,$5,$2,$0,$8,$5,$3,$0,$8,$6,$3,$1,$8,$6,$4
    
    HexDigit52Table:
    	.db $0,$5,$1,$7,$3,$8,$4,$0,$6,$1,$7,$3,$9,$4,$0,$6
    
    HexDigit53Table:
    	.db $0,$8,$7,$5,$4,$2,$1,$0,$8,$7,$5,$4,$2,$1,$0,$8
    
    HexDigit54Table:
    	.db $0,$4,$9,$4,$9,$4,$9,$4,$8,$3,$8,$3,$8,$3,$8,$2
    
    HexDigit55Table:
    	.db $0,$0,$0,$1,$1,$2,$2,$3,$3,$4,$4,$5,$5,$6,$6,$7
    

Categories: [Arithmetic](Category_Arithmetic.xhtml)
