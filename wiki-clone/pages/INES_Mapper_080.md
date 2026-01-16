# INES Mapper 080

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_080) | View [other pages](Special_AllPages.xhtml#INES_Mapper_080)

iNES Mapper 080 represents most boards using [Taito's X1-005 mapper IC](Taito_X1_005_pinout.xhtml "Taito X1-005 pinout"), which provides something somewhere between the [MMC6](MMC6.xhtml "MMC6") and the [Namcot 108](INES_Mapper_206.xhtml "INES Mapper 206") in sophistication. 

In comparison to to the Namcot 108, it: 

  * has the ability to bankswitch ROM from $C000-$DFFF
  * has mirroring control
  * has 128 extra bytes of RAM, optionally battery-backed



In comparison to the MMC6, it 

  * has the ability to bankswitch three different 8 KiB slices of PRG ROM simultaneously
  * does not have an IRQ
  * cannot swap which pattern table is 2×2KiB and which is 4×1KiB
  * has only 128 extra bytes of RAM



If the pinout is correct, CPU A7 is ignored and the control registers should also be present at $7E7x. 

See also: [Taito X1-017](Taito_X1_017.xhtml "INES Mapper 082"), [Taito X1-005 with alternate mirroring control](INES_Mapper_207.xhtml "INES Mapper 207"), [BootGod's forum post](http://forums.nesdev.org/viewtopic.php?p=30165)
    
    
     ========================
     =  Mapper 080          =
     ========================
     
     
     Example Games:
     --------------------------
     Kyonshiizu 2
     Minelvaton Saga
     Taito Grand Prix - Eikou heno License
     
     
     Notes:
     ---------------------------
     Regs appear at $7EFx, I'm unsure whether or not PRG-RAM can exist at $6000-7EFF
     
     Fudou Myouou Den is often marked to use this mapper -- however it uses [mapper 207](INES_Mapper_207.xhtml "INES Mapper 207").
     
     
     Registers:
     ---------------------------
     
       $7EF0-7EF5:  CHR Regs
     
       $7EF6:       [.... ...M]  Mirroring
         0 = Horz
         1 = Vert
    
       $7EF8,7EF9:  Internal RAM permission ($A3 enables reads/writes; any other value disables)
      
       $7EFA,7EFB:  PRG Reg 0 (8k @ $8000)
       $7EFC,7EFD:  PRG Reg 1 (8k @ $A000)
       $7EFE,7EFF:  PRG Reg 2 (8k @ $C000)
    
       $7F00-7FFF:  128 Bytes of RAM, mirrored once.
     
     
     CHR Setup:
     ---------------------------
     
            $0000   $0400   $0800   $0C00   $1000   $1400   $1800   $1C00 
          +---------------+---------------+-------+-------+-------+-------+
          |    <$7EF0>    |    <$7EF1>    | $7EF2 | $7EF3 | $7EF4 | $7EF5 |
          +---------------+---------------+-------+-------+-------+-------+
     
     PRG Setup:
     ---------------------------
     
           $8000   $A000   $C000   $E000  
         +-------+-------+-------+-------+
         | $7EFA | $7EFC | $7EFE | { -1} |
         +-------+-------+-------+-------+
    

Categories: [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
