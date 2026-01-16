# Extended NES I/O

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Extended_NES_I/O) | View [other pages](Special_AllPages.xhtml#Extended_NES_I_O)

The Extended NES I/O (ENIO) is a homebrew device by Chykn consisting of an EXP board and a CPU board. The EXP board plugs into the NES-001 expansion port and provides expansion audio and various headers, while the CPU board attaches to the EXP board and provides networking, storage, and USB. The EXP board was produced and sold in small quantities, while the CPU board did not make it beyond prototypes and was not generally sold. As such, much of this information has been sourced from archives of [public documentation](https://web.archive.org/web/20161225130448/http://enio.chykn.com/wiki/index.php/Main_Page). 

## Contents

  * 1 EXP board
    * 1.1 Pinout (v1.2)
      * 1.1.1 J1: NES EXP connector
      * 1.1.2 J2: Famicom EXP Pin Compatible Header (16-pin R/A IDC)
      * 1.1.3 J3: ENIO CPU Board Header (26-pin R/A IDC)
      * 1.1.4 J4: Misc Signal Header (16-pin unpopulated)
      * 1.1.5 J5: Audio and Video Out (3-pin unpopulated)
      * 1.1.6 J6: Audio In (2-pin unpopulated)
      * 1.1.7 J7: Power (3-pin Molex KK 254)
  * 2 CPU board
    * 2.1 Addressing
      * 2.1.1 Compatibility Mode
      * 2.1.2 Direct Addressing Mode
  * 3 References



# EXP board

The EXP board enables expansion audio by connecting EXP6 to the expansion port's audio in with a 47K resistor. Headers are included breaking out the various expansion port signals, one of them providing Famicom expansion port functionality via an included cable. 

## Pinout (v1.2)

### J1: NES EXP connector

See [NES expansion port](Expansion_port.xhtml#Pinout_2 "Expansion port"). Note that pins 1 and 48 (both +5V) are not connected on the EXP board. 

### J2: Famicom EXP Pin Compatible Header (16-pin R/A IDC)
    
    
                                +------+
                         GND -- |01  09| -> Joypad 2 /OE
    Audio In (not connected) -> |02  10| -> OUT2
                        /IRQ ?? |03  11| -> OUT1
                 Joypad 2 D4 ?? |04  12  -> OUT0
                 Joypad 2 D3 ?? |05  13  <- Joypad 1 D1
                 Joypad 2 D2 -> |06  14| -> Joypad 1 /OE
                 Joypad 2 D1 -> |07  15| -> +5V (J1.1)
                 Joypad 2 D0 ?? |08  16| -- NC
                                +------+ 
    

Notes: 

  * ?? directionality depends on the presence of other hardware, such as a joypad or a cartridge that always drives /IRQ. If conflicting hardware is not present, these can be used as inputs to the console.
  * **J2.2 Audio In** : On the Famicom, this pin is used for audio out, though can accept audio input. Here, it connects to J6.2 and is intended to connect to the NES audio in via a 47K resistor, which appears to normally not be populated, leaving this pin not connected.
  * Pin order differs on this header compared to J3 and J4.
  * The EXP board came with an adapter cable that connects to this header and provides a Famicom-compatible male DA15 port.



### J3: ENIO CPU Board Header (26-pin R/A IDC)
    
    
                       +------+
         +5V (J1.1) -- |01  02| -- GND
                A15 <- |03  04| -> OUT2
               OUT1 <- |05  06| -> OUT0
               EXP9 ?? |07  08| -> /CE (EXP8)
         R/W (EXP7) <- |09  10| ?? EXP5
       Joypad 2 /OE <- |11  12| ?? Joypad 2 D0
        Joypad 2 D1 -> |13  14  <- Joypad 2 D2
        Joypad 2 D3 ?? |15  16| ?? Joypad 2 D4
             CPU D0 <> |17  18| <> CPU D1
             CPU D2 <> |19  20| <> CPU D3
             CPU D4 <> |21  22| <> CPU D5
             CPU D6 <> |23  24| <> CPU D7
    Unregulated Vdd -- |25  26| ?? /IRQ
                       +------+
    

Notes: 

  * ?? directionality depends on the presence of other hardware, such as a joypad or a cartridge that always drives /IRQ. If conflicting hardware is not present, these can be used as inputs to the console.



### J4: Misc Signal Header (16-pin unpopulated)
    
    
         +5V (J1.48) -- |01  02| -- GND
                EXP0 ?? |03  04| <> /NMI
                EXP2 ?? |05  06| ?? EXP1
                EXP4 ?? |07  08| ?? EXP3
         Joypad 1 D4 ?? |09  10| -> Joypad 1 /OE
         Joypad 1 D2 -> |11  12| ?? Joypad 1 D3
         Joypad 1 D0 ?? |13  14| -> Joypad 1 D1
    4.00 MHz CIC CLK <- |15  16| ?? /IRQ
    

Notes: 

  * ?? directionality depends on the presence of other hardware, such as a joypad or a cartridge that always drives /IRQ. If conflicting hardware is not present, these can be used as inputs to the console.



### J5: Audio and Video Out (3-pin unpopulated)
    
    
    |1| -- GND
    |2| -> Audio Out
    |3| -> Video Out
    

### J6: Audio In (2-pin unpopulated)
    
    
    |1| -- GND
    |2| <- Audio In (not connected)
    

Notes: 

  * J6.2 Audio In: Connects to J2.2, but does not connect to the console. See J2.2 above.



### J7: Power (3-pin Molex KK 254)
    
    
    1  -- Unregulated Vdd
    2| -- +5V (J1.48)
    3  -- GND
    

# CPU board

The CPU board contains the following components: 

  * PIC32
  * 100 Mb ethernet port
  * MicroSD cart slot (for bootloading or game data storage)
  * USB A port (such as for keyboards)
  * MiniUSB port (for power if not connected to the console)
  * 26-pin IDC header (see EXP board J3 above)
  * Xilinx CPLD
  * 3.3V switching regulator (selectable between 5V and unregulated Vdd, and 5V selectable between NES and miniUSB)



The CPLD enables communication between the NES and PIC, raising an interrupt on the PIC on access, and also acts as a level shifter. The PIC establishes network connections, transfers data, and buffers the CGP (Classic Gaming Protocol)[1] packets sent between the ENIO and NES. 

## Addressing

The CPU board can be accessed 8 bits at a time in two ways. 

### Compatibility Mode

This can be used with any cartridge hardware, but is not compatible with a second controller. This uses: 

  * $4017R for reads
  * Any address below $8000 for writes
  * OUT1 as R/W



### Direct Addressing Mode

This uses the cartridge to map reads and writes to $4700, supplying R/W and /CE to the CPU board via EXP pins: 
    
    
    EXP7: R/W
    EXP8: /CE
    

# References

  1. ↑ [Classic Gaming Protocol proposal](https://web.archive.org/web/20150817135809/http://enio.chykn.com/wiki/index.php/CGP)


