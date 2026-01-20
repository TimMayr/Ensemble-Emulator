# NES 2.0 Mapper 355

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_355) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_355)

**NES 2.0 Mapper 355** denotes the **黃信維 3D-BLOCK** circuit board, used for two unlicensed games: 

  * _3D Block_ (1989 version, copyright "Hwang Shinwei")
  * _Block Force_ (original version, with credits and contact information on the options screen)



Its UNIF board name is **3D-BLOCK** without prefix. 

It is basically NROM-256 with 8 KiB of CHR-RAM rather than CHR-ROM, but with an unusual protection mechanism: a PIC16C54 microcontroller with an embedded internal program ROM watches A4..A14 of the 6502 CPU address bus and raises IRQs based on what it sees. 

To emulate this mapper, the embedded ROM's data (512 words, stored as 1024 bytes of little-endian data), included as a [Misc. ROM](NES_2_0.xhtml#Byte_14_.28Misc._ROMs.29 "NES 2.0") in the [NES 2.0](NES_2_0.xhtml "NES 2.0") specification, must be executed by a [PIC16C54 emulator](https://github.com/mamedev/mame/tree/master/src/devices/cpu/pic16c5x) (running at one fourth of the 6502 clock/M2 switching rate), with the emulated microcontrollers' input/output ports being connected to the cartridge's IRQ and CPU A4..A14 signals in the following way: 
    
    
    PIC16C54 pin#  Meaning
    ----------------------
    RA0 (17)       /IRQ
    RA1 (18)       CPU A6
    RA2 (1)        CPU A5
    RA3 (2)        CPU A4
    RB0 (6)        CPU A12
    RB1 (7)        CPU A7
    RB2 (8)        CPU A10
    RB3 (9)        CPU A11
    RB4 (10)       CPU A9
    RB5 (11)       CPU A8
    RB6 (12)       CPU A13
    RB7 (13)       CPU A14  
    

Note that even though _3D Block_ seems to write specific values to addresses in the $4xxx range, since the CPU D0..D7 signals are not connected to the microcontroller, their values cannot affect IRQ generation, though their addresses could. 

## Errata

  * The status bar in _Block Force_ during gameplay will jump slightly on some CPU/PPU alignments.
  * GoodNES 3.23b contains the correct dump of _3D Block_ as "3D Block (Hwang Shinwei) [!].nes" (sans embedded PIC ROM data) and a Bung Game Master hack as "3D Block (RCM Group) [p1].nes" which on its original floppy disk came with a custom fusemap for the Game Master's FPGA.
  * GoodNES 3.23b contains the correct dump of _Block Force_ as "Square Force (Unl) [!].nes" (sans embedded PIC ROM data) and a multicart extract, without full credits or contact information, that no longer requires the PIC, as "Block Force (Hwang Shinwei).nes", incorrectly set to mapper 219 instead of the correct 0.



## See also

[PCB image and analysis](https://forums.nesdev.org/viewtopic.php?f=3&t=16267#p222579)

Categories: [Mappers triggering on reads](Category_Mappers_triggering_on_reads.xhtml)
