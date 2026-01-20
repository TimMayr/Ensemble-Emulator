# Scanning large tables

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Scanning_large_tables) | View [other pages](Special_AllPages.xhtml#Scanning_large_tables)

Scanning tables larger than 256 bytes is more involved than [ scanning small tables](Scanning_tables.xhtml "Scanning Tables"), because there are no 16-bit equivalents to X and Y. Consider clearing a large block of RAM quickly. A straight-forward approach is to keep a 16-bit index in X and Y, and increment it until it reaches the desired index: 
    
    
    begin = ... ; address of first byte to clear
    count = ... ; number of bytes
    
    .zeropage
    addr:    .res 2
    
    .code
            lda #<begin
            sta addr
            lda #>begin
            sta addr+1
            
            lda #0
            
            ; X and Y form 16-bit index, with Y holding low byte
            ldx #0
            ldy #0
            
    loop:   ; Clear one byte
            sta (addr),y
            
            ; Go to next byte
            iny
            bne nohigh
            inx
            inc addr+1
    nohigh:
            ; See if low and high bytes of index match count
            cpy #<count
            bne loop
            cpx #>count
            bne loop
    

Using the same approach used in [ Efficient Forwards Scanning](Scanning_tables.xhtml#Efficient_Forwards_Scanning "Scanning Tables"), we can make this more efficient. 

First, consider the case of clearing all 64K of RAM. X and Y start out at 0, and they increment up to $FF $FF, then the loop ends. 
    
    
            lda #0
            sta addr
            lda #0
            sta addr+1
            
            lda #0
            
            ldx #0
            ldy #0
            
    loop:   sta (addr),y
            
            iny
            bne nohigh
            inx
            inc addr+1
    nohigh:
            cpy #0
            bne loop
            cpx #0
            bne loop
    

Since we end once X and Y wrap to 0, we can optimize the inner loop to not check X every time: 
    
    
            lda #0
            sta addr
            lda #0
            sta addr+1
            
            lda #0
            
            ldx #0
            ldy #0
            
    loop:   sta (addr),y
            iny
            bne loop
    
            inc addr+1
            inx
            bne loop
    

If we wanted to clear just the last $103 bytes of memory, we'd start with the values the above clear loop has when there are only $103 bytes remaining. So we'd start with X=$FE and Y=$FD, and add $FE00 to addr. This would first clear a byte at $FEFD, then $FEFE, $FEFF, and the 256 bytes beginning at $FF00, as desired: 
    
    
            lda #0
            sta addr
            lda #$FE
            sta addr+1
            
            lda #0
            
            ldx #$FE
            ldy #$FD
            
    loop:   sta (addr),y
            iny
            bne loop
    
            inc addr+1
            inx
            bne loop
    

Now, if we can have the above loop clear the last $103 bytes of the 64K block "beginning" at address 0, we can have it clear the last $103 bytes of a 64K block that "begins" at any address in memory (it would wrap around to the beginning). For example, a 64K block at $1234 extends through $FFFF, then back to $0000 and through $1233, covering 64K total, and the last $103 bytes of this table would go from $1131-$1233. 

Thus, we can use this to clear a block of $103 bytes _anywhere_ in memory; it doesn't matter whether they are the last $103 bytes of a 64K-byte block, since the loop only accesses those $103 bytes. But it does add $FD to the address, since Y starts out with that value, so we must subtract that. The following efficiently clears $103 bytes _anywhere_ in memory: 
    
    
            lda #<(begin - $FD)
            sta addr
            lda #>(begin - $FD)
            sta addr+1
            
            lda #0
            
            ldx #$FE
            ldy #$FD
            
    loop:   sta (addr),y
            iny
            bne loop
            
            inc addr+1
            inx
            bne loop
    

Addr starts out as begin-$FD. The first time through the loop, Y=$FD, so it accesses a byte at begin-$FD+$FD, that is, at begin, as desired. Then Y=$FE, so it accesses begin+1. Then Y=$FF and it accesses begin+2. Y wraps around to 0 and the inner loop falls through to the outer loop, which increments the high byte of addr and increments X to $FF and loops back. Now, Y=0 and addr=begin-$FD+$100, or more simply, begin+3, which is the desired address to be accessing this time through. Y keeps incrementing to $FF, then the inner and outer loops end, and it's cleared exactly $103 bytes of memory. 

The general pattern is to load X with the high byte of $10000-count, Y with the low byte, and also subtract Y's initial value from begin. $10000-count can more simply be calculated as -count, since you get the same result: 
    
    
            lda #<(begin - <-count)
            sta addr
            lda #>(begin - <-count)
            sta addr+1
            
            lda #0
            
            ldx #>-count
            ldy #<-count
            
    loop:   sta (addr),y
            iny
            bne loop
    
            inc addr+1
            inx
            bne loop
    

If the address and/or size isn't known until the program is running, the above must be done at run-time with instructions, rather than relying on the assembler: 
    
    
    .zeropage
    addr:   .res 2
    size:   .res 2
    
    .code
            ; Clears memory from addr through addr+size-1.
            ; Clears all memory if size=0.
            
            ; Adjust addr. Subtracting low byte of negated size is the same
            ; as adding the low byte of size and then a high byte of $FF,
            ; which greatly simplifies this calculation.
            lda addr
            clc
            adc size
            sta addr
            lda addr+1
            adc #$FF
            sta addr+1
            
            ; Subtract size from 0 to negate it, and put that into X and Y
            lda #0
            sec
            sbc size
            tay
            lda #0
            sbc size+1
            tax
            
            lda #0
            
    loop:   sta (addr),y
            iny
            bne loop
    
            inc addr+1
            inx
            bne loop
    

An alternate approach of similar efficienty is a dual loop, with the first handling full 256-byte pages efficiently and the second handling the final partial page: 
    
    
            ; Clears memory from addr through addr+size-1.
            ; Clears nothing if size=0.
            
            lda #0
            
            ldy #0
            
            ; Load page count and handle case where it's zero
            ldx size+1
            beq final
            
            ; Do full pages first
    pages:  sta (addr),y
            iny
            bne pages
            inc addr+1
            dex
            bne pages
            
    final:  ldx size
            beq done
            
            ldy #0
    loop:   sta (addr),y
            iny
            dex
            bne loop
            
    done:
    

This dual-loop approach works if the loop operation is short (here just the STA (addr),Y). If lots is done to each table entry, the previous approach is preferable due to reduced code size and not having to duplicate the code that operates on the table. 
