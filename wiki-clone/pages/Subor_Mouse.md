# Subor Mouse

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Subor_Mouse) | View [other pages](Special_AllPages.xhtml#Subor_Mouse)

A mouse came with Subor's SB2000 famiclone system. 

Example games: 

  * _Educational Computer 2000_



## Contents

  * 1 Input ($4016 write)
  * 2 Output ($4017 read)
    * 2.1 1-byte response format
    * 2.2 3-byte response format
  * 3 References



### Input ($4016 write)
    
    
    7  bit  0
    ---- ----
    xxxx xxxS
            |
            +- Strobe
    

### Output ($4017 read)
    
    
    7  bit  0
    ---- ----
    xxxx xxxD
            |
            +- Serial data
    

The mouse returns either 1-byte or 3-byte responses based on the size of the x/y axis movement since the last read. If the movement value for both the X and Y axis is between -1 and 1 (inclusively), the mouse returns a 1-byte response. Otherwise, a 3-byte response is sent. In this case, the strobe bit must be toggled 3 times in a row to read the entire response (e.g read $4017 8 times, toggle strobe bit, read 8 more times, etc.) 

#### 1-byte response format
    
    
    LRXXYY00
    ||||||||
    ||||||++- Always 0
    ||||++--- Y movement (0: no movement, 1 or 2 = went down 1 unit, 3 = went up 1 unit)
    ||++----- X movement (0: no movement, 1 or 2 = went right 1 unit, 3 = went left 1 unit)
    |+------- Right mouse button (1: pressed)
    +-------- Left mouse button (1: pressed)
    

The mouse button order is opposite that of the [Super NES Mouse](Super_NES_Mouse.xhtml "Super NES Mouse"). 

#### 3-byte response format

**Byte 1**
    
    
    LRSXTY01
    ||||||||
    ||||||++- Always 01
    |||||+--- Y movement (Bit 4)
    ||||+---- Y movement direction (1: up, 0: down)
    |||+----- X movement (Bit 4)
    ||+------ X movement direction (1: left, 0: right)
    |+------- Right mouse button (1: pressed)
    +-------- Left mouse button (1: pressed)
    

**Byte 2**
    
    
    --XXXX10
      ||||||
      ||||++- Always 10
      ++++--- X movement (Bits 0-3)
    

**Byte 3**
    
    
    --YYYY11
      ||||||
      ||||++- Always 11
      ++++--- Y movement (Bits 0-3)
    

## References

  * [Nocash's EveryNES documentation](http://problemkaputt.de/everynes.htm#controllerstrackballandmouse)



Categories: [Controllers](Category_Controllers.xhtml), [Pointing devices](Category_Pointing_devices.xhtml)
