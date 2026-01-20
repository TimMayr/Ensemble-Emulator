# MMC3 + NROM multicart

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/MMC3_%2B_NROM_multicart) | View [other pages](Special_AllPages.xhtml#MMC3___NROM_multicart)

[![MMC3+NROM farid components.jpg](../wiki-images/MMC3%2BNROM_farid_components.jpg)](File_MMC3_NROM_farid_components_jpg.xhtml)

[](File_MMC3_NROM_farid_components_jpg.xhtml "Enlarge")

[![MMC3+NROM farid top.jpg](../wiki-images/MMC3%2BNROM_farid_top.jpg)](File_MMC3_NROM_farid_top_jpg.xhtml)

[](File_MMC3_NROM_farid_top_jpg.xhtml "Enlarge")

[![MMC3+NROM farid bottom.jpg](../wiki-images/MMC3%2BNROM_farid_bottom.jpg)](File_MMC3_NROM_farid_bottom_jpg.xhtml)

[](File_MMC3_NROM_farid_bottom_jpg.xhtml "Enlarge")

[![MMC3+NROM farid schematics.png](../wiki-images/MMC3%2BNROM_farid_schematics.png)](File_MMC3_NROM_farid_schematics_png.xhtml)

[](File_MMC3_NROM_farid_schematics_png.xhtml "Enlarge")

## Contents

  * 1 Overview
  * 2 PRG banking
  * 3 CHR banking
  * 4 Mirroring
  * 5 Notes



## Overview

  * PRG: 512kB ROM1 + 32kB ROM2
  * CHR: 512kB ROM
  * Mirroring: H or V (selected by software)
  * Bus conflicts: no



All MMC3's PRG & CHR registers are available at $8000-$ffff for the whole time. There is no WRAM at $6000-$7fff but additional register (which can be protected from writes using MMC3's $a001 just like it protects WRAM). 

After power up & soft/hard reset, this is cleared and ROM2 is selected (for $8000-$ffff). 
    
    
    [011. ...R .wMG GPPP] REG $6000-$7fff (address bits are latched)
             |  ||| ||||
             |  ||| |+++- selects inner 16KB PRG when in NROM mode
             |  ||+-+---- selects outer 128KB PRG & 128KB CHR (in any mode)
             |  |+------- 0: NROM mode, 1: MMC3 mode
             |  +-------- 1: protect whole $6000-$7fff register from further changes
             +----------- 0: selects ROM2, 1: selects ROM1 and protects this single bit from further changes
    

## PRG banking
    
    
    condition         | $8000-$9fff | $a000-$bfff | $c000-$dfff | $e000-$ffff | notes
    ------------------+-------------+-------------+-------------+-------------+--------
    R=0               |  one 32 KB PRG from ROM2 (mapped to first 32 KB)      | NROM32 
    ------------------+-------------+-------------+-------------+-------------+--------
    R=1 & M=1         |   four CPU banks just as in MMC3 (from outer bank GG) | MMC3
    ------------------+-------------+-------------+-------------+-------------+--------
    R=1 & M=0 & GG=0  |        GGPPPP             |         GGPPPP            | NROM16
    ------------------+-------------+-------------+-------------+-------------+--------
    R=1 & M=0 & GG!=0 |         GGPPP0            |         GGPPP1            | NROM32
    ------------------+-------------+-------------+-------------+-------------+--------
    

## CHR banking

CHR banking works in the same manner like in MMC3. 

## Mirroring

Mirroring works in the same manner like in MMC3. 

## Notes

More info about this multicart can be found here: [http://forums.nesdev.org/viewtopic.php?t=7926&start=15](http://forums.nesdev.org/viewtopic.php?t=7926&start=15)

Categories: [Mappers](Category_Mappers.xhtml)
