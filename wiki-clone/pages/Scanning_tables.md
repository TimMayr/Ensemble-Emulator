# Scanning tables

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Scanning_tables) | View [other pages](Special_AllPages.xhtml#Scanning_tables)

There are several ways to scan a table that is 256 bytes or smaller, thanks to the many indexed addressing modes available (for larger tables, see [Scanning Large Tables](Scanning_large_tables.xhtml "Scanning Large Tables")). Scanning could be reading values looking for a particular one, writing multiple values, copying, etc. Some are more efficient than others, or simpler to code. 

## Common approaches

For example, this code fills a table of _size_ bytes with 0, from beginning to end: 
    
    
            lda #0
            ldy #0
    loop:   sta table,y       ; Y = 0 through size-1
            iny
            cpy #size
            bne loop
    

It's often more efficient to scan backwards, since DEY and similar instructions implicitly compare the decremented value with zero. Since it doesn't matter which direction we fill a table, backwards is preferable: 
    
    
            lda #0
            ldy #size
    loop:   dey               ; decrement BEFORE sta
            sta table,y       ; Y = size-1 through 0
            bne loop
    

Note how the DEY is before the STA. If it were after, once Y reached 0, the loop would end, even though the STA would never have been executed with Y=0. 

If we're doing something besides simply filling, we might not be able to put the DEY before the table operation. For example, if we're finding the sum of all values in the table using ADC, the ADC will alter flags. We could add a CPY #0 just before the branch, but then we're back with code similar to the original code. If the table is 128 bytes or smaller, we can instead use the BPL instruction: 
    
    
            lda #0
            ldy #size - 1     ; size must be 128 or less
    loop:   clc
            adc table,y       ; Y = size-1 through 0
            dey
            bpl loop
    

Once DEY decrements Y to 0, BPL still branches, since 0 is considered positive. The loop then executes a final time, with Y=0. Then DEY decrements it to $FF (-1), and BPL falls through. 

If the table is more than 128 bytes, we can still use BNE, and adjust the table address to compensate: 
    
    
            lda #0
            ldy #size
    loop:   clc
            adc table-1,y     ; Y = size through 1
            dey
            bne loop
    

## Efficient forwards scanning

Sometimes a table _must_ be scanned forwards, not backwards. For example, we might want to find the first occurrence of a particular byte equal to A: 
    
    
            ldy #0
    loop:   cmp table,y       ; Y = 0 through size-1
            beq found
            iny
            cpy #size
            bne loop
            
    found:  ...
    

The original approach works, but there is one as efficient as the backwards approach. First, consider scanning a 256-byte table forwards. We don't need a compare since Y will wrap around to 0 on the 256th iteration. So this is as fast as the backwards version: 
    
    
            ldy #0
    loop:   cmp table,y       ; Y = 0 through $FF
            beq found
            iny
            bne loop
    

Now, let's say we only want to scan the last 3 bytes of table. We would simply start the loop with Y set to $FD rather than 0. It would scan entries $FD, $FE, and $FF, then end (or find what it was looking for): 
    
    
            ldy #$FD
    loop:   cmp table,y       ; Y = $FD through $FF
            beq found
            iny
            bne loop
    

Since we can use the above loop to scan the last 3 bytes of table, we can use it to scan _any_ three bytes of memory; it doesn't matter whether they are the last 3 bytes of a 256-byte table, or a table that's only 3 bytes long. But it does add $FD to whatever table address we use, so we'll need to subtract that. The following scans a 3-byte table forwards for a value: 
    
    
            ldy #$FD
    loop:   cmp table - $FD,y          ; Y = $FD through $FF
            beq found
            iny
            bne loop
    

The general pattern is to load Y with the $100-size, and subtract that same value from table: 
    
    
            ldy #$100-size
    loop:   cmp table - ($100-size),y       ; Y = $100-size through $FF
            beq found
            iny
            bne loop
    

There is a slight overhead introduced by this approach: since most indexed instructions take an extra cycle if the low byte of the address added to the index exceeds $FF, this is likely to introduce an extra cycle. Using the 3-byte table example, if table's address were $10ED, the compare would assemble to CMP $0FF0,Y. Then, with Y=$FD, there would be a page crossing, adding an extra cycle. Since this approach uses the upper portion of Y's range, rather than the lower portion, it's more likely to cause page crossings. However, one cycle due to a page crossing is less than two cycles due to an immediate compare. 

If size is less than 128, Y going negative could stop the loop, allowing the typical offset to the table halved, reducing the chance of a page crossing: 
    
    
            ldy #$80-size
    loop:   cmp table - ($80-size),y       ; Y = $80-size through $7F
            beq found
            iny
            bpl loop
    

This approach has the efficiency of the decrement approach while avoiding accessing the table in reverse. One downside is that finding the actual index of the current byte requires some calculation. Also, since it's somewhat tricky to code, backwards is still preferred where the access order doesn't matter. Don't overlook the possibility of storing the table itself in reverse order so that the reverse approach can be used. 
