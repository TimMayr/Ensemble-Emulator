# Programming guide

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Programming_guide) | View [other pages](Special_AllPages.xhtml#Programming_guide)

This programming guide is the complement to the [NES reference guide](NES_reference_guide.xhtml "NES reference guide"). You will find more in-depth documentation about specific topics, tutorials on the most common problems encountered, a list of existing tools that will assist you during you development phase and more. If you're new to NES development, you should give a look to the "getting started" section first. 

## Contents

  * 1 Getting started
  * 2 General reference
  * 3 Tutorials
  * 4 Libraries and samples
    * 4.1 System
    * 4.2 Input
    * 4.3 Graphics
    * 4.4 Math
    * 4.5 Audio
    * 4.6 Misc
    * 4.7 Examples
  * 5 Reverse engineered techniques
  * 6 Useful reference materials
    * 6.1 Books
    * 6.2 Online
  * 7 Links



### Getting started

  * [Before the basics](Before_the_basics.xhtml "Before the basics")
  * [Commissioning](Commissioning.xhtml "Commissioning") \- suggestions for a non-programmer seeking help
  * [Programming Basics](Programming_Basics.xhtml "Programming Basics")
  * [Installing CC65](Installing_CC65.xhtml "Installing CC65")



### General reference

  * [Registers](Registers.xhtml "Registers")
  * [Waiting for the PPU to power on](PPU_power_up_state.xhtml#Best_practice "PPU power up state")
  * [CHR-ROM vs CHR-RAM](CHR_ROM_vs__CHR_RAM.xhtml "CHR-ROM vs CHR-RAM") and how to switch an NROM project from the former to the latter
  * [Emulation Libraries](Emulation_libraries.xhtml "Emulation Libraries")
  * [Catch-up](Catch_up.xhtml "Catch-up") technique used by emulators
  * [Flash Cartridges](Category_Flash_Cartridge.xhtml "Category:Flash Cartridge") ([PowerPak](PowerPak.xhtml "PowerPak") / [Everdrive N8](Everdrive_N8.xhtml "Everdrive N8") / [Everdrive N8 Pro](Everdrive_N8_Pro.xhtml "Everdrive N8 Pro") / [KrzysioCart](KrzysioCart.xhtml "KrzysioCart"))
  * [Music](Music.xhtml "Music"): [Expansion audio](Category_Expansion_audio.xhtml "Category:Expansion audio")
  * [Tools](Tools.xhtml "Tools") — includes assemblers, disassemblers, graphics, music/audio, and emulator-related utilities



### Tutorials

  * [CPU basics](CPU.xhtml "CPU")
  * [PPU reference](PPU.xhtml "PPU")
  * [APU basics](APU_basics.xhtml "APU basics")
  * [Compression](Compression.xhtml "Compression")
    * [Fixed Bit Length Encoding](Fixed_Bit_Length_Encoding.xhtml "Fixed Bit Length Encoding")
  * [Programming mappers](Programming_mappers.xhtml "Programming mappers")
    * [UNROM and UOROM](Programming_UNROM.xhtml "Programming UNROM")
    * [MMC1](Programming_MMC1.xhtml "Programming MMC1")
  * Programming Techniques 
    * [6502 assembly optimisations](6502_assembly_optimisations.xhtml "6502 assembly optimisations")
    * [Cycle counting](Cycle_counting.xhtml "Cycle counting")
    * [RTS Trick](RTS_Trick.xhtml "RTS Trick")
    * [Jump Table](Jump_table.xhtml "Jump Table")
    * [Scanning Tables](Scanning_tables.xhtml "Scanning Tables")
    * [Scanning Large Tables](Scanning_large_tables.xhtml "Scanning Large Tables")
    * [Synthetic Instructions](Synthetic_instructions.xhtml "Synthetic Instructions")
    * [Programming with unofficial opcodes](Programming_with_unofficial_opcodes.xhtml "Programming with unofficial opcodes")
    * [Pointer tables](Pointer_table.xhtml "Pointer table")
    * [Multibyte constant](Multibyte_constant.xhtml "Multibyte constant")
  * [Limitations](Limitations.xhtml "Limitations")
  * [Emulation Tutorials](Emulation_tutorials.xhtml "Emulation Tutorials")
  * [Sample RAM map](Sample_RAM_map.xhtml "Sample RAM map")
  * [PPU scrolling](PPU_scrolling.xhtml "PPU scrolling")
    * How to scroll an infinite map
    * How to scroll with less artifact
    * How to split the screen for a status bar
    * How to do parallax scrolling with the scroll register
  * [Sprite size](Sprite_size.xhtml "Sprite size")
  * [How to make raster effects](http://story.monoroch.net/nes_raster.html) by Bregalad
  * [The frame and NMIs](The_frame_and_NMIs.xhtml "The frame and NMIs") by Disch 
    * [NMI thread](NMI_thread.xhtml "NMI thread"): Make your status bar rock-solid
  * [Nerdy Nights sound](Nerdy_Nights_sound.xhtml "Nerdy Nights sound")
  * [Palette change mid frame](Palette_change_mid_frame.xhtml "Palette change mid frame")
  * [Don't hardcode OAM addresses](Don_t_hardcode_OAM_addresses.xhtml "Don't hardcode OAM addresses")
  * [Interrupt forwarding](Interrupt_forwarding.xhtml "Interrupt forwarding")
  * [Sprite cel streaming](Sprite_cel_streaming.xhtml "Sprite cel streaming")
  * [Drawing terrain](Drawing_terrain.xhtml "Drawing terrain")
  * [Releasing on modern platforms](Releasing_on_modern_platforms.xhtml "Releasing on modern platforms")



### Libraries and samples

#### System

  * [Init code](Init_code.xhtml "Init code")



#### Input

  * [Controller reading](Controller_reading.xhtml "Controller reading")
  * [Controller reading code](Controller_reading_code.xhtml "Controller reading code")



#### Graphics

  * [Detecting video standard](Detect_TV_system.xhtml "Detecting video standard")
  * [Placeholder graphics](Placeholder_graphics.xhtml "Placeholder graphics")
  * [RLE decompression code for CHR](http://forums.nesdev.org/viewtopic.php?p=142703#p142703) by [koitsu](User_Koitsu.xhtml "User:Koitsu")



#### Math

  * [Multiplication by a constant integer](Multiplication_by_a_constant_integer.xhtml "Multiplication by a constant integer")
  * [Fast signed multiply](Fast_signed_multiply.xhtml "Fast signed multiply") \- Fast table driven multiplication
  * [8-bit Multiply](8_bit_Multiply.xhtml "8-bit Multiply") \- Multiplies two 8-bit integers to a 16-bit result
  * [8-bit Divide](8_bit_Divide.xhtml "8-bit Divide") \- Divide two 8-bit integers to a 16-bit result
  * [Division by a constant integer](Division_by_a_constant_integer.xhtml "Division by a constant integer")
  * [Divide by 3](Divide_by_3.xhtml "Divide by 3")
  * [16-bit BCD](16_bit_BCD.xhtml "16-bit BCD") \- An efficient 16-bit binary to decimal converter
  * [Base 100](Base_100.xhtml "Base 100") \- An alternate method of storing numbers, to simplify BCD conversion
  * [Random number generator](Random_number_generator.xhtml "Random number generator")



#### Audio

  * [APU period table](APU_period_table.xhtml "APU period table")
  * [Audio drivers](Audio_drivers.xhtml "Audio drivers")
  * [Nerdtracker player in NESASM](Nerdtracker_player_in_NESASM.xhtml "Nerdtracker player in NESASM")



#### Misc

  * [Skipping the FDS license screen](https://forums.nesdev.org/viewtopic.php?p=194826#p194826)
  * [CRC32 checksum calculation](Calculate_CRC32.xhtml "Calculate CRC32")



#### Examples

  * [Projects](Projects.xhtml "Projects") \- includes many useful open-source examples.
  * [Nerdy Nights Out (aka: Learning 6502)](https://nerdy-nights.nes.science/) by BunnyBoy
  * [Programming NES Games in C](https://shiru.untergrund.net/articles/programming_nes_games_in_c.htm) by Shiru
  * [How to Program an NES game in C](https://nesdoug.com/) by dougeff
  * [Simple game and editing guide (in C)](https://nes-starter-kit.nes.science)
  * [Project template](https://pineight.com/nes/#template) by tepples
  * [Minimal NES example using ca65](https://forums.nesdev.org/viewtopic.php?t=11151) by rainwarrior



### Reverse engineered techniques

  * [Bad Apple](Bad_Apple.xhtml "Bad Apple") video compression
  * [Codemasters tile compression](https://forums.nesdev.org/viewtopic.php?t=5860)
  * [Big Bird's Hide and Speak sample compression](https://forums.nesdev.org/viewtopic.php?t=8675)
  * [Battletoads text compression](https://forums.nesdev.org/viewtopic.php?t=8609) (Huffman coding)
  * [Rad Racer rendering](https://forums.nesdev.org/viewtopic.php?t=8588)



### Useful reference materials

#### Books

  * Cady, Fredrick M. (1997). _Microcontrollers and Microcomputers: Principles of Software and Hardware Engineering_. New York: Oxford University Press. ISBN 0-19-511008-0.
  * Eyes, David; Lichty, Ron (1986). _Programming the 65816 including the 6502, 65C02, and 65802_. New York: Brady Books/Prentice Hall Press (Simon & Schuster, Inc). ISBN 0-89303-789-3. Rights later purchased by Western Design Center.
  * Fernandez, Judi N. (1983). _6502 Assembly Language Programming (Self-teaching Guides)_. John Wiley & Sons Inc. ISBN 978-0471861201.
  * Leventhal, Lance A. (1986). _6502 Assembly Language Programming 2nd Edition_. Osborne/McGraw-Hill. ISBN 0-07-881216-X.
  * Zaks, Rodnay (1983). _Programming the 6502 (Fourth Edition)_. Sybex, Inc. ISBN 0-89588-135-7.
  * Zaks, Rodnay (1982). _Advanced 6502 Programming_. Sybex, Inc. ISBN 0-89588-089-X.
  * Zaks, Rodnay (1980). _Programming the 6502 (Third Edition)_. Sybex, Inc. ISBN 0-89588-046-6.



#### Online

  * [Programming the 65816 (including the 6502, 65C02, and 65802)](http://wdc65xx.com/Programming-Manual/) by Western Design Center 
    1. [Programmingthe65816_ProgManual.pdf](http://www.westerndesigncenter.com/wdc/documentation/Programmingthe65816_ProgManual.pdf) — 54MBytes, created 2015/03/27. 
       * This is a full scan of the original Eyes/Lichty book using full images for each page, with the added bonus of OCR being applied so that text is searchable and copy-paste-able. Accurate/reliable given the nature of the scan. 
         * [backup copy](https://drive.google.com/file/d/1cMzhGjAZPkxlkJxS7c6cRtRO53KC9v1s/view?usp=sharing) via [Koitsu](User_Koitsu.xhtml "User:Koitsu") (primary)
         * [backup copy](https://jdc.koitsu.org/lj/Programmingthe65816_ProgManual.pdf) via [Koitsu](User_Koitsu.xhtml "User:Koitsu") (secondary)
    2. [Programmanual.pdf](https://www.nesdev.org/w/images/default/7/76/Programmanual.pdf "Programmanual.pdf") (a.k.a. WDC65C816_Program_Manual.pdf) — 1.7MBytes, created 2007/04/16. 
       * This is a "pure text" version of the Eyes/Lichty book, however it is rife with very dangerous typos (wrong opcodes, etc.) in many areas, and contains a large number of formatting/layout mistakes not present in the original book. Bill Mensch (6502/65C02/65816 architect/designer) [confirmed these mistakes](http://forums.nesdev.org/viewtopic.php?p=142722#p142722), stating WDC is in the process of fixing them. If at all possible, use the newer PDF mentioned above.
       * It is suspected that WDC has pulled this PDF entirely from [their website](http://65xx.com/Products/Programming_and_Hardware-Manuals/), instead advocating purchasing a "redone" version of the Lichty/Eyes book via Amazon. Kindle and paperback versions are available, but I ([Koitsu](User_Koitsu.xhtml "User:Koitsu")) have not reviewed it.



### Links

  * [The infamous 6502.txt](http://www.zophar.net/documents/6502/6502-txt.html): allegedly complete info on the 6502 instructions
  * [Machine Language for Beginners](http://www.atariarchives.org/mlb/): a 1983 introduction to 6502 assembly language


