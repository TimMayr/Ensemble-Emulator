# Namco IPL Interface

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Namco_IPL_Interface) | View [other pages](Special_AllPages.xhtml#Namco_IPL_Interface)

The Namco IPL Interface is a proprietary Famicom [expansion port](Expansion_port.xhtml "Expansion port") device used for an Initial Program Loader (`IPL.MAIN`) present in [Famicom Disk System](Family_Computer_Disk_System.xhtml "Famicom Disk System") games developed by Namco. The IPL and its interface allowed developers to upload PRG-RAM and [ PPU nametable](PPU_nametables.xhtml "PPU nametables") data to the system, with an option to save the data to disk. 

As the interface hardware was never released commercially, the information on this page is based on ongoing analysis of disassembled code from the IPL. 

## Contents

  * 1 Interface
    * 1.1 Input ($4016 write)
    * 1.2 Output ($4017 read)
  * 2 Data transfer format
  * 3 Hardware
  * 4 See Also
  * 5 References



## Interface

### Input ($4016 write)
    
    
    7  bit  0
    ---- ----
    xxxx xxxS
            |
            +- IPL interface strobe
    

The strobe is only ever performed once by the IPL on start-up. 

### Output ($4017 read)
    
    
    7  bit  0
    ---- ----
    xxxD SDDx
       | ||| 
       | ||+-- Data bit
       | |+--- Data bit
       | +---- Status bit
       +------ Data bit
    

The IPL checks for the interface's presence by reading the status bit before and after a single strobe. If the status bit changes (0->1 or 1->0) after the strobe, the IPL initialises a data transfer. Otherwise, the IPL displays a fake [ license screen](FDS_BIOS.xhtml#Approval_check "FDS BIOS") and boots the loaded game instead. The state of the status bit is unknown after successive polls (i.e. while the data bits are being polled). 

It is unknown if all data bits contain identical data. The IPL simply ANDs the $4017 read with `%00010110` and checks if any bits were set in the result. The bit order is `%76543210`. 

## Data transfer format

The interface transfers data to the Famicom using a variant of the [Intel HEX](https://en.wikipedia.org/wiki/Intel_HEX "wikipedia:Intel HEX") format described as the following: 
    
    
    offset	size	description
    ------	----	-----------
    0	1	start code, raw $3A byte (":")
    1	1	ASCII-encoded data length (0: end data transfer)
    2	2	ASCII-encoded destination address (big endian)
    4	1	ASCII-encoded record type (must be $00, data type)
    5	x	ASCII-encoded data
    5+x	1	ASCII-encoded checksum complement
    6+x		-
    

ASCII-encoded bytes are sent by the interface as byte pairs encoding the hexadecimal digits in ASCII (E.g. $5F would be encoded as `'5', 'F' -> $35, $46`), which are then decoded by the IPL. This means that the true sizes for each ASCII-encoded field in the above table are doubled. 

An example record expressed as a list of ASCII strings: 

`":", "02", "6942", "00", "BEEF", "A6"`

If the data length is 0, the IPL reads 10 more raw bytes before exiting. The most straightforward way to format the final record would be to append an extra ASCII-encoded byte to a traditional EOF record, as the record type is not checked in this scenario: 

`":", "00", "0000", "01", "FF", "00"`

  
The destination address determines whether the data is transferred to PRG-RAM or the PPU. If the destination address is below $6000, the IPL adds $2000 to it and sends the data to the PPU (likely intended for [ PPU nametable](PPU_nametables.xhtml "PPU nametables") data). Otherwise, the destination address is used as-is and the data is sent to PRG-RAM instead. 

The checksum operation is performed on the decoded bytes with the carry always cleared, starting from the data length field. The complement must result in a final checksum of 0. 

## Hardware

The interface hardware was likely connected using an RS232-style [ serial cable](Serial_Cable_Construction.xhtml "Serial Cable Construction") with a baud rate of 38400 (~46.60 CPU cycles per bit, the IPL waits 47 cycles). The polling routine in the IPL checks for a stop + start bit (0->1->0 transition), then waits 62 CPU cycles (~1.5 periods) before polling the data bits. Parity bits are not checked by the IPL. 

USB-to-TTL parts such as the PL2303 or FT232 may be able to recreate the interface on modern hardware, using something similar to [ this cable specification](Serial_Cable_Specification.xhtml "Serial Cable Specification"). A key difference here is that the TX signal must be _logically inverted_ (so the console "un-inverts" the incoming signal). 

## See Also

  * [Namco Serial Checker Interface](Namco_Serial_Checker_Interface.xhtml "Namco Serial Checker Interface") \- A similar serial interface used for some cartridge games developed by Namco.
  * [Serial Bootloader](Serial_Bootloader.xhtml "Serial Bootloader") \- A minimal bootloader which also uses serial transfers.
  * [Serial Cable Routines](Serial_Cable_Routines.xhtml "Serial Cable Routines") \- Code examples for serial transfers, targeting a baud rate of 57600.



## References

  * [GitHub repository:](https://github.com/TakuikaNinja/IPL-MAIN) IPL.MAIN documentation and disassembly. Also includes a Lua script to simulate the interface.
  * [GitHub repository:](https://github.com/TakuikaNinja/IPL-demo) IPL-demo program, to be loaded using the IPL.
  * [Forum discussion](https://forums.nesdev.org/viewtopic.php?t=24983)


