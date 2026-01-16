# NES 2.0 Mapper 554

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_554) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_554)

**NES 2.0 Mapper 554** denotes the Kaiser **KS-7010** circuit board, used for one of several cartridge conversions of the FDS version of _悪魔城 Dracula_ , the Japanese version of _Castlevania_. Bankswitching is unusual in being triggered entirely through _reads_ from "magic" addresses. The board mounts 128 KiB PRG-ROM, 128 KiB CHR-ROM, a KS-201 ASIC and a 74'32. 

# Banks

  * CPU $6000-$7FFF: Switchable 8 KiB PRG-ROM bank
  * CPU $8000-$9FFF: Fixed 8 KiB PRG-ROM bank #10
  * CPU $A000-$BFFF: Fixed 8 KiB PRG-ROM bank #11
  * CPU $C000-$DFFF: Fixed 8 KiB PRG-ROM bank #6
  * CPU $E000-$FFFF: Fixed 8 KiB PRG-ROM bank #7
  * PPU $0000-$1FFF: Switchable 8 KiB CHR-ROM bank
  * Nametable mirroring: hard-wired (to vertical)



# Bankswitching Reads

The 8 KiB PRG-ROM bank at $6000, and the 8 KiB CHR-ROM bank, are both switched to the same bank number whenever certain addresses are read from. 
    
    
    A~FEDC BA98 7654 3210
      -------------------
      .??? ???? ??DC BA1.
                  ++-++--- PRG/CHR A16..13
    

The exact address mask is not known; the following addresses have been verified to trigger a bankswitch, and are used by the game: 

  * CAB6-CAD6
  * EBE2, EE32
  * FFFC



# ASIC connection

See: [KS-201 pinout](KS_201_pinout.xhtml "KS-201 pinout")

Categories: [FDS conversion mappers](Category_FDS_conversion_mappers.xhtml), [Mappers triggering on reads](Category_Mappers_triggering_on_reads.xhtml)
