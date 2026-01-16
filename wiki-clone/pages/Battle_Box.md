# Battle Box

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Battle_Box) | View [other pages](Special_AllPages.xhtml#Battle_Box)

The Battle Box is a Famicom [expansion port](Expansion_port.xhtml "Expansion port") device holding 512 bytes of EEPROM. 

The TC89102 EEPROMs used are similar to standard microwire EEPROMs, except that the address is entered before the command. Each are configured for 7 bits of address and 16 bits of data. 

## Contents

  * 1 Hardware interface
    * 1.1 Input
      * 1.1.1 ($4017 Read)
      * 1.1.2 ($4016 Write)
    * 1.2 Output
      * 1.2.1 ($4017 Read)
  * 2 See also



## Hardware interface

### Input

#### ($4017 Read)
    
    
    7  bit  0
    ---- ----
    xxxW Rxxx
       | |
       | +---- see below
       +------ Data line to the two EEPROMs
    

The data line toggles after each time this is read, so the actual value is the opposite of what was last read. You cannot directly set the data output; you must read this port until it's the opposite of what you want. 

#### ($4016 Write)
    
    
    7  bit  0
    ---- ----
    xxxx xxxS
            |
            +- Clock line to the two TC89102 EEPROMs
    

If "S" is high during a read from $4017, it toggles which of two EEPROMs are enabled. 

### Output

#### ($4017 Read)
    
    
    7  bit  0
    ---- ----
    xxxW Rxxx
       | |
       | +---- bitwise NOT of the value read from the EEPROM
       +------ see above
    

## See also

[Krzysiobal's reverse-engineered schematic and the TC89102 datasheet](https://forums.nesdev.org/viewtopic.php?t=22685)
