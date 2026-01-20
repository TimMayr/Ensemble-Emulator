# User:Qalle/Algorithms

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AQalle/Algorithms) | View [other pages](Special_AllPages.xhtml#User_Qalle_Algorithms)

Miscellaneous algorithms that may be useful on the NES. 

## 8-bit unsigned integer to 2 hexadecimal ASCII digits
    
    
    byte2hex:
        ; in: A = unsigned integer
        ; out: X/A = ASCII digits of sixteens/ones in upper case
        pha
        lsr a
        lsr a
        lsr a
        lsr a
        clc
        jsr nybble2hex
        tax
        pla
        and #%00001111
        jsr nybble2hex
        rts
    nybble2hex:
        adc #$30        ; ASCII("0")
        cmp #$3a        ; ASCII("9")+1
        bcc +
        adc #6          ; ASCII("A")-(ASCII("9")+1)-1
    +   rts
    

## 8-bit unsigned integer to 3 decimal ASCII digits

Source: [1]
    
    
    byte2dec:
        ; in: A = unsigned integer
        ; out: Y/X/A = ASCII digits of hundreds/tens/ones
        ldy #$2f  ; ASCII("0")-1
        ldx #$3a  ; ASCII("9")+1
        sec
    -   iny
        sbc #100
        bcs -
    -   dex
        adc #10
        bmi -
        adc #$2f  ; ASCII("0")
        rts
    

## References

  1. ↑ [Tiny .A to ASCII routine](https://codebase64.org/doku.php?id=base:tiny_.a_to_ascii_routine), Codebase64


