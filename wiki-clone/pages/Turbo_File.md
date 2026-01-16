# Turbo File

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Turbo_File) | View [other pages](Special_AllPages.xhtml#Turbo_File)

The Turbo File (for [Family Computer](Family_Computer.xhtml "Famicom")) is a Famicom expansion port peripheral developed by ASCII Corporation, and is used to store save data for certain games. Variants of the device were also produced for the Game Boy (Turbo File GB), Game Boy Advance (Turbo File Advance), and Super Famicom (Turbo File Twin). Original Famicom Turbo File devices can be connected to a Super Famicom using an adapter unit, allowing certain SFC games to use the device in lieu of the TFTwin. 

The device exists in two iterations; the original model (AS-TFO2) containing 8K of battery-backed SRAM, and the Turbo File II (AS-TF21) containing 32K of battery-backed SRAM (HM62256ALP-15), which is split into four 8K banks that can be manually switched to allow for four individual game saves. Both devices have an LED that illuminates when connected to a Famicom that is powered on, as well as a write protect switch to prevent accidentally overwriting save data. 

The TFII also includes a proprietary cartridge slot that accepts "データROM" (Data ROM) packs that can add extra functionality to supported titles. 

Famicom games that support the Turbo File include: 

  * _Best Keiba - Derby Stallion_ (1991)
  * _Best Play Pro Yakyuu_ (1988)
  * _Best Play Pro Yakyuu '90_ (1990)
  * _Best Play Pro Yakyuu II_ (1990)
  * _Best Play Pro Yakyuu Shin Data_ (1988)
  * _Best Play Pro Yakyuu Special_ (1992)
  * _Castle Excellent_ (1986)
  * _Derby Stallion - Zenkoku Ban_ (1992)
  * _Downtown - Nekketsu Monogatari_ (1989)
  * _Dungeon Kid_ (1990)
  * _Famicom Shougi - Ryuuousen_ (1991)
  * _Fleet Commander_ (1988)
  * _Haja no Fuuin_ (1987)
  * _Itadaki Street - Watashi no Mise ni Yottette_ (1990)
  * _The Money Game 2 - Kabutochou no Kiseki_ (1989)
  * _Ninjara Hoi!_ (1990)
  * _Wizardry - Legacy of Llylgamyn_ (1989)
  * _Wizardry - Proving Grounds of the Mad Overlord_ (1987)
  * _Wizardry - The Knight of Diamonds_ (1991)



Most titles bear a "TF" logo on the cartridge label signifying their compatibility. 

Games that are believed to potentially work with the device (but remain unconfirmed) include: 

  * _Castlequest_ (1989) (US version of _Castle Excellent_.)
  * _Kunio 8-in-1_ (Pirate multicart that contains _Downtown - Nekketsu Monogatari_ listed as "HEROS STORY" in the selection menu.)



## Memory Setup and File Format

The first byte of memory (offset 0000h) is unused, potentially out of fear that certain games with controller access might disrupt it, so a dummy byte is used to skip it after resetting the address. All the rest of the space (0001h-1FFFh) is used to store save data, with files being attached directly after each other. Invalid file IDs indicate the start of free memory. 

The majority of Turbo File games utilize this format for save files: 
    
    
    2   ID "AB" (41h,42h)
    2   Filesize (16+N+2) (including title and checksum)
    16  Title in ASCII (terminated by 00h or 01h)
    N   Data Portion
    2   Checksum (all N bytes in Data Portion added together)
    

The exception to this, _Castle Excellent_ , uses a unique file format, shown here: 
    
    
    1   Don't care (should be 00h)    ;fixed, at offset 0001h
    2   ID AAh,55h                    ;fixed, at offset 0002h..0003h
    508 Data Portion (Data, end code "BEDEUTUN", followed by some unused bytes)
    

This game also forgoes a filename and utilizes a hardcoded memory offset of 511 bytes (0001h-01FFh). Due to the hardcoded memory offset, _Castle Excellent_ will destroy any other file that is located at the same address. Some later games for both the Famicom and Super Famicom (including _Fleet Commander_) are able to properly manage the Castle Excellent save file. 

## Reference

  * nocash's NES documentation: <https://problemkaputt.de/everynes.htm#storageturbofile>



Categories: [File formats](Category_File_formats.xhtml), [Hardware](Category_Hardware.xhtml)
