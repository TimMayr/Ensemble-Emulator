# Mask ROM pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Mask_ROM_pinout) | View [other pages](Special_AllPages.xhtml#Mask_ROM_pinout)

## Contents

  * 1 8kB / 16kB / 32kB / 64kBytes (28pin) ROMs
  * 2 128/256/512 KBytes (28/32pin) ROMs
  * 3 Variants
    * 3.1 Nintendo RROM CHR ROM pinout - 8 KBytes (28pin)
    * 3.2 Nintendo SROM CHR ROM pinout - 8 KBytes (24pin)
    * 3.3 Nintendo STROM
    * 3.4 Nintendo AOROM PRG ROM pinout - 128/256/KBytes (32pin)
  * 4 Signal descriptions
  * 5 Converting a donor board
  * 6 See also



## 8kB / 16kB / 32kB / 64kBytes (28pin) ROMs

Nintendo used by default JEDEC standard compatible pinouts for all their mask ROMs of 64 kBytes and below (but some particular boards might be exceptions !). Names [in brackets] applies when the corresponding address pin is unused. On boards where an adress pin is never used (for example, A15 is never used on [NROM](NROM.xhtml "NROM") boards as the ROM can't be greater than 32k), what is in brackets connects to the unused pin. 

For some unknown reasons, unused address lines on smaller ROMs **had to be** put to +5V, as Nintendo made different boards for each size (as opposed to place EPROMs of different sizes into the same slot). Some boards, such as CNROM, features solder pads in order to force those pins to +5V though. As Nintendo liked to use Mask ROMs from various manufacturers, apparently unused pins could be either not connected or additional active high CE enable lines. This would ensure that if Nintendo would order mask ROMs with additional enable lines that aren't used, they could place them on the board without having the fear of having them disabled accidentally. 

This doesn't apply to CHR ROMs - i.e. a smaller ROM can always fit a slot made with a larger ROM in mind. Probably as the game will work if the ROM is "accidentally" disabled before the first write to CHR bankswitching registers. 
    
    
      27C64/128/256/512 EPROM pinout
    
                  ---_---
     [+5V] A15 - |01   28| - +5V
           A12 - |02   27| - A14 [PGM]
           A7  - |03   26| - A13 [NC]
           A6  - |04   25| - A8
           A5  - |05   24| - A9
           A4  - |06   23| - A11
           A3  - |07   22| - /OE
           A2  - |08   21| - A10
           A1  - |09   20| - /CE
           A0  - |10   19| - D7
           D0  - |11   18| - D6
           D1  - |12   17| - D5
           D2  - |13   16| - D4
           GND - |14   15| - D3
                  -------
    

## 128/256/512 KBytes (28/32pin) ROMs

Nintendo used the standard pinout for (extremely rare) prototype boards intended to take a 27C010/020/040 EPROM. But retail Game Paks made by Nintendo have mask ROMs with a different pinout which is not compatible. This pinout, with reshuffled enable lines and higher address lines, allows a 32-pin hole to take a 28-pin 128 KiB PRG ROM in pin 3 to pin 30. 

For games using 256 KiB and larger ROMs, other companies producing Famicom or NES boards used either epoxy blobs or standard EPROM pinout. But games with 128 KiB of PRG were more often in a 28-pin package than not. 

On Nintendo's boards where PRG A18 is never used, pin 2 is connected with pin 22. 

Nintendo's MMC5 boards use the same pinout for both PRG and CHR ROMs, and even 128 KB PRG ROMs are 32-pin so they can have a VCC pin. 
    
    
            CHR ROM, PRG ROM, and 27C010/020/040/080 EPROM pinouts compared
     
     MMC5    CHR        PRG      EPROM                   EPROM        PRG   CHR   MMC5
                                             ---_---
     A17    A17 [+5V]  A17      [VPP] A19 - |01   32| - +5V
     A18    /OE        A18 [/CE]      A16 - |02   31| - A18 [PGM]    +5V    /CE2   /CE2
                                      A15 - |03   30| - A17 [NC]     +5V    +5V    A19
                                      A12 - |04   29| - A14
                                      A7  - |05   28| - A13
                                      A6  - |06   27| - A8
                                      A5  - |07   26| - A9
                                      A4  - |08   25| - A11
                                      A3  - |09   24| - /OE          A16    A16    A16
                                      A2  - |10   23| - A10
                                      A1  - |11   22| - /CE                 GND
                                      A0  - |12   21| - D7
                                      D0  - |13   20| - D6
                                      D1  - |14   19| - D5
                                      D2  - |15   18| - D4
                                      GND - |16   17| - D3
                                             -------
    
    PRG and CHR pins are listed only if they differ from EPROM.
    
    

## Variants

Here is a list of multiple variants of Nintendo's pinouts above. Only a couple of enable pins typically differs (which are shown in bold). 

### Nintendo RROM CHR ROM pinout - 8 KBytes (28pin)

This particular board is functionally identical to NROM but with a strange CHR pinout : 
    
    
      [RROM](https://nescartdb.com/nescartdb.com/profile/view/2314) Non-standard 64-kbit CHR pinout
                 ---_---
          **A7**  - |01   28| - +5V
          **A6**  - |02   27| - **A8**
          **A5**  - |03   26| - **A9**
          **A4**  - |04   25| - **A12**
          **A3**  - |05   24| - **/CE**
          **+5V** - |06   23| - **NC**
          **+5V** - |07   22| - /OE
          A2  - |08   21| - A10
          A1  - |09   20| - **A11**
          A0  - |10   19| - D7
          D0  - |11   18| - D6
          D1  - |12   17| - D5
          D2  - |13   16| - D4
          GND - |14   15| - D3
                 -------
    

### Nintendo SROM CHR ROM pinout - 8 KBytes (24pin)

This particular board is functionally identical to NROM but the CHR uses a 24-pin 8KB ROM with pinout very similar to the 27C32: 
    
    
      SROM  23C62/64 JEDEC-Standard 64-kbit CHR pinout
                ---_---
          A7 - |01   24| - Vcc
          A6 - |02   23| - A8
          A5 - |03   22| - A9
          A4 - |04   21| - **A12**
          A3 - |05   20| - /OE
          A2 - |06   19| - A10
          A1 - |07   18| - **A11**
          A0 - |08   17| - D7
          D0 - |09   16| - D6
          D1 - |10   15| - D5
          D2 - |11   14| - D4
         Gnd - |12   13| - D3
                -------
    

With only one output enable, the board synthesizes the signal (/OE = PPU A13 + PPU /RD) using two transistors and a resistor. 

### Nintendo STROM

This board uses three 23C62, 8 KBytes, 24 pin MASK ROMs (two as PRG-ROM and one as CHR-ROM) and a few discrete elements for an address decoder [[1]](https://forums.nesdev.org/viewtopic.php?f=9&t=3447&p=274110#p25434)

### Nintendo AOROM PRG ROM pinout - 128/256/KBytes (32pin)

Very slight variant of the standard PRG-ROM pinout above, where an additional active high enable line is used to prevent bus conflicts. 
    
    
                  ---_---
           A17 - |01   32| - +5V
           /CE - |02   31| - **CE (R/W)**
           A15 - |03   30| - +5V
           A12 - |04   29| - A14
           A7  - |05   28| - A13
           A6  - |06   27| - A8 
           A5  - |07   26| - A9
           A4  - |08   25| - A11
           A3  - |09   24| - A16
           A2  - |10   23| - A10
           A1  - |11   22| - /CE
           A0  - |12   21| - D7
           D0  - |13   20| - D6
           D1  - |14   19| - D5
           D2  - |15   18| - D4
          GND  - |16   17| - D3
                  -------
    

Japanese _After Burner_ connects two such 128 kB mask roms in parallel (pin 2 = PPU /RD, pin 22 = SS4-CHR-/CE, pin31 = SS4-CHR-A17), which implies that one of them needs to have pin 31 active low and the other active high. [[2]](https://nescartdb.com/profile/view/3806/after-burner)

## Signal descriptions

A0-A12
    address
D0-D7
    data
/CE, /OE
    The ROM will output data at address A on pins D only if all its CE (chip enable) and OE (output enable) pins are active (CE active high, /CE active low). /CE is sometimes called /CS (chip select). A memory responds faster to /OE than to /CE but draws less current while /CE is not asserted.
PGM, VPP
    Used only during EPROM programing / erasing process.

See [Cartridge connector](Cartridge_connector.xhtml#Signal_Descriptions "Cartridge connector") for other signals descriptions. 

## Converting a donor board

To convert a cartridge board to accept EPROM: 

  1. Compare the pinout of your EPROM to the mask ROM pinout expected by the board.
  2. Lift any pins whose signals differ.
  3. Solder short wires to the corresponding holes.
  4. Solder down the memory or socket with the pins that do not differ.
  5. Solder each lifted pin to the hole with the same signal.



## See also

  * [NES EPROM Conversions](http://nesdev.org/NES%20EPROM%20Conversions.txt) by Drk. Instructions on how to modify certain boards to use EPROMs.
  * [NES ROM pinouts](http://nesdev.org/NES%20ROM%20Pinouts.txt) by Drk. Covers all PRG, CHR, and RAM chips used in NES cartridges.
  * [EPROM pinouts](http://nesdev.org/EPROM%20Pinouts.txt) by Drk.
  * [Advanced MMC3 NES Reproduction Tutorial](http://callanbrown.com/index.php/advanced-mmc3-nes-reproduction) by Callan Brown
  * [SNROM to SUROM](http://forums.nesdev.org/viewtopic.php?p=171375#p171375) by Ice Man



Categories: [Pinouts](Category_Pinouts.xhtml)
