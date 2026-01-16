# 6264 static RAM

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/6264_static_RAM) | View [other pages](Special_AllPages.xhtml#6264_static_RAM)

The **6264** is an 8kB static RAM, available in 70 to 200 nanosecond access time variants. It can function on both the NES PPU's 8080 style bus (separate /WE and /OE strobes) or on the CPU's 6500 style bus (by grounding /OE and connecting the R/W signal to /WE). The **62256** is similar but has 32kB of RAM instead. 

## Contents

  * 1 6116
  * 2 6264 Pinout (62256 in parentheses)
  * 3 621024 Pinout (62512 in parentheses)
  * 4 Signal descriptions



## 6116

Certain Konami VRC boards take a 24-pin 2Kx8 SRAM: 
    
    
          ,---\/---.
     A7 --| 1    24|-- Vcc
     A6 --| 2    23|-- A8
     A5 --| 3    22|-- A9
     A4 --| 4    21|o- /WE
     A3 --| 5    20|o- /OE
     A2 --| 6    19|-- A10
     A1 --| 7    18|o- /CS1
     A0 --| 8    17|-- D7
     D0 --| 9    16|-- D6
     D1 --|10    15|-- D5
     D2 --|11    14|-- D4
    Vss --|12    13|-- D3
          `--------
    

(Source: [MB8416](http://matthieu.benoit.free.fr/cross/data_sheets/MB8416-20.pdf); [6116](https://www.idt.com/document/dst/6116sala-data-sheet)) 

## 6264 Pinout (62256 in parentheses)

The 6264 has been used for work RAM and save RAM in numerous NES games, commonly [decoded at $6000-$7FFF](PRG_RAM_circuit.xhtml "PRG RAM circuit"). The 62256 is used in _RacerMate Challenge II_ , _Romance of the Three Kingdoms II_ , and a few Famicom games. 

  * 6264: 8Kx8-bit SRAM
  * 62256: 32Kx8-bit SRAM


    
    
               .----\/----.
    (A14) nc - |01      28| - +5V
         A12 - |02      27| - /WE
          A7 - |03      26| - CS2 (A13) 
          A6 - |04      25| - A8      
          A5 - |05      24| - A9      
          A4 - |06      23| - A11       
          A3 - |07      22| - /OE       
          A2 - |08      21| - A10       
          A1 - |09      20| - /CS1    
          A0 - |10      19| - D7      
          D0 - |11      18| - D6      
          D1 - |12      17| - D5      
          D2 - |13      16| - D4      
         GND - |14      15| - D3      
               `----------'
    

## 621024 Pinout (62512 in parentheses)

In the NES, these chips might be used in "EPROM emulators", or devices into which a game is loaded from an external source (such as a connected personal computer or a NAND flash) before it runs. 

  * 621024: 128Kx8-bit SRAM
  * 62512: 512Kx8-bit SRAM (breaks name pattern)


    
    
               .----\/----.
    (A18) nc - |01      32| - +5V
         A16 - |02      31| - A15
         A14 - |03      30| - CS2 (A17)
         A12 - |04      29| - /WE
          A7 - |05      28| - A13
          A6 - |06      27| - A8      
          A5 - |07      26| - A9      
          A4 - |08      25| - A11       
          A3 - |09      24| - /OE       
          A2 - |10      23| - A10       
          A1 - |11      22| - /CS1    
          A0 - |12      21| - D7      
          D0 - |13      20| - D6      
          D1 - |14      19| - D5      
          D2 - |15      18| - D4      
         GND - |16      17| - D3      
               `----------'
    

## Signal descriptions

A0-A12(-A14)
    address
D0-D7
    data
/CS1 and (when present) CS2 (chip selects 1 and 2)
    when /CS1 is low and CS2 is high, the chip is selected and will accept input or provide output to the data bus.
/WE (write enable)
    When the chip is selected, if this pin is low, the eight-bit data D will be written to the address inside the RAM specified by A
/OE (output enable)
    When the chip is selected, if this pin is low and /WE is high, the eight bit data in the RAM at address A will be output onto the pins D.

The difference between /OE and the chip selects is that the 6264 responds much faster to /OE, but it draws less power when the chip selects are deasserted. 

A circuit to generate /CS1 for decoding a 6264 RAM at $6000-$7FFF is described at [PRG RAM circuit](PRG_RAM_circuit.xhtml "PRG RAM circuit"). It can also be used to decode a 62256 if you have latches available to control A13 and A14. 

Categories: [Integrated circuits](Category_Integrated_circuits.xhtml)
