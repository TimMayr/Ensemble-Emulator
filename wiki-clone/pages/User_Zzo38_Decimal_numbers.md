# User:Zzo38/Decimal numbers

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AZzo38/Decimal_numbers) | View [other pages](Special_AllPages.xhtml#User_Zzo38_Decimal_numbers)

Converting a 16-bit number into decimal format. Assume the low 8-bits in X and the high 8-bits in Y. Also assume at the beginning of the pattern table (whichever one is going to be used for displaying these numbers) has numbers 0 to 9 repeated a few times. You will have tables `DL0`, `DL1`, `DH0`, etc for converting one digit into the decimal (each table is 256 bytes long). 
    
    
    	CLC
    	LDA DL0,X
    	ADC DH0,Y
    	STA <DIGIT0
    	CMP #10
    	LDA DL1,X
    	ADC DH1,Y
    	STA <DIGIT1
    	CMP #10
    	LDA DL2,X
    	ADC DH2,Y
    	STA <DIGIT2
    	CMP #10
    	LDA #0
    	ADC DH3,Y
    	STA <DIGIT3
    	CMP #10
    	LDA #0
    	ADC DH4,Y
    	STA <DIGIT4
    

If the numbers are not at the beginning of the pattern table, you can alter the tables for converting the high 8-bits into decimal, and change `#10` into the different number by the bias of the carry that is used in this case. 

Categories: [Arithmetic](Category_Arithmetic.xhtml)
