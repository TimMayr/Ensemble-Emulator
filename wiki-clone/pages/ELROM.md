# ELROM

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/ELROM) | View [other pages](Special_AllPages.xhtml#ELROM)

**ELROM** (NES-ELROM and HVC-ELROM) is a common board within the [ExROM](ExROM.xhtml "ExROM") set. Like other ExROM boards, ELROM uses the [Nintendo MMC5](MMC5.xhtml "MMC5") ASIC. 

## Overview

  * PRG ROM size: 128, 256 or 512 KB (DIP-28/32 Nintendo pinout)
  * PRG ROM bank size: 8 KB, 16 KB, or 32 KB
  * PRG RAM: none
  * CHR capacity: 128, 256 or 512 KB ROM (DIP-32 Nintendo pinout)
  * CHR bank size: 1 KB, 2 KB, 4 KB, or 8 KB
  * Nametable [mirroring](Mirroring.xhtml "Mirroring"): Controlled by mapper
  * Subject to [bus conflicts](Bus_conflict.xhtml "Bus conflict"): No



## Solder Pad Config

  * MMC5 in 'CL mode' : 'CL3', 'CL4', 'CL5' and 'CL6' connected, 'SL3', 'SL4', 'SL5' and 'SL6' disconnected.
  * MMC5 in 'SL mode' : 'CL3', 'CL4', 'CL5' and 'CL6' disconnected, 'SL3', 'SL4', 'SL5' and 'SL6' connected.


  * Sound circuitry used : R1, R2, R4, C2, C3 present, R9 present (NES-ELROM only) or R3 and C4 present (HVC-ELROM only). An American or European NES must be modified to output cartridge audio, even if the audio circuitry is implemented.
  * Sound circuitry unused : R1, R2, R3, R4, R9, C2, C3 and C4 not present.



Note : Maybe there are more than two ways to put the MMC5 in 'CL mode' or 'SL mode' if some pads are set to CL and some other to SL. More testing on this should be done. 

## See also

  * [Nintendo MMC5](MMC5.xhtml "MMC5")


