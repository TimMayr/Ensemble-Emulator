# Family Computer Disk System

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Family_Computer_Disk_System) | View [other pages](Special_AllPages.xhtml#Family_Computer_Disk_System)

[![](../wiki-images/Nintendo-Famicom-Disk-System.jpg)](File_Nintendo_Famicom_Disk_System_jpg.xhtml)

[](File_Nintendo_Famicom_Disk_System_jpg.xhtml "Enlarge")

Famicom, FDS RAM adapter, and FDS disk drive

**FDS**

**Company** | Nintendo   
---|---  
**Complexity** | ASIC   
**Pinout** | [RP2C33 pinout](RP2C33_pinout.xhtml "RP2C33 pinout")  
**BIOS PRG ROM size** | 8K   
**PRG RAM capacity** | 32K   
**CHR capacity** | 8K   
**Disk capacity** | ~64K per side   
**Mirroring** | H or V, switchable   
**Bus conflicts** | No   
**IRQ** | Yes   
**Audio** | Yes   
  
The **Family Computer Disk System** (**Famicom Disk System** , **FDS** for short) is a video game add-on made by Nintendo for the Famicom and sold in Japan and Hong Kong. It was designed to reduce the cost of making copies of software by switching from mask [ROM](ROM.xhtml "ROM") chips to Nintendo's proprietary Disk Card, a storage medium based on Mitsumi's Quick Disk (QD). 

Unfortunately for Nintendo, it also reduced the pirates' cost of making copies of software. Software is stored on one or multiple disk sides. The [FDS BIOS](FDS_BIOS.xhtml "FDS BIOS") is used to load data from disks to PRG RAM or VRAM, and software can execute from there. Additional hardware features include a timer [IRQ](IRQ.xhtml "IRQ") and a [wavetable channel](FDS_audio.xhtml "FDS audio"). 

## Contents

  * 1 Hardware
    * 1.1 Protections
    * 1.2 Disks
  * 2 Banks
  * 3 Registers
    * 3.1 Timer IRQ reload value low ($4020)
    * 3.2 Timer IRQ reload value high ($4021)
    * 3.3 Timer IRQ control ($4022)
    * 3.4 Master I/O enable ($4023)
    * 3.5 Write data register ($4024)
    * 3.6 FDS Control ($4025)
    * 3.7 External connector ($4026)
    * 3.8 Disk Status register ($4030)
    * 3.9 Read data register ($4031)
    * 3.10 Disk drive status register ($4032)
    * 3.11 External connector read ($4033)
    * 3.12 Sound ($4040-$4092)
  * 4 BIOS
  * 5 See Also
  * 6 References



## Hardware

The FDS comes in two parts: The disk drive (HVC-022) and the RAM adapter (HVC-023). 

The RAM adapter is a special shaped cartridge that contains the RAM chips and an ASIC with DRAM controller, IRQ hardware, sound generation hardware, serial interface for the disk drive, and parallel port. The disk drive has to be powered separately and is only connected to the console via a [serial cable](FDS_RAM_adaptor_cable_pinout.xhtml "FDS RAM adaptor cable pinout") to the RAM adapter. 

The disk drive is a modified version of the Mistumi Quick Disk (QD) drive. Conventional floppy disk drives contain two motors: a spindle motor that spins the disk at a specific speed, and a stepper motor which moves the read/write head between each circular data track. By comparison, QD drives only contain a single motor which does both at once, so it instead stores the data in a single spiral-shaped track. There is a mechanism that detects when the head reaches the end of the disk and makes it return to the start (making an audible click). Because of this limitation, random access to the disk is impossible, making QD drive data access behave similarly to a reel of tape (but _much_ faster). Data can only be accessed by spinning the disk, waiting for the head to reach the inner edge, then waiting again until the desired data file is reached. A complete cycle through an entire disk side takes about 7 seconds. 

The QD drive only contains basic electronics, there is no "intelligence" in it; therefore, the serial interface almost directly represents what is stored on the disk. 

### Protections

Some protections were added to the FDS drive in an attempt to thwart unlicensed software (primarily disk copiers): 

  * The FDS disk drive contains a physical lockout in the form of molds which only fit disks with recesses spelling "NINTENDO", in order to prevent regular QDs from being used (also a similar trademark protection as the [license screen](FDS_BIOS.xhtml#Approval_check "FDS BIOS")). This can be bypassed by using disks with custom recesses, including adapters which can be fitted onto regular QDs.
  * Later revisions of the drive controller power board/IC contain protections against long data write operations performed by disk copiers. This can be bypassed through hardware modifications. 
    * TODO: Measure/determine how much data can be written on protected drives.



### Disks

The FDS Disk Card (HVC-021) is a modified version of the Mitsumi Quick Disk. The recesses spelling "NINTENDO" is designed to fit with molds on the FDS disk drive (see protections above). 

See: 

  * [FDS disk format](FDS_disk_format.xhtml "FDS disk format") \- the disk data format and file structure
  * [FDS file format](FDS_file_format.xhtml "FDS file format") (.FDS) - an archival file format for storing and emulating FDS disks



## Banks

All Banks are fixed 

  * PPU $0000-$1FFF: 8k CHR RAM
  * CPU $6000-$DFFF: 32k PRG RAM
  * CPU $E000-$FFFF: 8k BIOS PRG ROM



## Registers

$402x registers are write-only, $403x registers are read-only 

### Timer IRQ reload value low ($4020)
    
    
    7  bit  0
    ---------
    LLLL LLLL
    |||| ||||
    ++++-++++- 8 LSB of timer IRQ reload value
    

### Timer IRQ reload value high ($4021)
    
    
    7  bit  0
    ---------
    LLLL LLLL
    |||| ||||
    ++++-++++- 8 MSB of timer IRQ reload value
    

Unlike $4022, $4020 and $4021 are not affected by the $4023.0 (disk registers enabled) flag - the reload value can be altered even when disk registers are disabled. 

### Timer IRQ control ($4022)
    
    
    7  bit  0
    ---------
    xxxx xxER
           ||
           |-- Timer IRQ Repeat Flag
           +-- Timer IRQ Enabled
    

When $4022 is written to with bit 1 (IRQ enabled) set, the reload value is copied into the IRQ's counter. Each CPU clock cycle the counter is decremented by one if the enable flag is set. 

When the counter's value is 0 and the IRQ enable flag is on, the following happens on every CPU cycle: 

  * An IRQ is generated.
  * The IRQ counter is reset to its reload value (contained in $4020+$4021)
  * If the IRQ repeat flag is NOT set, the IRQ enabled flag is cleared and the counter stops.



Notes: 

  * This register is affected by the $4023.0 (Enable disk I/O registers) flag - if disk registers are disabled, it is impossible to start the IRQ counter (writing to $4022 has no effect).
  * Clearing $4023.0 will immediately stop the IRQ counter and acknowledge any pending timer IRQs.
  * Writing to $4022 with bit 1 (IRQ enabled) cleared will stop the IRQ counter and acknowledge any pending timer IRQs.
  * Enabling timer IRQs when the reload value is set to 0 will cause an IRQ immediately. Doing this with the repeat flag enabled will cause an infinite loop of IRQs on every CPU cycle.
  * Since the disk transfer routine also uses IRQs, it's very important to disable timer IRQs before doing any access to the disk.



There are only 3 known ways to acknowledge the timer IRQ: 

  * Read $4030
  * Disable timer IRQs by writing to $4022
  * Disable disk registers by writing to $4023



### Master I/O enable ($4023)
    
    
    7  bit  0
    ---------
    xxxx xxSD
           ||
           |+- Enable disk I/O registers
           +-- Enable sound I/O registers
    

The FDS BIOS writes $00, then $83 to it during reset. The purpose of bit 7 is unknown. 

Disabling disk registers disables both disk and timer IRQs. 

### Write data register ($4024)

The data that this register is programmed with will be the next 8-bit quantity to load into the shift register (next time the byte transfer flag raises), and to be shifted out and appear on pin 5 of the RAM adapter cable (2C33 pin 52). 

Writing to this register acknowledges disk IRQs.[**_citation needed_**]

### FDS Control ($4025)
    
    
    7  bit  0
    ---------
    IE1C MRDT
    |||| ||||
    |||| |||+- Transfer Reset
    |||| |||     1: Reset transfer timing to the initial state.
    |||| ||+-- Drive Motor Control (0: start, 1: stop)
    |||| |+--- Transfer Mode (0: write; 1: read)
    |||| +---- Nametable Arrangement
    ||||         0: Horizontal ("Vertical Mirroring")
    ||||         1: Vertical ("Horizontal Mirroring")
    |||+------ CRC Transfer Control (1: transfer CRC value)
    ||+------- Unknown, always set to '1'
    |+-------- CRC Enabled (0: disable/reset, 1: enable)
    +--------- Interrupt Enabled
                 1: Generate an IRQ every time the byte transfer flag is raised.
    

Notes: 

  * The BIOS and commercial software are known to always write 1 to bit 5. Writing 0 to it alters bits 1 and 2 of the external connector ($4026/$4033) but its purpose is currently unknown. (TODO: this supposedly also affects byte transfer IRQs?)
  * CRC values are [stored on the disk](FDS_disk_format.xhtml#CRCs "FDS disk format") and verified when reading/writing a given file.
  * Disabling the CRC resets its state. The FDS BIOS disables the CRC between file blocks, then enables it before accessing each file block to calculate/verify their CRC values.
  * To change the nametable arrangement on the fly, a read-modify-write of its [mirror variable](FDS_BIOS.xhtml#Zero-page_variables "FDS BIOS") should be done to prevent altering unrelated bits.



Writing to this register acknowledges disk IRQ. 

### External connector ($4026)

Output of [expansion terminal](FDS_expansion_port_pinout.xhtml "FDS expansion port pinout") where there's a shutter on the back of the RAM adapter. The outputs of $4026 (open-collector with 4.7K ohm pull-ups (except on bit 7)), are shared with the inputs on $4033. 

TODO: Setting $4025.D5 to 0 alters bits 1 and 2 of the output, which are also reflected in $4033. Its purpose is currently unknown. 

### Disk Status register ($4030)
    
    
    7  bit  0
    ---------
    IExB MxTD
    || | | ||
    || | | |+- Timer Interrupt (1: an IRQ occurred)
    || | | +-- Byte transfer flag. Set every time 8 bits have been transferred between the RAM adaptor & disk drive (service $4024/$4031). 
    || | |     Reset when $4024, $4031, or $4030 has been serviced.
    || | +---- Nametable Arrangement (from $4025.D3)
    || +------ CRC control (0: CRC passed; 1: CRC error)
    |+-------- End of Head (1 when disk head is on the most inner track)
    +--------- Disk Data Read/Write Enable (1 when disk is readable/writeable)
    

Reading this register acknowledges timer and disk IRQs. 

### Read data register ($4031)

This register is loaded with the contents of an internal shift register every time the byte transfer flag raises. The shift register receives its serial data via pin 9 of the RAM adapter cable (2C33 pin 51). 

Reading this register acknowledges disk IRQs. 

### Disk drive status register ($4032)
    
    
    7  bit  0
    ---------
    xxxx xPRS
          |||
          ||+- Disk flag  (0: Disk inserted; 1: Disk not inserted)
          |+-- Ready flag (0: Disk readу; 1: Disk not ready)
          +--- Protect flag (0: Not write protected; 1: Write protected or disk ejected)
    

Notes: 

  * The Ready flag corresponds to the drive head's position. It is set to 1 when the head reaches the end of the disk, and cleared once it returns to the beginning of the disk.
  * The Protect flag corresponds to the write protect tab present on the upper-left corner of the inserted disk side. It is set to 1 if the tab is broken.



Reading this register acknowledges disk IRQs.[**_citation needed_**]

### External connector read ($4033)
    
    
    7  bit  0
    ---------
    BIII IIII
    |||| ||||
    |+++-++++- Input from [expansion terminal](FDS_expansion_port_pinout.xhtml "FDS expansion port pinout") where there's a shutter on the back of the RAM adapter.
    +--------- Battery status (0: Voltage is low; 1: Good).
    

When a bit is clear in $4026 port it will read back as '0' here (including battery bit) because of how open collector input works. Battery bit should be checked when the motor is on, otherwise it always will be read as 0. 

### Sound ($4040-$4092)

For details on sound information, see [FDS audio](FDS_audio.xhtml "FDS audio"). 

## BIOS

The FDS contains a fixed 8KB BIOS at $E000-FFFF. This controls the Famicom at power-on and reset, dispatches the NMI and IRQ, and offers an API for accessing the [data on disk](FDS_disk_format.xhtml "FDS disk format"). Routines for common tasks including controller reading and PPU handling are also provided for programmer convenience. 

See: [FDS BIOS](FDS_BIOS.xhtml "FDS BIOS")

## See Also

  * [FDS BIOS](FDS_BIOS.xhtml "FDS BIOS")
  * [FDS disk format](FDS_disk_format.xhtml "FDS disk format")
  * [FDS file format](FDS_file_format.xhtml "FDS file format") (**.FDS**)
  * [FDS audio](FDS_audio.xhtml "FDS audio")
  * [FDS RAM adaptor cable pinout](FDS_RAM_adaptor_cable_pinout.xhtml "FDS RAM adaptor cable pinout")
  * [FDS expansion port pinout](FDS_expansion_port_pinout.xhtml "FDS expansion port pinout")
  * [RP2C33 pinout](RP2C33_pinout.xhtml "RP2C33 pinout")
  * [FdsIrqTests(v7)](https://forums.nesdev.org/download/file.php?id=10240) by Sour - Test program for various elements of the FDS' IRQ
  * [FDS-Mirroring-Test](https://forums.nesdev.org/download/file.php?id=28216) by TakuikaNinja - Test program for the FDS' nametable arrangement/mirroring switching functionality
  * [iNES mapper 20](INES_Mapper_020.xhtml "INES Mapper 020") \- Reserved for FDS dumps, but not widely used for it.
  * [TNES](TNES.xhtml "TNES") \- Nintendo 3DS Virtual Console ROM format with support for FDS disk images.
  * [GitHub repository:](https://github.com/bbbradsmith/NES-ca65-example/tree/fds) Simple FDS example for ca65
  * [GitHub repository:](https://github.com/TakuikaNinja/asm6f-fds-example) Simple FDS example for asm6f
  * [Forum post:](https://forums.nesdev.org/viewtopic.php?p=194826#p194826) Skipping the FDS license screen
  * [FDS List](https://www.chrismcovell.com/software.html) by ccovell - command line utility to inspect FDS disk image contents.
  * [FDS Lister](https://www.chrismcovell.com/fds-lister.html) by ccovell - utility to inspect FDS disk contents that runs on an FDS.
  * [Triton Quick Disk Drive](https://en.wikipedia.org/wiki/Triton_Quick_Disk_Drive "wikipedia:Triton Quick Disk Drive") \- a QD drive peripheral which was sold for various home computers.



## References

  * <https://www.nintendo.com/jp/famicom/hardware/disksystem.html> (Japanese)
  * [FDS technical reference.txt](https://nesdev.org/FDS%20technical%20reference.txt) by Brad Taylor (old/outdated)
  * [Enri's Famicom Disk System page](http://cmpslv3.stars.ne.jp/Famic/Famdis.htm) (Japanese)
  * [Enri's Famicom Disk System page](https://web.archive.org/web/20091023182159/http://www2.odn.ne.jp/~haf09260/Famic/Famdis.htm) (Japanese) (old/outdated)
  * [fds-nori.txt](https://nesdev.org/fds-nori.txt) \- FDS reference in Japanese by Nori (old/outdated)
  * [Forum post](https://forums.nesdev.org/viewtopic.php?p=194867#p194867): .fds format: Can checksums be heuristically detected? - Includes a CRC implementation in C.
  * [Forum post](https://forums.nesdev.org/viewtopic.php?f=3&t=16507): FDS IRQ reload flag/value


  * <https://www.famicomdisksystem.com/disks/>
  * <https://famicomworld.com/workshop/tech/famicom-disk-system-fd3206-write-mod/>
  * <https://famicomworld.com/workshop/tech/fds-power-board-modifications/>



Categories: [Mappers using $4020-$5FFF](Category_Mappers_using__4020__5FFF.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml)
