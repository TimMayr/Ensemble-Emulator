# Power Pad

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Power_Pad) | View [other pages](Special_AllPages.xhtml#Power_Pad)

The Power Pad exercise mat for the Nintendo Entertainment System has twelve sensors arranged in a 3x4 grid, which the player steps on to control the action on the screen. 

The Power Pad can be used in either [controller port](Controller_port_pinout.xhtml "Controller port pinout"), though most games will only allow you to use it in controller port #2, leaving port #1 for a standard controller used for navigating through options. 
    
    
              |                        |
    ,---------+---------.    ,---------+---------.
    | POWER PAD  side B |    | POWER PAD  side A |
    |  (1) (2) (3) (4)  |    |      (O) (O)      |
    |                   |    |                   |
    |  (5) (6) (7) (8)  |    |  (O) (X) (X) (O)  |
    |                   |    |                   |
    |  (9) (10)(11)(12) |    |      (O) (O)      |
    |                   |    |                   |
    `-------------------'    `-------------------'
    

The sensors are spaced at 252 mm across by 285 mm front and back. 

Most games used side B, with the numbers on top. A few games turned the pad over to side A, whose markings lack numerals and lack markings for spaces 1, 4, 9, and 12 entirely (but they still send a signal). There is a third possible configuration, which no official game used, but which may be useful for homebrew dance simulation games in the style of Dance Dance Revolution: side B rotated 90 degrees anticlockwise, placing sensors 4, 8, and 12 toward the display. 
    
    
    | ,---------------.           | ,---------------.
    | |  SIDE B       |           | |  SIDE DDR     |
    | |  (4) (8) (12) |           | | (Sel)    (St) |
    | |               |           | |               |
    | |  (3) (7) (11) |    ____   | |  (X) (U) (O)  |
    `-+               |    ____   `-+               |
      |  (2) (6) (10) |             |  (L)     (R)  |
      |               |             |               |
      |  (1) (5) (9)  |             |      (D)      |
      |               |             |               |
      `---------------'             `---------------'
    

## Contents

  * 1 Hardware interface
    * 1.1 Input ($4016 write)
    * 1.2 Output ($4016/$4017 read)
  * 2 Family Trainer Mat
    * 2.1 Input ($4016 write)
    * 2.2 Output ($4017 read)
  * 3 Programs
  * 4 References



## Hardware interface

### Input ($4016 write)
    
    
    7  bit  0
    ---- ----
    xxxx xxxS
            |
            +- Controller shift register strobe
    

Writing 1 to this bit will record the state of each button. Writing 0 afterwards will allow the buttons to be read back, two at a time. 

### Output ($4016/$4017 read)
    
    
    7  bit  0
    ---- ----
    xxxH Lxxx
       | |
       | +---- Serial data from buttons 2, 1, 5, 9, 6, 10, 11, 7
       +------ Serial data from buttons 4, 3, 12, 8 (following 4 bits read as H=1)
    

The first 8 reads will indicate which buttons are pressed (1 if pressed, 0 if not pressed); all subsequent reads will return H=1 and L=1. 

Remember to save _both_ bits that you get from each read. If you're playing [samples](APU_DMC.xhtml "APU DMC"), beware of [bit deletions](Standard_controller.xhtml#Evil_Details "Standard controller"). 

## Family Trainer Mat

The Famicom's version of the Power Pad looked similar, but had an entirely different protocol that took advantage of the greater number of digital outputs on the [Famicom expansion port](Expansion_port.xhtml "Famicom expansion port pinout"): 

### Input ($4016 write)
    
    
    7  bit  0
    ---- ----
    xxxx xABC
          |||
          ||+-- 1: Ignore bottom row, 0: include bottom row (buttons 9-12)
          |+--- 1: Ignore middle row, 0: include middle row (buttons 5-8)
          +---- 1: Ignore top row, 0: include top row (buttons 1-4)
    

### Output ($4017 read)
    
    
    7  bit  0
    ---- ----
    xxxD EFGx
       | |||
       | ||+-- right-most column (buttons 4,8,12)
       | |+--- right-center column (buttons 3,7,11)
       | +---- left-center column (buttons 2,6,10)
       +------ left-most column (buttons 1,5,9)
    

If a button is pressed in the currently selected rows, then the output returns 0. If no button is pressed, the output for that column is 1. 

The Famicom must manually scan the matrix; additionally it seems that there are quite large parasitic capacitances so you may need to wait a whole millisecond before going on to the next row. 

The Family Trainer mat does have the needed diodes for n-key-rollover. [[1]](https://forums.nesdev.org/viewtopic.php?t=24649)

## Programs

  * [Controller test](https://forums.nesdev.org/viewtopic.php?p=184266#p184266) by Damian Yerrick (Supports NES version)
  * [powerpadgesture](https://forums.nesdev.org/viewtopic.php?t=24192) by Damian Yerrick
  * [powerpd.zip](https://www.nesdev.org/powerpd.zip) by Tennessee Carmel-Veilleux, 1999



## References

  * [powerpad.txt](https://www.nesdev.org/powerpad.txt) \- documentation by Tennessee Carmel-Veilleux, 2000



Categories: [Controllers](Category_Controllers.xhtml)
