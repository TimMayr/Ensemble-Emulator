# NES 2.0 Mapper 324

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_324) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_324)

NES 2.0 Mapper 324 is used for Farid's homebrew 8-in-1 UNROM multicart board. It consists of an UNROM latch with an additional outer bank register. Mirroring is hard-wired. Its UNIF board name is **FARID_UNROM_8-IN-1**. 

## Bank Register ($8000-$FFFF, write)
    
    
    Mask: $8000
    
    D~7654 3210  **BUS CONFLICTS**
      ---------
      Cooo kIII
      |||| ||||
      |||| |+++- Standard UNROM inner bank (select 16K at $8000-$BFFF)
      |||| +---- 1=Lock Outer Bank
      |+++------ Select 128 KiB outer PRG-ROM bank
      +--------- Clock Generator ("oook" bits latched only when this changes from 0 to 1 if k had been 0)
    

The middle four bits ($78, lowercase) are one 74'161; their contents are explicitly zeroed on power-up or reset. The outer four bits ($87, uppercase) are the other 74'161; their contents are NOT. 

## UNROM latch ($8000-$FFFF)

See [INES Mapper 002](UxROM.xhtml "INES Mapper 002"). 

## See also

  * [NesDev discussion](https://forums.nesdev.org/viewtopic.php?f=9&t=11099)
  * [Mapper 72](INES_Mapper_072.xhtml "INES Mapper 072") uses a similar "one latch's output clocks another latch" structure.



Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
