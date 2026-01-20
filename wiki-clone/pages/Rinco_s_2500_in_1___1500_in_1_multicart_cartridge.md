# Rinco's 2500-in-1 / 1500-in-1 multicart cartridge

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Rinco%27s_2500-in-1_/_1500-in-1_multicart_cartridge) | View [other pages](Special_AllPages.xhtml#Rinco_s_2500_in_1___1500_in_1_multicart_cartridge)

This multicart cartridge was placed in Rinco Thompsonic's second slot cartridge and shipped with it. It consists of 1 MB mask ROM, 32 kB EEPROM, PAL16L8, latches and some gates. It allows to run the following mappers: 16 kB NROM, 32 kB NROM, 128kB UNROM. 

[![](../wiki-images/2500-in-1_top.jpg)](File_2500_in_1_top_jpg.xhtml)

[](File_2500_in_1_top_jpg.xhtml "Enlarge")

2500-in-1 top

[![](../wiki-images/2500-in-1_bottom.jpg)](File_2500_in_1_bottom_jpg.xhtml)

[](File_2500_in_1_bottom_jpg.xhtml "Enlarge")

2500-in-1 bottom

[![](../wiki-images/2500-in-1_title_screen.jpg)](File_2500_in_1_title_screen_jpg.xhtml)

[](File_2500_in_1_title_screen_jpg.xhtml "Enlarge")

2500-in-1 title screen

[![](../wiki-images/2500-in-1_schematic.png)](File_2500_in_1_schematic_png.xhtml)

[](File_2500_in_1_schematic_png.xhtml "Enlarge")

2500-in-1 schematic

To be exact, it consists of the following games: 

  * 1\. contra
  * 2\. tetris 2
  * 3\. tank
  * 4\. b-wings
  * 5\. super mario 1
  * 6\. 1942
  * 7\. ninja III
  * 8\. dig dug II
  * 9\. binary land
  * 10\. bomber man
  * 11\. ice climber
  * 12\. galaxian
  * 13\. road fighter
  * 14\. hogans alley
  * 15\. wild gunman
  * 16\. duck shoot
  * 17\. antarctic adventure
  * 18\. dig dug
  * 19\. urban champion
  * 20\. baloon fight
  * 21\. clu clu clan jss
  * 22\. sky destroyer
  * 23\. popeye
  * 24\. wrecking crew
  * 25\. raid on bay her
  * 26\. milk nuts
  * 27\. devil world
  * 28\. chack pop
  * 29\. macross
  * 30\. mario bros



## Contents

  * 1 CPU Banks
  * 2 PPU Banks
  * 3 Mirroring
  * 4 Subject to bus conflicts
  * 5 Registers
  * 6 Quirks



## CPU Banks

$8000-$BFFF: 16 kB PRG bank 

$C000-$FFFF: 16 kB PRG bank 

or 

$8000-$FFFF: 32 kB PRG bank 

If PRG bank = 0 or PRG bank = 7 (PRG-A16=1 and PRG-A15=1 and PRG-A14=1) then EEPROM is used, otherwise MaskRom is used. 

## PPU Banks

$0000-$1FFF: 8 kB CHR RAM (not banked), but in some modes it can be protected against writes. 

## Mirroring

H or V (set by software). 

## Subject to bus conflicts

Yes. 

## Registers

$8000-$FFFF: one global configuration register (bits of written address is latched, data is irrelevant) 
    
    
    15                0
    ---- ---- ---- ----
    1... ..pP wPPQ QRmM
           || |||| ||||
           || |||| |||+--- PRG size (0=16 kB, 1=32 kB)
           || |||| ||+---- mirroring (0=vertical, 1=horizontal)
           || |||+-++----- Inner PRG bank at $8000
           |+--++--------- 128 KiB Outer PRG bank
           |  +----------- UNROM mode (0=enabled) / CHR-RAM write protection (0=disabled, 1=enabled)
           +-------------- Inner PRG bank at $C000 in UNROM mode (note, single bit but used three times)
    
    

M,w \ PRG bank | $8000-$bfff | $c000-$ffff | resembles   
---|---|---|---  
M=0 and w=0 | PPPQQR | PPPppp | UNROM   
M=0 and w=1 | PPPQQR | PPPQQR | 16K NROM   
M=1 and w=0 | PPPQQ0 | PPPppp | (defective combination)   
M=1 and w=1 | PPPQQ0 | PPPQQ1 | 32K NROM   
  
## Quirks

  * In most cartridges where reset detection is needed, an R-C-D circuit is present and connected to any latches' clear inputs. As this cartridge uses current-based 74LS logic, only a capacitor and diode are needed.


  * The total PRG capacity is not 1 MiB + 32 KiB but only 1 MiB: the 32 KiB EPROM is a patch that replaces banks 0 and 7. It is believed that those banks contains menu data (list of games) and authors of the cartridge were unhappy with the original menu from mask ROM and they wanted to change it.


  * There are at least two variants of this cartridge with different markings on mask ROM and EPROM (other logic, including PAL is the same): 
    * 1500-in-1 (mask ROM: CT-121H, EPROM: 91150)
    * 2500-in-1 (mask ROM: CT-120H, EPROM: 91250)



Categories: [Mappers](Category_Mappers.xhtml)
