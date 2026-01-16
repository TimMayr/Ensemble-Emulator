# Game Genie

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Game_Genie) | View [other pages](Special_AllPages.xhtml#Game_Genie)

[![](../wiki-images/Game-Genie-NES.jpg)](File_Game_Genie_NES_jpg.xhtml)

[](File_Game_Genie_NES_jpg.xhtml "Enlarge")

Game Genie

The **Game Genie** is a enhancement cart for the NES designed by [Camerica](https://en.wikipedia.org/wiki/Camerica "wikipedia:Camerica") and distributed by [Galoob](https://en.wikipedia.org/wiki/Galoob "wikipedia:Galoob") and Camerica. It functions as a pass-thru, with a [72-pin cartridge connector](Cartridge_connector.xhtml#Pinout_of_72-pin_NES_consoles_and_cartridges "Cartridge connector") connecting it to the NES, and a 72-pin cartridge slot for a game to be inserted into. When plugged in between a game and the NES and turned on, it provides a simple interface to enter up to three cheat codes, which then modify the behavior of the game. First revision were build using [ASIC blob chip](NES_Game_Genie_IC_pinout.xhtml "NES Game Genie IC pinout") and 4 kB ROM, the latter one has both chips integrated into single epoxy blob. There even exist a console (_Geniecom Enhance Console video Game_) that has the Game Genie ASIC DIP CHIP (_GENIECOM-V1 BIC_) built it. 

The Game Genie is not assigned a [mapper](Mapper.xhtml "Mapper") number. 

[![](../wiki-images/Game_Genie_Revision_1.5A_%28PCB_Front%29.jpg)](File_Game_Genie_Revision_1_5A__PCB_Front__jpg.xhtml)

[](File_Game_Genie_Revision_1_5A__PCB_Front__jpg.xhtml "Enlarge")

PCB Front

[![](../wiki-images/Game_Genie_Revision_1.5A_%28PCB_Back%29.jpg)](File_Game_Genie_Revision_1_5A__PCB_Back__jpg.xhtml)

[](File_Game_Genie_Revision_1_5A__PCB_Back__jpg.xhtml "Enlarge")

PCB Back

[![](../wiki-images/Game_genie_blob_top.jpg)](File_Game_genie_blob_top_jpg.xhtml)

[](File_Game_genie_blob_top_jpg.xhtml "Enlarge")

Blob Top

[![](../wiki-images/Game_genie_blob_bottom.jpg)](File_Game_genie_blob_bottom_jpg.xhtml)

[](File_Game_genie_blob_bottom_jpg.xhtml "Enlarge")

Blob Bottom

[![](../wiki-images/Nesgenieformat.png)](File_Nesgenieformat_png.xhtml)

[](File_Nesgenieformat_png.xhtml "Enlarge")

A diagram that explains the format of the codes.

## Contents

  * 1 Technical
  * 2 Software version
  * 3 Registers
    * 3.1 Master Control ($8000)
    * 3.2 Address High ($8001, $8005, $8009)
    * 3.3 Address Low ($8002, $8006, $800A)
    * 3.4 Compare ($8003, $8007, $800B)
    * 3.5 Replace ($8004, $8008, $800C)
    * 3.6 Unknown ($FFF0, $FFF1)
  * 4 Pattern Tables
  * 5 Bugs
  * 6 References
  * 7 External links



## Technical

The Game Genie works by intercepting CPU reads and replacing the game cart's response with its own response. It can intercept any three addresses in CPU $8000…$FFFF and respond with a single replacement for each. To make the tool more compatible with [bank-switching](Glossary.xhtml#B "Glossary"), each of the three codes has an optional compare value which can be used to only replace the byte if the original byte matches the compare, hopefully limiting the cheat to functioning on the desired bank. 

When first booted, the Game Genie presents its own 4-KiB [PRG ROM](Glossary.xhtml#P "Glossary") (mapped at $c000-$ffff and mirrored 4 times; $8000-$bfff is open bus) and a series of simple gates masquerading as a [CHR ROM](Glossary.xhtml#C "Glossary"). The included PRG ROM runs code to show a simple code entry user interface. When the user presses Start, the cheat values are written to memory-mapped registers, and then another register is written which switches the Game Genie into game mode, where the attached game cart's CHR and PRG is passed through, save whatever code replacements were defined. The Game Genie remains in game mode until power-cycled, and will respond to no further writes. 

## Software version

The only version known to exists is 1.5A (CRC32 = 1A3A22A1), present both in DIP chip version (with ROM chip marked as **GENIE V1.5A**) and single epoxy blob one). 

## Registers

### Master Control ($8000)
    
    
    7  bit  0
    ---- ----
    .DDD CCCE
     ||| ||||
     ||| |||+- Write 1 to switch into game mode
     ||| +++-- Compare enable for each of the three codes
     +++------ Disable each of the three codes
    

Bit 1 and 4 correspond to the code at $8001…$8004. 

Game Genie writes first a value with bit 0 set and then writes 0x00 to this register. Because after the first write, the Game Genie logic switches into game mode, any further writes to range $8000–$ffff will cause the slave cartridge /ROMSEL to become low for that write cycle. As a result, the second write will be seen and interpreted by the hardware inside slave game cartridge. The reason for this write is unknown, maybe it initializes the bank select register for MMC3 games? 

### Address High ($8001, $8005, $8009)
    
    
    7  bit  0
    ---- ----
    .AAA AAAA
     ||| ||||
     +++-++++- Bits 8:14 of address for this cheat (bit 15 fixed to 1)
    

### Address Low ($8002, $8006, $800A)
    
    
    7  bit  0
    ---- ----
    AAAA AAAA
    |||| ||||
    ++++-++++- Bits 0:7 of address for this cheat
    

### Compare ($8003, $8007, $800B)
    
    
    7  bit  0
    ---- ----
    CCCC CCCC
    |||| ||||
    ++++-++++- Compare value for this cheat (write 0 if unused?)
    

### Replace ($8004, $8008, $800C)
    
    
    7  bit  0
    ---- ----
    RRRR RRRR
    |||| ||||
    ++++-++++- Replacement value for this cheat
    

### Unknown ($FFF0, $FFF1)

The Game Genie rom writes 0 to $FFF0, $FFF1, $FFF0 in that sequence. 

## Pattern Tables

When game mode is inactive, the Game Genie generates [PPU pattern tables](PPU_pattern_tables.xhtml "PPU pattern tables") through PPU $0000…$1FFF by the following method: 

  * When PPU `A2` = 1: 
    * PPU `A4` → PPU `D4`…`D7`
    * PPU `A5` → PPU `D0`…`D3`
  * When PPU `A2` = 0: 
    * PPU `A6` → PPU `D4`…`D7`
    * PPU `A7` → PPU `D0`…`D3`



This creates 16 distinct objects that are used to build the menu graphics: 

[![Gg sprites.PNG](../wiki-images/Gg_sprites.PNG)](File_Gg_sprites_PNG.xhtml)

## Bugs

Because of how the hardware is designed, there are some bugs or limitations of this device: 

  * When a cartridge has something mapped at $4020–$7FFF (WRAM, PRG ROM) and a code for region $C020–$FFFF is added, the Game Genie will hold the slave cartridge's /ROMSEL at 1 when reading from that location. But then, the cartridge logic will see this read cycle as something below $8000, enabling the chip that is mapped here, causing bus conflict at this location and resulting in invalid data being returned to the CPU.[1] Some codes rely on these bus conflicts. For example, in The Legend of Zelda, the code XYYYYL causes the sword beam to leave bait behind, but crashes instead if bus conflicts are not emulated.
  * Cartridges that rely only on PPU /A13 when decoding CHR-ROM (like MMC5) will not display the the Game Genie menu properly, as the Game Genie ignores this line, causing bus conflict.[1]
  * According to the Game Genie patent, the process of determining if a code with comparison should be enabled is asynchronous. This makes it impossible to apply multiple codes with the same address but different replace/compare values. The Game Genie allows entering such codes, but when it comes to sending them to the ASIC chip, only the first such code will be enabled.



## References

  1. ↑ 1.0 1.1 [Forums: Game Genie - does it work for $e000-$ffff + WRAM?](https://forums.nesdev.org/viewtopic.php?p=230447#p230447)



## External links

  * [nesgg.txt](http://nesdev.org/nesgg.txt) – _NES Game Genie Code Format DOC v0.71_ by Benzene of Digital Emutations, 1997-07-10
  * [Patent US5112051A – _Interfacing device for a computer games system_](http://patents.google.com/patent/US5112051A/en)
  * [NES Game Genie ROM disassembly](https://www.kevinselwyn.com/posts/game-genie-disassembly/) by Kevin Selwyn ([GitHub page](http://github.com/kevinselwyn/game-genie-disassembly))
  * [NES Game Genie ROM disassembly](http://github.com/qalle2/nes-gg-disassembly) by qalle


