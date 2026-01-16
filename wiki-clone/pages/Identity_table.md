# Identity table

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Identity_table) | View [other pages](Special_AllPages.xhtml#Identity_table)

An identity table is simply a table with bytes 0 to 255 in ascending order, usually in PRG ROM. The value at any index is equal to the index: 
    
    
    identity_table:
        .byte $00,$01,$02,$03,$04,$05,$06,$07,$08,$09,$0a,$0b,$0c,$0d,$0e,$0f
        .byte $10,$11,$12,$13,$14,$15,$16,$17,$18,$19,$1a,$1b,$1c,$1d,$1e,$1f
        .byte $20,$21,$22,$23,$24,$25,$26,$27,$28,$29,$2a,$2b,$2c,$2d,$2e,$2f
        .byte $30,$31,$32,$33,$34,$35,$36,$37,$38,$39,$3a,$3b,$3c,$3d,$3e,$3f
        .byte $40,$41,$42,$43,$44,$45,$46,$47,$48,$49,$4a,$4b,$4c,$4d,$4e,$4f
        .byte $50,$51,$52,$53,$54,$55,$56,$57,$58,$59,$5a,$5b,$5c,$5d,$5e,$5f
        .byte $60,$61,$62,$63,$64,$65,$66,$67,$68,$69,$6a,$6b,$6c,$6d,$6e,$6f
        .byte $70,$71,$72,$73,$74,$75,$76,$77,$78,$79,$7a,$7b,$7c,$7d,$7e,$7f
        .byte $80,$81,$82,$83,$84,$85,$86,$87,$88,$89,$8a,$8b,$8c,$8d,$8e,$8f
        .byte $90,$91,$92,$93,$94,$95,$96,$97,$98,$99,$9a,$9b,$9c,$9d,$9e,$9f
        .byte $a0,$a1,$a2,$a3,$a4,$a5,$a6,$a7,$a8,$a9,$aa,$ab,$ac,$ad,$ae,$af
        .byte $b0,$b1,$b2,$b3,$b4,$b5,$b6,$b7,$b8,$b9,$ba,$bb,$bc,$bd,$be,$bf
        .byte $c0,$c1,$c2,$c3,$c4,$c5,$c6,$c7,$c8,$c9,$ca,$cb,$cc,$cd,$ce,$cf
        .byte $d0,$d1,$d2,$d3,$d4,$d5,$d6,$d7,$d8,$d9,$da,$db,$dc,$dd,$de,$df
        .byte $e0,$e1,$e2,$e3,$e4,$e5,$e6,$e7,$e8,$e9,$ea,$eb,$ec,$ed,$ee,$ef
        .byte $f0,$f1,$f2,$f3,$f4,$f5,$f6,$f7,$f8,$f9,$fa,$fb,$fc,$fd,$fe,$ff
    

Using the identity table, new assembly instructions can be synthesized as follows: 

`ldx identity_table,y` | `tyx`  
---|---  
`ldy identity_table,x` | `txy`  
`and identity_table,x` | `and X`  
`and identity_table,y` | `and Y`  
`ora identity_table,x` | `ora X`  
`ora identity_table,y` | `ora Y`  
`eor identity_table,x` | `eor X`  
`eor identity_table,y` | `eor Y`  
`adc identity_table,x` | `adc X`  
`adc identity_table,y` | `adc Y`  
`sbc identity_table,x` | `sbc X`  
`sbc identity_table,y` | `sbc Y`  
`cmp identity_table,x` | `cmp X`  
`cmp identity_table,y` | `cmp Y`  
`bit identity_table+_constant_` | `bit #_constant_`  
  
All the instructions above take 3 bytes and 4 CPU cycles, assuming the table is page-aligned (starts at `$xx00`) and not on zero page. (Crossing a page boundary causes slowdown with indexed addressing modes.) 

[1]

## Constant offsets with two identity tables

By defining another identity table immediately after the first one, the values of X and Y can be offset by a constant, at the cost of 255 more ROM bytes: 
    
    
    identity_table1:
        .byte $00, $01, ..., $ff  ; as above
    identity_table2:
        .byte $00, $01, ..., $fe  ; $ff not needed
    

For example: 

  * `adc identity_table1+_constant_ ,x` is equivalent to `adc x+_constant_` where _constant_ = 0-255
  * `adc identity_table2-_constant_ ,x` is equivalent to `adc x-_constant_` where _constant_ = 1-255



[2]

## See also

  * [6502 assembly optimisations](6502_assembly_optimisations.xhtml "6502 assembly optimisations")
  * [Synthetic instructions](Synthetic_instructions.xhtml "Synthetic instructions")



## References

  1. ↑ [NESDev forum - "6502 ASM trick" \- post by tokumaru](http://forums.nesdev.org/viewtopic.php?f=2&t=3682)
  2. ↑ [NESDev forum - "6502 ASM trick" \- post by Jarhmander](https://forums.nesdev.org/viewtopic.php?p=127081#p127081)


