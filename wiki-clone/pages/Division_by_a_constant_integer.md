# Division by a constant integer

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Division_by_a_constant_integer) | View [other pages](Special_AllPages.xhtml#Division_by_a_constant_integer)

## Contents

  * 1 Divide by a power of two
  * 2 Division by a constant
  * 3 Chain divide by a variable
  * 4 Resources
  * 5 See also



## Divide by a power of two

In binary arithmetic, division by 2^n is equivalent to shifting right n times. For this reason, it is recommended that each NES project is designed in a way that takes advantage of this fact. The remainder/modulus of the division operation can be easily obtained by ANDing the original value with (2^n)-1. 

For signed numbers, it is required that the bit shifting "in" the number at the left is the previous sign bit and not a '0'. This is commonly called an "arithmetic shift right" as opposed to a "logical shift right". Since the 6502 doesn't have any arithmetic shift right instruction, it can be achieved with the following [synthetic instruction](Synthetic_instructions.xhtml#Arithmetic_shift_right "Synthetic instructions"): 
    
    
        cmp #$80
        ror A
    

This will be true for all divisions discussed in this article, which focuses on unsigned numbers. 

## Division by a constant

When doing a division with a constant denominator, it is possible take advantage of this to optimize code (as opposed to use the general purpose [division by 2 variables](8_bit_Divide.xhtml "8-bit Divide")). 

First thing is to decompose the constant number into sum-of-power-of-two (i.e. write it in binary form). It then needs to be determined how many bits are needed to hold the result. Let's call the number of bits n, and let's call the constant divisor c. For each bit k, compare the variable it to c<<k (= c*2^k) (in the order k = n-1, n-2, ... downto 0 included). If the comparison bit is set, subtract c<<k, and in all cases rotate the result left (note that after the subtraction c will always be set). For example this division code divides the variable in A by 14 and keeps the lower 4 bit of results. 
    
    
    ;Division by 14
          pha
          lda #$00
          sta Res      ;Init the res variable (needed because we're doing less than 8 shifts)
          pla
          cmp #$70     ;Compare to 14<<3 and set bit
          bcc + 
          sbc #$70 
    +     rol Res 
          cmp #$38     ;14<<2
          bcc + 
          sbc #$38 
    +     rol Res 
          cmp #$1c     ;14<<1
          bcc + 
          sbc #$1c 
    +     rol Res 
          cmp #$0e     ;14<<0
          bcc + 
          sbc #$0e 
    +     rol Res      ;A = remainder, Res = quotient
    

Of course the result will only be correct if if fits in 4-bit in the above example (because it does 4 compare/shift operations), if a larger number of bits is expected, the code should be adapted to take that into account. 

See also: [Unsigned Integer Division Routines](http://forums.nesdev.org/viewtopic.php?f=2&t=11336) \- NESDev forum post with a collection of efficient 8-bit division by constant routines. 

## Chain divide by a variable

If an algorithm does a really great number of divisions by a variable, but that the value of the variable is constant for the whole algorithm, it could be faster to write a code that generate the above code in RAM and execute it that way instead of doing the slower variable / variable code. 

## Resources

  * [Unsigned Integer Division Routines](https://forums.nesdev.org/viewtopic.php?t=11336) forum post by Omegamatrix with routines for many constants.



## See also

  * [Divide by 3](Divide_by_3.xhtml "Divide by 3")



Categories: [Arithmetic](Category_Arithmetic.xhtml)
