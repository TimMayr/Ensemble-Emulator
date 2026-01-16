# Calculate CRC32

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Calculate_CRC32) | View [other pages](Special_AllPages.xhtml#Calculate_CRC32)

The following code from Kevin Horton can be used to calculate a [CRC32 checksum](http://en.wikipedia.org/wiki/Cyclic_redundancy_check). 

## Details

  * Variable **testcrc** should be in zero page and (obviously) requires 4 bytes of space
  * The "magic value" $EDB88320 (bytes $ED, $B8, $83, and $20) are the polynomial



## Code
    
    
    ;usage:
    ;
    ;initialize by calling crc32init
    ;
    ;feed bytes to crc32
    ;
    ;finish by calling crc32end
    ;
    ;result is in testcrc0-3
    ;
    
    crc32init:   ldx #3
                 lda #$ff
    
    c3il:        sta testcrc+0,x
                 dex
                 bpl c3il
                 rts
    
    
    crc32:       ldx #8
                 eor testcrc+0
                 sta testcrc+0
    
    c32l:        lsr testcrc+3
                 ror testcrc+2
                 ror testcrc+1
                 ror testcrc+0
                 bcc dc32
                 lda #$ed
                 eor testcrc+3
                 sta testcrc+3
                 lda #$b8
                 eor testcrc+2
                 sta testcrc+2
                 lda #$83
                 eor testcrc+1
                 sta testcrc+1
                 lda #$20
                 eor testcrc+0
                 sta testcrc+0
    
    dc32:        dex
                 bne c32l
                 rts
    
    crc32end:    ldx #3
    
    c3el:        lda #$ff
                 eor testcrc+0,x
                 sta testcrc+0,x
                 dex
                 bpl c3el
                 rts
    

## References

  * <http://mdfs.net/Info/Comp/Comms/CRC32.htm>
  * <http://www.6502.org/source/integers/crc.htm>



Categories: [Arithmetic](Category_Arithmetic.xhtml)
