# Talk:J.Y. Company ASIC

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AJ.Y._Company_ASIC) | View [other pages](Special_AllPages.xhtml#Talk_J_Y__Company_ASIC)

## PPU read IRQ mode

I don't see why the PPU read mode of the interval timer should be labeled "wtf". Wouldn't an IRQ mode based on PPU reads just clock 170 times per scanline? It'd be sort of like CPU mode, except not dependent on the [CPU to PPU clock ratio](Cycle_reference_chart.xhtml "Clock rate"). And it'd be unaffected by "borrowing" 8x16 sprites from the other pattern table, which trips up PA12-rise PITs such as MMC3's. With the /256 prescaler, it would appear fairly straightforward to work with: as the last thing in vblank, load 170/256 (very close to two-thirds) times the number of scanlines to wait. Apart from "CPU writes" and "funky mode", I see nothing particularly strange about 90's PIT. Or is there something harsh that I'm missing? --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 23:04, 10 May 2014 (MDT) 

    When Disch wrote these, he had Strong Opinions about anything that wasn't MMC3-shaped. See also his original writeup of Bandai LZ93D50 EEPROM traffic[[1]](https://wiki.nesdev.org/w/index.php/Bandai_FCG_board?oldid=3070). —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 00:19, 11 May 2014 (MDT)

## Multiplier

I have been playing with a Donkey Kong Country 4 Cartridge "mapper 211", and regarding the multiplier, the result is not changed with writes to $5800. The result is updated only after $5801 is written to. — [Holodnak](https://www.nesdev.org/w/index.php?title=User:Holodnak&action=edit&redlink=1 "User:Holodnak \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Holodnak&action=edit&redlink=1 "User talk:Holodnak \(page does not exist\)")) 00:37 3 September 2016 (MDT) 

    Updated. Are you willing to open the cartridge? If so, is the mapper IC packaged? Is there any non-generic information on the PCB? — [Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 11:25, 3 September 2016 (MDT) 

    I peeked. Black epoxy blobs. :(

  


## IRQ Source

Goal 2 uses CPU Cycle as IRQ source (if PPU A12 will be forced as clock source, country flags at startup and statusbar won't be displayed correctly) <https://obrazki.elektroda.pl/1866366500_1524519957.png> [Krzysiobal](User_Krzysiobal.xhtml "User:Krzysiobal") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Krzysiobal&action=edit&redlink=1 "User talk:Krzysiobal \(page does not exist\)")) 15:46, 23 April 2018 (MDT) 
