# NES 2.0 Mapper 444

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_444) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_444)

[![](../wiki-images/NC7000M.jpg)](File_NC7000M_jpg.xhtml)

[](File_NC7000M_jpg.xhtml "Enlarge")

Front view

**NES 2.0 Mapper 444** denotes the **NC7000M** multicart circuit board. Its UNIF board name is **BMC-BS-110** , which is the name of a different PCB whose name was mixed-up. 

The top-right solder pads are relevant for bankswitching and are denoted by Submapper bits. A clear bit represents the _upper_ pad connection: 

  * Submapper bit 0 (top-rightmost pad): CHR-ROM A17 connected to $6000.0 (clear) or to the MMC3 CHR A17 output (set). Selects whether the MMC3 can address 128 or 256 KiB of CHR data.
  * Submapper bit 1 (the top-second-rightmost pad): CHR-ROM A18 connected to $6000.1 (clear) or to $6000.4 (set). Since PRG A18 is also determined by $6000.1, it thus selects whether CHR A18 can be determined separately or not.



Known cartridges: 

  * _7000-in-1_ (CA-004): both bits set (submapper 3)
  * _Super 8000-in-1_ : both bits clear (submapper 0)



With both pads in the position represented by submapper 0, NC7000M's definition has a subset in the [**K-3043** PCB](https://forums.nesdev.org/viewtopic.php?p=277283). 

## Outer Bank and Mode Register ($6000-$7FFF, write)
    
    
    A~[011. .... ...C MMBA]
                    | ||++- PRG A18..17
                    | |||+- CHR A17 if submapper bit 0 clear
                    | ||+-- CHR A18 if submapper bit 1 clear
                    | ++--- Select PRG-ROM mode
                    |        0: MMC3 mode
                    |        1: NROM-256 mode (PRG A13..14=CPU A13..14, PRG A15..16=MMC3 A15..16)
                    |        2: Read solder pads' value in D0/D1 (leftmost pads in picture)
                    |        3: NROM-128 mode (PRG A13=CPU A13, PRG A14..16=MMC3 A14..16)
                    +------ CHR A18 if submapper bit 1 set
    

  * WRAM must be enabled in $A001.7 before writing to this register.
  * The inner PRG bank is restricted to 128 KiB.
  * NROM mode forces MMC3's CPU A13 and A14 inputs to GND to make MMC3 bank register 6 apply to the entire CPU $8000-$FFFF range.



## MMC3-compatible registers

Mask: $E001 

See [MMC3](MMC3.xhtml "MMC3"). 

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml), [NES 2.0 mappers with submappers](Category_NES_2_0_mappers_with_submappers.xhtml)
