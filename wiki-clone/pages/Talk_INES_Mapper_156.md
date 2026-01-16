# Talk:INES Mapper 156

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_156) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_156)

the split chr hi/lo registers seem to be: 

lo: 8 bits hi: 1 bit, d0 

borrowing from the vrc2/4 article: 

### CHR Select 0 low($C000), high($C004)
    
    
      $C004        $C000
    7  bit  0    7  bit  0
    ---------    ---------
    .... ...H    LLLL LLLL
            |    |||| ||||
            |    ++++ ++++- Low 8 bits
            +-------------- High bit of 1 KiB CHR bank at PPU $0000
    

  
this info was taken from the fceu source for the mapper and inspected in the debugger with general's son running 
