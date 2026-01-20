# Talk:Non-power-of-two ROM size

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ANon-power-of-two_ROM_size) | View [other pages](Special_AllPages.xhtml#Talk_Non_power_of_two_ROM_size)

## Action 52 PCB

_The most notable exception on NES is Action 52, which has four footprints for 512Kx8 (4 Mbit) PRG ROMs, of which three are populated totaling 12 Mbit (1.5 MiB). '_ Which PCB has footprint for 4 PRG ROMs? [Krzysiobal](User_Krzysiobal.xhtml "User:Krzysiobal") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Krzysiobal&action=edit&redlink=1 "User talk:Krzysiobal \(page does not exist\)")) 09:45, 24 June 2021 (MDT) 

    I'll have to review my sources and edit if my recollection was wrong. If the production PCB has three footprints, is there a decoder output line that goes NC? --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 09:03, 27 June 2021 (MDT) 

    PAL16L8 is responsible for enabling ROMs in Action52. I don't have the PCB to confirm operation, but if wiki says there is `open bus` when bank 2 is selected, then it means the logic formula inside PAL does not enable any ROM when this bank is selected. [Krzysiobal](User_Krzysiobal.xhtml "User:Krzysiobal") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Krzysiobal&action=edit&redlink=1 "User talk:Krzysiobal \(page does not exist\)")) 10:43, 27 June 2021 (MDT)
