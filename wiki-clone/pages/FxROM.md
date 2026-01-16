# FxROM

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/FxROM) | View [other pages](Special_AllPages.xhtml#FxROM)

The generic designation **FxROM** refers to cartridge boards made by Nintendo that use the [Nintendo MMC4](MMC4.xhtml "MMC4") mapper. 

## Contents

  * 1 Board Types
  * 2 Overview
  * 3 Solder Pad config
    * 3.1 Battery backup
  * 4 Hardware
  * 5 See also



## Board Types

The following FxROM boards are known to exist: 

Board | PRG ROM | PRG RAM | CHR   
---|---|---|---  
FJROM | 128 / 256 KB | 8 KB | 16 / 32 / 64 KB ROM   
FKROM | 128 / 256 KB | 8 KB | 128 KB ROM   
  
## Overview

  * PRG ROM size: up to 256 KB
  * PRG ROM bank size: 16 KB
  * PRG RAM: 8 KB plus optional battery
  * CHR capacity: up to 128 KB ROM
  * CHR bank size: 4 KB
  * Nametable [mirroring](Mirroring.xhtml "Mirroring"): Controlled by mapper
  * Subject to [bus conflicts](Bus_conflict.xhtml "Bus conflict"): No



## Solder Pad config

### Battery backup

  * PRG RAM with data retention : 'SL' disconnected, D1, D2, R1, R2, R3 present.
  * PRG RAM without data retention : 'SL' connected, D1, D2, R1, R2, R3 not present.



On the FKROM board, battery backup appears to be compulsory unless there is a hidden solder pad below the MM1026 chip. 

## Hardware

The FKROM board uses a [Mitsumi MM1026](http://www.mitsumi.co.jp/latest/Catalog/ic/common/stop/MM1026_1245_1080_1134_e.pdf) chip dedicated to safe battery backup. It's the only Nintendo board to use a specific chip dedicated to safe battery backup. 

## See also

  * [Nintendo MMC4](MMC4.xhtml "MMC4")



Categories: [Nintendo licensed mappers](Category_Nintendo_licensed_mappers.xhtml)
