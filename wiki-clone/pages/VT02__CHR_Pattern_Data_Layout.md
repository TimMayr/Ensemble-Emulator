# VT02+ CHR Pattern Data Layout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/VT02%2B_CHR_Pattern_Data_Layout) | View [other pages](Special_AllPages.xhtml#VT02__CHR_Pattern_Data_Layout)

The layout of CHR pattern data differs between the [different video modes](VT02__Video_Modes.xhtml "VT02+ Video Modes") available on the [VTxx](VTxx.xhtml "VTxx") series of Famiclone consoles. 

# Two bits per pixel

In 2bpp mode, CHR pattern data layout is the [same as on the original NES/Famicom](PPU_pattern_tables.xhtml "PPU pattern tables"), with each tile's CHR data occupying sixteen bytes, and the two bitplanes' data stored sequentially: 
    
    
    Byte Bit Meaning                Byte Bit Meaning
    ----------------                ----------------
    0    7   pixel Y=0 X=0, bit 0   8    7   pixel Y=0 X=0, bit 1
    0    6   pixel Y=0 X=1, bit 0   8    6   pixel Y=0 X=1, bit 1
    0    5   pixel Y=0 X=2, bit 0   8    5   pixel Y=0 X=2, bit 1
    0    4   pixel Y=0 X=3, bit 0   8    4   pixel Y=0 X=3, bit 1
    0    3   pixel Y=0 X=4, bit 0   8    3   pixel Y=0 X=4, bit 1
    0    2   pixel Y=0 X=5, bit 0   8    2   pixel Y=0 X=5, bit 1
    0    1   pixel Y=0 X=6, bit 0   8    1   pixel Y=0 X=6, bit 1
    0    0   pixel Y=0 X=7, bit 0   8    0   pixel Y=0 X=7, bit 1
    1    7   pixel Y=1 X=0, bit 0   9    7   pixel Y=1 X=0, bit 1
    1    6   pixel Y=1 X=1, bit 0   9    6   pixel Y=1 X=1, bit 1
    1    5   pixel Y=1 X=2, bit 0   9    5   pixel Y=1 X=2, bit 1
    1    4   pixel Y=1 X=3, bit 0   9    4   pixel Y=1 X=3, bit 1
    1    3   pixel Y=1 X=4, bit 0   9    3   pixel Y=1 X=4, bit 1
    1    2   pixel Y=1 X=5, bit 0   9    2   pixel Y=1 X=5, bit 1
    1    1   pixel Y=1 X=6, bit 0   9    1   pixel Y=1 X=6, bit 1
    1    0   pixel Y=1 X=7, bit 0   9    0   pixel Y=1 X=7, bit 1
    2    7   pixel Y=2 X=0, bit 0   A    7   pixel Y=2 X=0, bit 1
    :    :   :                      :    :   :
    7    7   pixel Y=7 X=7, bit 0   F    7   pixel Y=7 X=7, bit 1
    
    All numbers hexadecimal.
    

# Four bits per pixel, 8 bit data bus

Available on the VT03 and later consoles, the 4bpp mode expands the same plane-sequential system, so that each tile's CHR data now occupies thirty-two bytes: 
    
    
    Byte Bit Meaning                Byte Bit Meaning                Byte Bit Meaning                 Byte Bit Meaning                        
    ----------------                ----------------                ----------------                 ----------------                
    00   7   pixel Y=0 X=0, bit 0   08   7   pixel Y=0 X=0, bit 1   10   7   pixel Y=0 X=0, bit 2    18   7   pixel Y=0 X=0, bit 3   
    00   6   pixel Y=0 X=1, bit 0   08   6   pixel Y=0 X=1, bit 1   10   6   pixel Y=0 X=1, bit 2    18   6   pixel Y=0 X=1, bit 3   
    00   5   pixel Y=0 X=2, bit 0   08   5   pixel Y=0 X=2, bit 1   10   5   pixel Y=0 X=2, bit 2    18   5   pixel Y=0 X=2, bit 3   
    00   4   pixel Y=0 X=3, bit 0   08   4   pixel Y=0 X=3, bit 1   10   4   pixel Y=0 X=3, bit 2    18   4   pixel Y=0 X=3, bit 3   
    00   3   pixel Y=0 X=4, bit 0   08   3   pixel Y=0 X=4, bit 1   10   3   pixel Y=0 X=4, bit 2    18   3   pixel Y=0 X=4, bit 3   
    00   2   pixel Y=0 X=5, bit 0   08   2   pixel Y=0 X=5, bit 1   10   2   pixel Y=0 X=5, bit 2    18   2   pixel Y=0 X=5, bit 3   
    00   1   pixel Y=0 X=6, bit 0   08   1   pixel Y=0 X=6, bit 1   10   1   pixel Y=0 X=6, bit 2    18   1   pixel Y=0 X=6, bit 3   
    00   0   pixel Y=0 X=7, bit 0   08   0   pixel Y=0 X=7, bit 1   10   0   pixel Y=0 X=7, bit 2    18   0   pixel Y=0 X=7, bit 3   
    01   7   pixel Y=1 X=0, bit 0   09   7   pixel Y=1 X=0, bit 1   11   7   pixel Y=1 X=0, bit 2    19   7   pixel Y=1 X=0, bit 3   
    01   6   pixel Y=1 X=1, bit 0   09   6   pixel Y=1 X=1, bit 1   11   6   pixel Y=1 X=1, bit 2    19   6   pixel Y=1 X=1, bit 3   
    01   5   pixel Y=1 X=2, bit 0   09   5   pixel Y=1 X=2, bit 1   11   5   pixel Y=1 X=2, bit 2    19   5   pixel Y=1 X=2, bit 3   
    01   4   pixel Y=1 X=3, bit 0   09   4   pixel Y=1 X=3, bit 1   11   4   pixel Y=1 X=3, bit 2    19   4   pixel Y=1 X=3, bit 3   
    01   3   pixel Y=1 X=4, bit 0   09   3   pixel Y=1 X=4, bit 1   11   3   pixel Y=1 X=4, bit 2    19   3   pixel Y=1 X=4, bit 3   
    01   2   pixel Y=1 X=5, bit 0   09   2   pixel Y=1 X=5, bit 1   11   2   pixel Y=1 X=5, bit 2    19   2   pixel Y=1 X=5, bit 3   
    01   1   pixel Y=1 X=6, bit 0   09   1   pixel Y=1 X=6, bit 1   11   1   pixel Y=1 X=6, bit 2    19   1   pixel Y=1 X=6, bit 3   
    01   0   pixel Y=1 X=7, bit 0   09   0   pixel Y=1 X=7, bit 1   11   0   pixel Y=1 X=7, bit 2    19   0   pixel Y=1 X=7, bit 3   
    02   7   pixel Y=2 X=0, bit 0   0A   7   pixel Y=2 X=0, bit 1   12   7   pixel Y=2 X=0, bit 2    1A   7   pixel Y=2 X=0, bit 3   
     :   :   :                       :   :   :                       :   :   :                        :   :   :                      
    07   7   pixel Y=7 X=7, bit 0   0F   7   pixel Y=7 X=7, bit 1   17   7   pixel Y=7 X=7, bit 2    1F   7   pixel Y=7 X=7, bit 3   
    
    All numbers hexadecimal.
    

# Four bits per pixel, 16 bit data bus

The VT16 (and presumably the VT09) offer an alternative 4bpp mode that allows two bytes to be fetched in one 16 bit read operation. Its purpose is described as allowing the use of slower memory chips. It resembles the Super NES 4bpp tile format except that bit planes 1 and 2 of each tile are swapped. 
    
    
    Byte  Bit  Meaning                Byte  Bit  Meaning             
    ------------------                ------------------      
    00/01 F    pixel Y=0 X=0, bit 2   10/11 F    pixel Y=0 X=0, bit 3
    00/01 E    pixel Y=0 X=1, bit 2   10/11 E    pixel Y=0 X=1, bit 3
    00/01 D    pixel Y=0 X=2, bit 2   10/11 D    pixel Y=0 X=2, bit 3
    00/01 C    pixel Y=0 X=3, bit 2   10/11 C    pixel Y=0 X=3, bit 3
    00/01 B    pixel Y=0 X=4, bit 2   10/11 B    pixel Y=0 X=4, bit 3
    00/01 A    pixel Y=0 X=5, bit 2   10/11 A    pixel Y=0 X=5, bit 3
    00/01 9    pixel Y=0 X=6, bit 2   10/11 9    pixel Y=0 X=6, bit 3
    00/01 8    pixel Y=0 X=7, bit 2   10/11 8    pixel Y=0 X=7, bit 3
    00/01 7    pixel Y=0 X=0, bit 0   10/11 7    pixel Y=0 X=0, bit 1
    00/01 6    pixel Y=0 X=1, bit 0   10/11 6    pixel Y=0 X=1, bit 1
    00/01 5    pixel Y=0 X=2, bit 0   10/11 5    pixel Y=0 X=2, bit 1
    00/01 4    pixel Y=0 X=3, bit 0   10/11 4    pixel Y=0 X=3, bit 1
    00/01 3    pixel Y=0 X=4, bit 0   10/11 3    pixel Y=0 X=4, bit 1
    00/01 2    pixel Y=0 X=5, bit 0   10/11 2    pixel Y=0 X=5, bit 1
    00/01 1    pixel Y=0 X=6, bit 0   10/11 1    pixel Y=0 X=6, bit 1
    00/01 0    pixel Y=0 X=7, bit 0   10/11 0    pixel Y=0 X=7, bit 1
    02/03 F    pixel Y=1 X=0, bit 2   12/13 F    pixel Y=1 X=0, bit 3
    02/03 E    pixel Y=1 X=1, bit 2   12/13 E    pixel Y=1 X=1, bit 3
    02/03 D    pixel Y=1 X=2, bit 2   12/13 D    pixel Y=1 X=2, bit 3
    02/03 C    pixel Y=1 X=3, bit 2   12/13 C    pixel Y=1 X=3, bit 3
    02/03 B    pixel Y=1 X=4, bit 2   12/13 B    pixel Y=1 X=4, bit 3
    02/03 A    pixel Y=1 X=5, bit 2   12/13 A    pixel Y=1 X=5, bit 3
    02/03 9    pixel Y=1 X=6, bit 2   12/13 9    pixel Y=1 X=6, bit 3
    02/03 8    pixel Y=1 X=7, bit 2   12/13 8    pixel Y=1 X=7, bit 3
    02/03 7    pixel Y=1 X=0, bit 0   12/13 7    pixel Y=1 X=0, bit 1
    02/03 6    pixel Y=1 X=1, bit 0   12/13 6    pixel Y=1 X=1, bit 1
    02/03 5    pixel Y=1 X=2, bit 0   12/13 5    pixel Y=1 X=2, bit 1
    02/03 4    pixel Y=1 X=3, bit 0   12/13 4    pixel Y=1 X=3, bit 1
    02/03 3    pixel Y=1 X=4, bit 0   12/13 3    pixel Y=1 X=4, bit 1
    02/03 2    pixel Y=1 X=5, bit 0   12/13 2    pixel Y=1 X=5, bit 1
    02/03 1    pixel Y=1 X=6, bit 0   12/13 1    pixel Y=1 X=6, bit 1
    02/03 0    pixel Y=1 X=7, bit 0   12/13 0    pixel Y=1 X=7, bit 1
      :   :    :                        :   :    :
    0E/0F 0    pixel Y=7 X=7, bit 0   1E/1F 8    pixel Y=7 X=7, bit 1
    
    All numbers hexadecimal. Little-endian byte order.
    
