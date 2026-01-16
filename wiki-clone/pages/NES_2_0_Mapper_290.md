# NES 2.0 Mapper 290

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_290) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_290)

NES 2.0 Mapper 290 is used for the _Asder 20-in-1_ multicart. Its UNIF board name is **BMC-NTD-03**. 

## Address latch
    
    
    Mask: $8000
    
    $8000: A~[1PPP PMCC Sp.. .CCC]
               ||| |||| ||    |||
               ||| ||++-------+++- Select 8 KiB CHR-ROM bank at PPU $0000-$1FFF
               ||| ||   |+-------- Select 16 KiB PRG-ROM bank at CPU $8000/$C000 if S=1
               ||| ||   +--------- Select PRG-ROM bank size, 0=32 KiB, 1=16 KiB
               ||| |+------------- Select nametable mirroring, 0=vertical, 1=horizontal 
               +++-+-------------- Select 32 KiB PRG-ROM bank at CPU $8000
    

## Notes

  * The game _Benico_ on that multicart has graphical glitches during attract mode on real hardware.



Categories: [Multicart mappers](Category_Multicart_mappers.xhtml)
