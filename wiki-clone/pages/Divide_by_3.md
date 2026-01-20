# Divide by 3

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Divide_by_3) | View [other pages](Special_AllPages.xhtml#Divide_by_3)

## Contents

  * 1 16-bit dividend, no remainder
  * 2 8-bit dividend, no remainder
  * 3 References
  * 4 See also



A lot of times, you need to divide something by 3. One way is to multiply by $55. 

## 16-bit dividend, no remainder
    
    
     ; divide 16 bit number by 3 by multiplying by 1/3
     ; enter with
     ; A containing the hi byte of the number to be divided by 3
     ; Y containing the lo byte of the number to be divided by 3
     ; the hi byte of the partial product is kept in A or saved
     ; on the stack when neccessary
     ; the product (N/3 quotient) is returned hi byte in A,
     ; lo byte in Y
    
    .proc div3_ay
    
     ; save the number in lo_temp, hi_temp
    
     sty lo_temp
     sty lo_product
     sta hi_temp
    
     ldy #$09
     clc
     bcc ENTER
    
     ; each pass through loop adds the number in
     ; lo_temp, hi_temp to the partial product and
     ; then divides the partial product by 4
    
    LOOP:
     pha
     lda lo_product
     adc lo_temp
     sta lo_product
     pla
     adc hi_temp
    ENTER:
     ror
     ror lo_product
     lsr
     ror lo_product
     dey
     bne LOOP
     ldy lo_product
     rts
    .endproc
    

## 8-bit dividend, no remainder
    
    
     ; enter with number to be divided in A
     ; answer returned in A
    .proc div3_a
     sta temp
     lsr
     lsr
     adc temp
     ror
     lsr
     adc temp
     ror
     lsr
     adc temp
     ror
     lsr
     adc temp
     ror
     lsr
     rts
    .endproc
    

## References

  * [bogax's post](http://forums.nesdev.org/viewtopic.php?p=74242#p74242)



## See also

  * [Division by a constant integer#Division by a constant](Division_by_a_constant_integer.xhtml#Division_by_a_constant "Division by a constant integer")



Categories: [Arithmetic](Category_Arithmetic.xhtml)
