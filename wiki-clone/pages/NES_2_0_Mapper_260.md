# NES 2.0 Mapper 260

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_260) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_260)

NES 2.0 Mapper 260 is used for HP10xx/HP20xx multicarts. It is a predecessor to [FK23C](INES_Mapper_176.xhtml "INES Mapper 176"), both in its register layout, and the fact that several multicarts exist in HPxx and FK23C versions. Its UNIF board names are **BMC-HPxx** and **BMC-HP2018-A**. 

## Contents

  * 1 Registers
    * 1.1 DIP Switch ($5000, read)
    * 1.2 Mode Register ($5000, write)
    * 1.3 PRG Base Register ($5001, write)
    * 1.4 CHR Base Register ($5002, write)
    * 1.5 CNROM/Nametable Arrangement Latch ($8000-$FFFF, write)
    * 1.6 MMC3-compatible registers ($8000-$FFFF, write)
  * 2 Notes



# Registers

## DIP Switch ($5000, read)
    
    
    Mask: Unknown
    
    7654 3210
    ---------
    .... ..DD
           ++- DIP Switch Setting
    
    

## Mode Register ($5000, write)
    
    
    Mask: $F003
    
    7654 3210
    ---------
    L... .MMM
    |     +++- Select banking mode
    |          0: MMC3: 256 KiB PRG, 256 KiB CHR
    |          1: MMC3: 256 KiB PRG, 128 KiB CHR
    |          2: MMC3: 128 KiB PRG, 256 KiB CHR
    |          3: MMC3: 128 KiB PRG, 128 KiB CHR
    |          4: NROM-128: 16 KiB PRG (mirrored at $8000 and $C000), 8 KiB CHR
    |          5: NROM-256: 32 KiB PRG, 8 KiB CHR
    |          6: CNROM: 32 KiB PRG, 16 KiB CHR
    |          7: CNROM: 32 KiB PRG, 32 KiB CHR
    +--------- 1= Lock, do not respond to further writes in the $5xxx range
    
    

In MMC3 modes, the final PRG/CHR bank number is the result of masking the MMC3 bank register content according to the specified size (128 or 256 KiB) and OR'ing with the opposite-masked content of the PRG ($5001) and CHR ($5002) Base registers. In the CNROM modes, the inner bank comes from the CNROM Latch (one bit only in 16 KiB CHR mode, two bits in 32 KiB CHR mode) OR'ed with the opposite-masked content of the CHR ($5002) Base register. 

The nametable arrangement/mirroring is determined by MMC3 register A000 only in modes 0-3. In modes 4-7, it is determined by the latch's M bit.. 

## PRG Base Register ($5001, write)
    
    
    Mask: $F003
    
    7654 3210
    ---------
    ..PP PPPP
      ++-++++- Select 16 KiB PRG Base
      
    

## CHR Base Register ($5002, write)
    
    
    Mask: $F003
    
    7654 3210
    ---------
    .PPP PPPP
     +++-++++- Select 8 KiB CHR Base
     
    

## CNROM/Nametable Arrangement Latch ($8000-$FFFF, write)
    
    
    Mask: $8000
    
    7654 3210
    ---------
    .... .MLL
          |++- Select 8 KiB Inner CHR Bank in CNROM modes
          +--- Nametable arrangement
               0: Horizontal arrangement/Vertical mirroring
               1: Vertical arrangement/Horizontal mirroring
    

## MMC3-compatible registers ($8000-$FFFF, write)
    
    
    Mask: $E001
    
    $8000, $8001, $A000, $A001, $C000, $C001, $E000, $E001: As normal [MMC3](MMC3.xhtml "MMC3").
    

# Notes

  * The description of CNROM mode is based on the FCEUX source code. None of the available ROM images actually use it; instead, the games on those multicarts that originally were CNROM have all been modified to directly modify the CHR Base register ($5002).
  * WRAM at $6000-$7FFF is supported.
  * The KY6009 6-in-1 multicart menu times its music by polling $2002 bit 7 but does not take the [race condition](NMI.xhtml#Race_condition "NMI") into account. As a result, its music is audibly slowed down irregularly when played on an original NES/Famicom console.



Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
