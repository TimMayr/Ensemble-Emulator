# Talk:NES 2.0 Mapper 547

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ANES_2.0_Mapper_547) | View [other pages](Special_AllPages.xhtml#Talk_NES_2_0_Mapper_547)

This section appears to be slightly incorrect. 
    
    
    // "row" and "col" are the first and second 7-bit JIS X 0208 code byte, respectively, each minus the $21 offset.
    code = (col %32)		// First, go through 32 columns of a column-third.
          +(row %16)*32 		// Then, through 16 rows of a row-third.
          +(col /32)*32*16		// Then, through three column-thirds.
          +(row /16)*32*16*3	// Finally, through six row-thirds.
    ;
    

I believe it should be this: 
    
    
    // "row" and "col" are the first and second 7-bit JIS X 0208 code byte, respectively, each minus a $20 offset.
    

Alternatively, the same variables could be used: 
    
    
    // "row" and "col" are the first and second 7-bit JIS X 0208 code byte, respectively.
    code = (col %32)		// First, go through 32 columns of a column-third.
          +(row %16)*32 		// Then, through 16 rows of a row-third.
          +((col /32)-1)*32*16	// Then, through three column-thirds. (-1 performs -0x20 from value written to 0xDC00)
          +((row /16)-2)*32*16*3	// Finally, through six row-thirds. (-2 performs -0x20 from value written to 0xDD00)
    ;
    

If I understand the code correctly, it's only using 3/4 of column and row spaces in translating from Kanji. The values it is dropping are for bit pattern {0,0}, and using {0,1} {1,0} and {1,1}. By subtracting 1 for each, the patterns then become {0,0}, {0,1}, and {1,0}, and multiplying by 3 will then line up correctly. In implementing this for MiSTer, it wouldn't work until I did the equivalent of subtracting 0x20 from each. 

Technically, I just did this (as a slightly larger table is more efficient than the subtraction logic in MiSTer): 
    
    
    code = (col %32)		// First, go through 32 columns of a total column.
          +(row %16)*32 		// Then, through 16 rows of a total row.
          +(col /32)*32*16		// Then, through 4 columns.
          +(row /16)*32*16*4	// Finally, through 8 rows.
    ;
    static const uint8_t pageTable[0x40] ={
     	0x0,0x0, 0x0,0x0,0x0,0x0,0x0,0x0, 0x0,0x0, 0x0,0x0,0x0,0x0,0x0,0x0,                                   // JIS X 0208 rows $00-$1F. These are not valid row numbers. 
    	0x0,0x0, 0x0,0x0,0x2,0x2,0x1,0x1, 0x0,0x0, 0x4,0x5,0x6,0x7,0x8,0x9, 0x0,0x0, 0xA,0xB,0xC,0xD,0xE,0xF, // JIS X 0208 rows $20-$4F. $20 is not a valid row number.
    	0x0,0x0, 0x0,0x1,0x2,0x3,0x4,0x5, 0x0,0x0, 0x6,0x7,0x8,0x9,0xA,0xB, 0x0,0x0, 0xC,0xD,0xE,0xF,0xD,0xD  // JIS X 0208 rows $50-$7F. $7F is not a valid row number.
    };
    

Note the location of the extra 0s. Those are the ones that need to be dropped if using "thirds". ie The first row (1/4) is dropped by subtracting 0x20 from the row, and the extra zeroes in the other two rows (1/4) are removed by subtracting 0x20 from the column. 
