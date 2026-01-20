# NES 2.0 Mapper 471

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_471) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_471)

**NES 2.0 Mapper 471** denotes the **Impact Soft IM1** circuit board, used for _Haratyler_ (without _HG_ or _MP_) and _Haraforce_. It is basically [INES Mapper 201](INES_Mapper_201.xhtml "INES Mapper 201") with the addition of a scanline IRQ. 

# Banks

  * CPU $8000-$FFFF: 32 KiB switchable PRG ROM bank
  * PPU $0000-$1FFF: 8 KiB switchable CHR ROM bank
  * Nametable mirroring: hard-wired, selectable via solder pad



# Address Latch/Acknowledge IRQ ($8000-$FFFF, write)
    
    
    A~[1... .... .... ...B]
                         +- Select PRG A15 and CHR A13
    

Writing to this latch acknowledges a pending IRQ. 

# Scanline IRQ

On every falling edge of PPU A12, an IRQ is raised that, unless interrupts are disabled in the CPU flag register, must be acknowledged by writing to the address latch. No filtering is performed; it is assumed that the interrupt handler will take long enough to execute so that it gets to be executed only once per scanline. 
