# Namco Serial Checker Interface

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Namco_Serial_Checker_Interface) | View [other pages](Special_AllPages.xhtml#Namco_Serial_Checker_Interface)

The **Namco Serial Checker Interface** is a proprietary Famicom [expansion port](Expansion_port.xhtml "Expansion port") device used for some cartridge games developed by Namco. Supported games send and receive data using this interface to check if a cartridge self-test should be performed at power-on/reset. 

As the interface hardware was never released commercially, the information on this page is based on analysis of disassembled code from supported games. 

## Contents

  * 1 Interface
    * 1.1 Input ($4016 write)
    * 1.2 Output ($4017 read)
  * 2 Usage
  * 3 See Also
  * 4 References



## Interface

### Input ($4016 write)
    
    
    7  bit  0
    ---- ----
    xxxx xxxI
            |
            +- Data to interface input
    

### Output ($4017 read)
    
    
    7  bit  0
    ---- ----
    xxxx Dxxx
         |
         |
         |
         +---- Data from interface output
    

The interface appears to consist of a 4-bit shift register (initial contents are undefined). Data sent to the interface will appear in the output after 4 reads. The received data is logically inverted from what was sent due to how the expansion port inputs work. 

## Usage

Games using the interface do the following at some point in their reset handler (the specific code may differ): 

  1. Fetch a source byte.
  2. Initialise a target byte.
  3. Read the data bit from $4017.d3 and shift it into the target byte.
  4. Shift one bit out from the source byte and write it to $4016.d0.
  5. Repeat steps 3-4 11 more times. (i.e. 4 "dummy" bits + 8 data bits total)
  6. Check if the target byte matches the source byte XORed with $FF. Go to step 9 if false.
  7. Return to step 1 and repeat for however many source bytes remain.
  8. If all bytes pass the check, execute a cartridge self-test (partial PRG/CHR checksum verification).
  9. Continue with init tasks.



Note: The source data used is commonly a 32- or 64-byte string consisting of programmer credits, sometimes XORed with $FF. 

## See Also

  * [GitHub Gist](https://gist.github.com/TakuikaNinja/cf2dab7ed4d5869c4be94a1753ffa76f) \- Lua script to simulate the interface in Mesen2.
  * [Namco IPL Interface](Namco_IPL_Interface.xhtml "Namco IPL Interface") \- A similar serial interface used in [FDS](Family_Computer_Disk_System.xhtml "FDS") games developed by Namco.



## References

  * [The Cutting Room Floor](https://tcrf.net/The_Cutting_Room_Floor:Common_Things#Cartridge_Self-Tests)
  * [Forum discussion](https://forums.nesdev.org/viewtopic.php?t=25809)


