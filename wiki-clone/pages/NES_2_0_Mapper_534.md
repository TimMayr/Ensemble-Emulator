# NES 2.0 Mapper 534

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_534) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_534)

iNES/NES 2.0 Mappers **126** , **422** and **534** denote circuit boards using the [MMC3](MMC3.xhtml "MMC3")-based **TEC9719** , **ING003C** and **ING-022** ASICs. Four additional outer bank registers are provided at $6000-$7FFF for multicart use, overlaying WRAM. They can only be written to if the WRAM bit is enabled in the MMC3 ($A001=$80). Clearing the WRAM bit does _not_ disable WRAM itself. 

According to its datasheet, the ASIC can address up to 4 MiB of PRG ROM, while CHR ROM/RAM capacity is still limited to 256 KiB. Some cartridges mount more CHR ROM than that, repurposing higher PRG address lines as higher CHR address lines for that purpose. Mapper **126** differs from mappers **422** and **534** in which repurposed higher PRG address lines are used for which higher CHR address lines. This implies that in the absence of more than 256 KiB of CHR ROM or when using CHR RAM, mappers 126 and 422 are identical. 

Mapper **534** differs from mappers **126** and **422** in the implementation of the MMC3's scanline counter. Mappers 126 and 422 implement it normally, while mapper **534** takes the _inverted_ (XOR $FF) $C000 value (**ING-022** behavior). 

A few circuit boards mount two PRG ROM chips and select between them outside of the normal addressing scheme. Two NES 2.0 Submappers denote this non-contiguous address space: 

  * **Submapper 1** uses PRG A21 as a chip select between two 1 MiB chips.
  * **Submapper 2** uses $6001 bit 2 as a chip select between two 1 MiB chips.



Two UNIF board names have been assigned that resemble mapper 422: **BMC-BS-400R** and **BMC-BS-4040R**. 

## Contents

  * 1 Outer Bank/PRG Mask Register ($6000, write)
  * 2 Miscellaneous Register ($6001, write)
  * 3 8 KiB CHR Bank Register ($6002, write)
  * 4 Mode Register ($6003, write)
  * 5 Example Cartridges
  * 6 See also



## Outer Bank/PRG Mask Register ($6000, write)
    
    
    Mask: $E003, set to $00 on reset
    
    D~7654 3210
      ---------
      XYbB CPPp
      |||| |||+- PRG A17 if Y=1
      |||| |++-- PRG A19..A18
      |||| |+--- CHR A20
      ||++-|---- **Submapper 0** : PRG A21..A20
      ||+|-|---- **Submapper 1** : PRG A20
      ||||-+---- CHR A17 if X=1
      ||++------ CHR A19,A18 (mappers 422/534), CHR A18,A19 (mapper 126)
      |+-------- PRG A17 mode
      |           0: PRG A17=MMC3 PRG A17 (256 KiB inner PRG bank)
      |           1: PRG A17=p (128 KiB inner PRG bank)
      +--------- CHR A17 mode
                  0: CHR A17=MMC3 CHR A17 (256 KiB inner CHR bank)
                  1: CHR A17=C (128 KiB inner CHR bank)
    

Bit 5 (b) is _inverted_ before being output to the respective ASIC pin. Because the register is set to $00 on reset, this means that in the case of 4 MiB of PRG ROM, the cartridge boots in the second 2 MiB half. Older dumps do not take this inversion into account. 

## Miscellaneous Register ($6001, write)
    
    
    Mask: $E003, set to $00 on reset
    
    D~7654 3210
      ---------
      .... .PEM
            ||+- 0: PRG A0=CPU A0
            ||   1: PRG A0=SL0 input (used for menu selection)
            |+-- 0: MMC3 can only use V and H mirroring via $A000.0
            |    1: MMC3 can also select single-screen mirroring via $A000.1
            +--- **Submapper 2** : Select PRG chip 0 or 1
    

## 8 KiB CHR Bank Register ($6002, write)
    
    
    Mask: $E003, set to $00 on reset
    
    D~7654 3210
      ---------
      LGFE DCBA
      |||| ++++- CHR A16..A13 in 8 KiB CHR mode
      |||+------ Lock B bit
      ||+------- Lock C bit
      |+-------- Lock D bit
      +--------- Lock GFE bits
    

## Mode Register ($6003, write)
    
    
    Mask: $E003, set to $00 on reset
    
    D~7654 3210
      ---------
      L.M8 DCBA
      | || ++++- PRG Banking Mode (see below)
      | |+------ CHR Banking Mode
      | |         0: Use MMC3 CHR banks
      | |         1: Use 8 KiB CHR banks using register $6002
      | +------- Nametable Mirroring Mode
      |           0: MMC3, use register $A000
      |           1: ANROM, use register $8000.6 bit 4
      +--------- Lock registers $6000,$6001,$6003
    

The DCBA bits determine ... 

  1. what is being fed to the MMC3 clone's CPU A13 and A14 inputs when $8000-$FFFF is being written to, in order to replicate the UNROM/ANROM latch being reachable across the entire $8000-$FFFF address range;
  2. what is being fed to the MMC3 clone's CPU A13 and A14 inputs when $8000-$FFFF is being read from, in order to disable the MMC3's fixed banks in NROM and ANROM modes, or to obtain one switchable 16 KiB instead of two switchable 8 KiB banks in UNROM mode;
  3. how the MMC3 clone's PRG A17..A13 outputs are shifted to switch 16 or 32 KiB banks, instead of 8 KiB banks, in UNROM/ANROM modes;
  4. how the MMC3 clone's PRG A14..A13 outputs are substituted to create UNROM, ANROM, NROM-128 and NROM-256 modes.



In PRG Banking Modes $8-$F (bit D=1), CPU A0 is forced to 1 during writes to $8000-$FFFF, meaning that only the odd-numbered MMC3 registers are available. Furthermore, in modes D and F (bits D,C,A=1), CPU A13 and CPU A14 are forced to 0 during writes to $8000-$FFFF, meaning that any write actually goes to $8001, which after having set $8000 to 6 before, makes MMC3 register 6 serve as an UNROM/ANROM latch. 

This results in the following modes: 
    
    
     PRG    PRG Bit Source                      Address
     Mode#  A17    A16    A15    A14    A13     Range       Comment
    --------------------------------------------------------------------
      0     R6.4   R6.3   R6.2   R6.1   R6.0    8000-9FFF   Regular MMC3
            R7.4   R7.3   R7.2   R7.1   R7.0    A000-BFFF
            =1     =1     =1     =1     CPUA13  C000-FFFF
    
      1,2   R6.4   R6.3   R6.2   R6.1   CPUA13  8000-FFFF   NROM-128
    
      3     R6.4   R6.3   R6.2   CPUA14 CPUA13  8000-FFFF   NROM-256 
    --------------------------------------------------------------------
      C     R6.3   R6.2   R6.1   R6.1   R6.0    8000-9FFF   Nonsense
            R7.3   R7.2   R7.1   R7.1   R7.0    A000-BFFF
            =1     =1     =1     =1     CPUA13  C000-FFFF
    
      D     R6.3   R6.2   R6.1   R6.0   CPUA13  8000-BFFF   UNROM
            =1     =1     =1     =1     CPUA13  C000-FFFF
    
      E     R6.2   R6.1   R6.0   R6.1   R6.0    8000-9FFF   Nonsense
            R7.2   R7.1   R7.0   R7.1   R7.0    A000-BFFF
            =1     =1     =1     =1     CPUA13  C000-FFFF
    
      F     R6.2   R6.1   R6.0   CPUA14 CPUA13  8000-FFFF   ANROM
    --------------------------------------------------------------------
      4-7   Same as 0-3
      8-B   Same as C-F, but latch reachable only at $8000-$9FFF
    ----------------------------------------------------------------
    

Note that PRG A17 is still subject to being substituted with register $6000 bit 0 if register $6000 bit 6=1. 

## Example Cartridges

  * Mapper 126: 
    * _Power Joy Classic TV Game 84-in-1 (PJ-008)_
    * _Gamezone 118-in-1 (AT-207)_
  * Mapper 422: 
    * _(AB-099) 7-in-1_
    * _(AB-199) AB 一簇 10-in-1_
    * _(AB-239) 最新力作 Well 4-in-1_
    * _(BL-4029) Well 4-in-1_
    * _(GD-105) 9898-in-1_
    * _(GD-106) 18-in-1_
    * _(TL-8248) 6-in-1 Superstar Edition_
    * _(V-76005) 76-in-1_
    * _1998 4000000-in-1_ (BS-400 PCB)
    * _3000000-in-1_ (BS-300 PCB)
    * _700000-in-1_ (BS-400 PCB)
    * _Double Dragon 530-in-1_ (BS-4040 PCB)
  * Mapper 534: 
    * _Atari Flashback Mini 7800_
    * _TV Play Power - Intellivision 25-in-1_
    * _2-in-1 数独/五子棋_ (NJ064) by Shenzhen Nanjing
    * _18-in-1 Educational Computer_ by Zhuhai S.E.Z. Liming Electronic Co. Ltd.
    * _8-in-1_ (kk3311-kk3315) by Waixing
    * _101-in-1_ (a variant of which uses [INES Mapper 045](INES_Mapper_045.xhtml "INES Mapper 045")).



## See also

  * [Discussion and original data sheet](https://forums.nesdev.org/viewtopic.php?t=24629)



Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
