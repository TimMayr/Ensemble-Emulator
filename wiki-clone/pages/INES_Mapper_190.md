# INES Mapper 190

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_190) | View [other pages](Special_AllPages.xhtml#INES_Mapper_190)

  
This mapper was assigned by Kevtris for a newly-dumped game from 1992: Magic Kid GooGoo 

Here's a page discussing it: <http://forums.lostlevels.org/viewtopic.php?p=35234>

Mednafen supported it with [0.9.42](http://forum.fobby.net/index.php?t=msg&th=1483)

FCEUX supported it with [r3334](https://sourceforge.net/p/fceultra/code/3334/)

PowerPak Mapper: [forum thread](https://forums.nesdev.org/viewtopic.php?f=9&t=15682)

Here's Kevtris's notes on the mapper: 
    
    
    Magic Kid GooGoo
    ----------------
    
    012517
    Kevtris
    
    
    I have assigned this game mapper 190.  Fortunately the mapper is fairly
    simple, being composed of two 74LS670 4*4 register file chips, a PAL16R4,
    some pullup resistors, two ROMs and an 8K WRAM.
    
    The mapper is fairly straight forward, but kind of weird.  
    
    PRG switching:
    --------------
    
    There's 16 16K PRG ROM banks, but a combination of address and data lines are used
    to select it.
    
    C000-FFFF is set to the *first* bank of ROM at all times (vs. the more common
    last bank of ROM).
    
    8000-BFFF is selectable by writing to either 8000-9FFF or C000-DFFF.
    
    Writing to 8000-9FFF will select bank 0-7 depending on D0-D2.
    Writing to C000-DFFF will select bank 8-15 depending on D0-D2.
    
    So the 4 bank bits are formed like so:
    
    A14, D2, D1, D0  (highest bit to lowest bit)
    
    I don't know why they did it this way but they did.
    
    There is also 8K of RAM mapped in at 6000-7FFF.  This RAM is always enabled and is
    not battery backed.
    
    
    CHR Switching:
    --------------
    
    CHR is composed of 4 2K banks at 0000-07FF, 0800-0FFF, 1000-17FF, 1800-1FFF.
    
    Writing to A000 selects the bank at 0000-07FF in PPU space,
    A001 selects the bank at 0800-0FFF
    A002 selects the bank at 1000-17FF
    A003 selects the bank at 1800-1FFF
    
    Only A0, A1, A13, 14, and 15 are used to select the CHR bank and match the following mask:
    
    101x xxxx xxxx xxbb
    
    b = CHR bank # to change.   
    
    This means that A000, A004, A008, A00C, ... BFF8, BFFC all set the first 2K CHR bank.
    

Rephrasing in a more tabular manner: 
    
    
    A~[1P0. .... .... ....] D~[.... .PPP]
        |                            |||
        +----------------------------+++--- Select 16 KiB PRG bank at $8000-$BFFF
    

PRG bank at $C000-$FFFF fixed to bank 0 
    
    
    A~[101. .... .... ..bb] D~[CCCC CCCC]
                        ||     |||| ||||
                        ||     ++++-++++--- Select 2 KiB CHR bank ...
                        ++----------------- for PPU address starting at bb*$800
    

Categories: [INES Mappers](Category_INES_Mappers.xhtml)
