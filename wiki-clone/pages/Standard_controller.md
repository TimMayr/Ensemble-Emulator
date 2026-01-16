# Standard controller

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Standard_controller) | View [other pages](Special_AllPages.xhtml#Standard_controller)

[![](../wiki-images/NES-Controller-Flat.jpg)](File_NES_Controller_Flat_jpg.xhtml)

[](File_NES_Controller_Flat_jpg.xhtml "Enlarge")

Standard NES controller

All NES units come with at least one standard controller - without it, you wouldn't be able to play any games! 

Standard controllers can be used in both controller ports, or in a [Four score](Four_player_adapters.xhtml "Four score") accessory. 

    _For code examples, see:[Controller reading code](Controller_reading_code.xhtml "Controller reading code")_

## Contents

  * 1 Report
  * 2 Input ($4016 write)
  * 3 Output ($4016/$4017 read)
    * 3.1 Famicom
  * 4 Hardware
    * 4.1 PAL
    * 4.2 Schematic
  * 5 Turbo
  * 6 See also
  * 7 References



## Report

The standard NES controller will report 8 bits on its data line: 
    
    
    0 - A
    1 - B
    2 - Select
    3 - Start
    4 - Up
    5 - Down
    6 - Left
    7 - Right
    

After 8 bits are read, all subsequent bits will report 1 on a standard NES controller, but third party and other controllers may report other values here. 

If using DPCM audio samples, read conflicts must be corrected with a software technique. The most common symptom of this is spurious Right presses as the DPCM conflict deletes one bit of the report, and an extra 1 bit appears in the Right press position. See: [Controller reading: DPCM conflict](Controller_reading.xhtml#DPCM_conflict "Controller reading"). 

## Input ($4016 write)
    
    
    7  bit  0
    ---- ----
    xxxx xxxS
            |
            +- Controller shift register strobe
    

While S ([strobe](Glossary.xhtml "Glossary")) is high, the shift registers in the controllers are continuously reloaded from the button states, and reading $4016/$4017 will keep returning the current state of the first button (A). Once S goes low, this reloading will stop. Hence a 1/0 write sequence is required to get the button states, after which the buttons can be read back one at a time. 

(Note that bits 2-0 of $4016/write are stored in internal latches in the 2A03/07.) 

## Output ($4016/$4017 read)
    
    
    7  bit  0
    ---- ----
    xxxx xMES
          |||
          ||+- Primary controller status bit
          |+-- Expansion controller status bit (Famicom)
          +--- Microphone status bit (Famicom, $4016 only)
    

Though both are polled from a write to $4016, controller 1 is read through $4016, and controller 2 is separately read through $4017. 

Each read reports one bit at a time through D0. The first 8 reads will indicate which buttons or directions are pressed (1 if pressed, 0 if not pressed). All subsequent reads will return 1 on official Nintendo brand controllers but may return 0 on third party controllers such as the [U-Force](https://www.nesdev.org/w/index.php?title=U-Force&action=edit&redlink=1 "U-Force \(page does not exist\)"). 

Status for each controller is returned as an 8-bit report in the following order: A, B, Select, Start, Up, Down, Left, Right. 

In the NES and Famicom, the top three (or five) bits are not driven, and so retain the bits of the previous byte on the bus. Usually this is the most significant byte of the address of the controller port—0x40. Certain games (such as Paperboy) rely on this behavior and require that reads from the controller ports return exactly $40 or $41 as appropriate. _See:[Controller reading: unconnected data lines](Controller_reading.xhtml#Unconnected_data_lines_and_open_bus "Controller reading")._

When no controller is connected, the corresponding status bit will report 0. This is due to the presence of internal pull-up resistors, and the internal inverter. (See: [Controller reading](Controller_reading.xhtml "Controller reading")) 

### Famicom

[![](../wiki-images/Nintendo-Famicom-Controller-II-FL.jpg)](File_Nintendo_Famicom_Controller_II_FL_jpg.xhtml)

[](File_Nintendo_Famicom_Controller_II_FL_jpg.xhtml "Enlarge")

Famicom second-player controller

The original Famicom's hard-wired second controller (II) is missing the Select and Start buttons. Its corresponding bits will read as 0, so Famicom games must not rely on the second player being able to push Start or Select. 

This hard-wired second controller also contains a microphone, which gives an immediate 1-bit report at $4016 D2 whenever it is read. 

The later AV Famicom used detachable controllers, with connectors identical to the NES. Its second controller was the same as the first, with Select and Start present, and no microphone. 

The expansion port of the Famicom could be used to connect external controllers. These gave the same standard 8-bit report, but through D1 instead of D0. It was common for Famicom games to combine D1 and D0 (logical OR) when reading to permit players to use expansion controllers instead, though several games do not support this[1]. Alternatively, these could be used as extra controllers for [4-player](Four_player_adapters.xhtml "Four Score") games. 

## Hardware

The [4021 IC](4021.xhtml "4021") is an 8-bit parallel-to-serial shift register. This allows the 8 button states to be latched into the register simultaneously (parallel), then read out one bit at a time (serial). 

From the [controller port pinouts](Controller_port_pinout.xhtml "Controller port pinout"): 

  * **OUT** (`$4016:0`) controls the 4021's **Parallel/Serial Control**. When this goes high, the current state of the 8 buttons are read into the 4021's 8-bit register.
  * **CLK** controls the 4021's **Clock** input. On a low-to-high transition this will shift each bit of the register to the next higher bit. This is normally held high, and becomes low during a read of `$4016` or `$4017`. When the read is finished, it returns high, triggering the shift to prepare for the next bit to be read.
  * **D0** reads the 4021's **Q8** output. This is the current state of the last bit in the register. Note that the NES will invert this signal, so for the 4021 unpressed buttons are stored as 1, but will read to the NES CPU as 0.
  * **+5V** powers the 4021 through its **Vcc** pin.
  * **GND** provides ground to the 4021 through **Vss** but is also connected to its **Serial In** input, which will shift a 0 into each empty bit as the 4021 is clocked. This is why (after inversion) the standard controller will read back all 1s once the 8 buttons have been read.



Each of the 8 **PI** parallel input pins is connected to **+5V** through its own pull-up resistor (~10k), keeping it high normally. When a button is pressed, it connects the input to ground, bypassing the pull-up, creating a low signal when latched. **PI-8** corresponds to the first button read: the **A** button. 

If using DPCM audio samples, read conflicts may occur requiring a software technique to correct for them. See: [Controller reading: DPCM conflict](Controller_reading.xhtml#DPCM_conflict "Controller reading"). 

### PAL

Some PAL region consoles (for example FRA, HOL, NOE) have internal diodes on their controller port. (See: [Controller port pinout: Protection Diodes](Controller_port_pinout.xhtml#Protection_Diodes "Controller port pinout")) 

These diodes prevent the Clock and Latch signals from functioning unless they are pulled high. PAL controllers for these regions (model NES-004E) each contain a 3.6KΩ resistor between these two inputs and 5V.[2]

On these systems, only PAL controllers with the pull-ups can be read. NTSC systems, along with early PAL market systems (at least SCN) can read controllers of either type. Modifying the internal controller port to bypass these diodes will make the PAL system compatible with both. Conversely, modifying a controller to add the pull-up resistors makes it compatible with both types of systems. 

This also extends to the 4-score peripheral: Model NESE-034 ver1.1 is also diode protected and will require pull-up-equipped controllers. 

### Schematic
    
    
                         .-----\/-----.
    *--------- A Button -|PI-8     Vcc|- 5V
                      x -|Q6      PI-7|- B Button -------*
       +------------ D0 -|Q8      PI-6|- Select Button --*
    *--|----- Up Button -|PI-4    PI-5|- Start Button ---*
    *--|--- Down Button -|PI-3      Q7|- x                    |\
    *--|--- Left Button -|PI-2   SerIn|- GND             GND -|o\
    *--|-- Right Button -|PI-1   Clock|---+------------- CLK -|oo|- 5V
       |            GND -|Vss    Latch|---|--+---------- OUT -|oo|- D3 --x
       |                 .____4021____.   |  |  +-------- D0 -|oo|- D4 --x
       |                                  |  |  |             .__.
       +----------------------------------|--|--+            (Port)
                                          |  |
    8 x *--+--[ 10k ]-- 5V                |  | (PAL pullups)
           |                              +--|--[ 3.6k ]--+-- 5V
           |   __+__ (Button)                |            |
           +---o   o--- GND                  +--[ 3.6k ]--+
    

## Turbo

A **turbo controller** such as the NES Max or NES Advantage is read just like a standard controller, but the user can switch some of its buttons to be toggled by an oscillator. Such an oscillator turns the button on and off at 15 to 30 Hz, producing rapid fire in games. 

A controller should not toggle the button states on each strobe pulse. Doing so will cause problems for games that poll the controller in a loop until they get two identical consecutive reads (see DMC conflict above). The game may halt while the turbo button is held, or crash, or cause other unknown behaviour. 

## See also

  * [Controller reading](Controller_reading.xhtml "Controller reading")
  * [Controller detection](Controller_detection.xhtml "Controller detection")
  * [Controller port pinout](Controller_port_pinout.xhtml "Controller port pinout")
  * [Four Score](Four_player_adapters.xhtml "Four Score") 4-player adapter
  * [SNES controller](SNES_controller.xhtml "SNES controller") has backward compatible protocol with the NES



## References

  1. ↑ [Famicom World forum post](https://www.famicomworld.com/forum/index.php?topic=6701.msg167005#msg167005): Famicom games that do not work with pads connected through the expansion port.
  2. ↑ [Forum post](https://forums.nesdev.org/viewtopic.php?p=238272#p238272): explaining PAL controller diodes and their function.



  * [Forum post:](https://forums.nesdev.org/viewtopic.php?f=9&t=8848) Famicom controller PCB and exterior photographs
  * [Hori controller schematic](https://wiki.console5.com/wiki/File:Hori_HJ-12_Schematic.png) \- Third party controller similar to the NES or Famicom controller, and contains the Clock/Latch pull-ups needed for PAL compatibility.



Categories: [Controllers](Category_Controllers.xhtml)
