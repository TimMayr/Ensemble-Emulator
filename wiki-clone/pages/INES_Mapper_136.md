# INES Mapper 136

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_136) | View [other pages](Special_AllPages.xhtml#INES_Mapper_136)

INES Mapper 136 is used to denote the Sachen 3011 board, used by four early releases from Joy Van/Sachen: 

  * _未来小子_ (Wèilái Xiǎozi, "Joy Van Kid", original Taiwan release of _Metal Fighter µ_ with Chinese title screen) (TC-008)
  * _蝶變_ (Diébiàn, "Incantation", original Taiwan release of _Galactic Crusader_ with Chinese title screen, often incorrectly rendered as Japanese "Chou Hen") (TC-009)
  * _四川麻將_ (Sìchuān Májiàng, "Mahjong Trap", original Taiwan release of _Shisen Mahjong - Seifuku Hen_) (TC-010)
  * _Mahjong Companion_ (alternative version, more common version use mappers [79](NINA_003_006.xhtml "INES Mapper 079") or [139](Sachen_8259.xhtml "INES Mapper 139") instead.



## Contents

  * 1 Banks
  * 2 Registers
  * 3 Similar Mappers
  * 4 See also



## Banks

  * CPU $8000-$FFFF: 32 KiB switchable PRG ROM bank (only used for _Mahjong Companion_)
  * PPU $0000-$1FFF: 8 KB switchable CHR ROM bank



## Registers

Mapper 136 uses a custom IC (real number "JV001", fake marking "GS63030-A") serving as a latch, adder and inverter. There are five registers: Input (6 bits), Output (6 bits), Register (6 bits), Mode (1 bit) and Invert (1 bit). 
    
    
    Mask: $E103
    Read $4100-$4103: [..RR RRRR]: Read Register. Bits 4-5 are inverted if Invert==1. Bits 6-7 are open bus.
    Write $4100: When Mode==0: Bits 0-5 of Register := Input, bits 0-3 being inverted if Invert==1.
                 When Mode==1: Bits 0-3 of Register incremented by one, bits 4-5 unaffected.
    Write $4101: Invert := Written value bit 0.
    Write $4102: Input := Written value bits 0-5.
    Write $4103: Mode := Written value bit 0.
    Write $8000-$FFFF: Output := Register; written value is ignored.
    

In Mapper 136, bits 0/2 of the 8 KiB CHR ROM bank number (CHR A13..A15) come from Output bits 0-2, and the 32 KiB PRG ROM bank number (PRG A15) comes from Output bit 4: 
    
    
    8 KiB CHR ROM bank number := (Output &7);
    32 KiB PRG ROM bank number := (Output >>4) &1;
    

Sachen's original **3011** circuit board only supported CHR ROM switching. The connection to switch 32 KiB PRG ROM was obtained by [soldering an additional wire/resistor](https://datomatic.no-intro.org/attachments%20-%20sources/Nintendo%20-%20Nintendo%20Entertainment%20System/20210330_144245_3138_pcb_back.jpg). 

Games will check the lower four or six bits of $4100 for the correct value after several increment and inversion operations as a copy-protection measure. 

## Similar Mappers

  * Mapper 136 is almost identical to [INES Mapper 132](INES_Mapper_132.xhtml "INES Mapper 132") with only 32 KiB PRG-ROM except in the value read from $4100 due to Mapper 136 having four (28-pin JV001 ASIC) versus Mapper 132 having three adder bits (24-pin 05-00002-010 ASIC).
  * [INES Mapper 147](INES_Mapper_147.xhtml "INES Mapper 147") expands CHR-ROM size to 128 KiB and additionally supports 128 KiB PRG-ROM, switched in 32 KiB amounts.
  * [INES Mapper 172](INES_Mapper_172.xhtml "INES Mapper 172") is similar to Mapper 136, connecting the six CPU bits in reverse order due to the JV001 ASIC being mounted upside down.



## See also

  * [PCB image](http://forums.nesdev.org/viewtopic.php?f=3&t=15961&start=30#p210702)


