# User:Alphamule

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AAlphamule) | View [other pages](Special_AllPages.xhtml#User_Alphamule)

The **6264** is an 8kB static RAM, available in 70 to 200 nanosecond access time variants. It can function on both the NES PPU's 8080 style bus (separate /WE and /OE strobes) or on the CPU's 6500 style bus (by grounding /OE and connecting the R/W signal to /WE). The **62256** is similar but has 32kB of RAM instead. 

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
    

## 32-pin 62xxxx Pinout
    
    
    (62512 in parentheses, 2048 in bold, 8) ==
    

In the NES, these chips might be used in "EPROM emulators", or devices into which a game is loaded from an external source (such as a connected personal computer or a NAND Flash drive) before it runs. 

  * 621024: 128Kx8-bit SRAM
  * 62512: 512Kx8-bit SRAM (breaks name pattern as this is a 4096Kb device)


    
    
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

Categories: [Integrated circuits](Category_Integrated_circuits.xhtml)
