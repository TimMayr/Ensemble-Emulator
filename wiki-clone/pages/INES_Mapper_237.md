# INES Mapper 237

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_237) | View [other pages](Special_AllPages.xhtml#INES_Mapper_237)

iNES Mapper 237 represents the Teletubbies 420-in-1 multicart. 

No bus conflicts on writes: 
    
    
           address           data
     15 12   8    4    0  7  bit  0
     ---- ---- ---- ----  ---- ----
     1... .... .... .BLT  mtMB Bbbb
                     |||  |||| ||||
                     |||  |||| |+++-- inner 16 KiB bank
                     +-------+-+----- outer 128 KiB bank
                      ||  ||+-------- mirroring (0:horizontal/A11  1:vertical/A10)
                      ||  |+--------- transparency (0:PRG A14 is connected to lsb of latch  1:PRG A14 is connected to CPU A14 )
                      ||  +---------- mode (0:UNROM-PRG A14..A16 is bbb ORed with CPU A14  1:NROM-PRG A14..A16 is bbb )
                      |+------------- type (0:normal execution  1:PRG A1 is always 1 so CPU must execute from RAM)
                      +-------------- lock (0:allow further writes  1:only writes to bbb are allowed
    

The `mt` bits interact, so it might be more convenient to think of them together: 

mt | $8000 | $C000 | in words   
---|---|---|---  
00 | <BBBbbb> | <BBB111> | 128 KiB UNROM   
40 | <BBBbb0> | <BBB111> | defective interaction; UNROM but the lsb of the latch is ignored and treated as 0   
80 | <BBBbbb> | <BBBbbb> | 16 KiB NROM (PRG A14 is connected to lsb of latch)   
C0 | <BBBbb0> | <BBBbb1> | 32 KiB NROM (PRG A14 is connected to CPU A14)   
  
Note: the "type" bit probably depends on the specific cartridge. Implementing this differently will produce a different-seeming multicart. The software pretends that setting the Type bit replaces the entire ROM with a byte that specifies the kind of multicart present. 

The L, T, m, and B bits are cleared on all resets. The others (t, M, and b) may be also. 

See also: 

  * [http://forums.nesdev.org/viewtopic.php?f=3&t=5977](http://forums.nesdev.org/viewtopic.php?f=3&t=5977)
  * <http://bootgod.dyndns.org:7777/downloads/420IN1.TXT>



Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
