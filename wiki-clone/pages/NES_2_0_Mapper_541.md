# NES 2.0 Mapper 541

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_541) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_541)

NES 2.0 Mapper 541 denotes the Korean _리틀컴 (LittleCom) 160-in-1_ multicart. 

## Address latch ($C000-$FFFF, write)
    
    
    Mask: $C000
    
    A~FEDC BA98 7654 3210
      -------------------
      11.. .... PPPP PpBM
                |||| |||+- Nametable mirroring
                |||| |||    0: Horizontal
                |||| |||    1: Vertical
                |||| ||+-- Set PRG A14 behavior
                |||| ||     0: PRG A14=CPU A14 (NROM-256)
                |||| ||     1: PRG A14=p (NROM-128)
                ++++-++--- Set 16 KiB PRG-ROM bank (PRG A14..A19)
                           at CPU $8000-$BFFF/$C000-$FFFF,
                           p replaced with CPU A14 if B=0
    

## Notes

  * CHR pattern data is stored in 8 KiB of unbanked CHR-RAM.
  * Some games write to the unmapped CPU address range $8000-$BFFF. These writes have no effect.
  * The mirroring bit works in the opposite way compared to most mappers.



Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
