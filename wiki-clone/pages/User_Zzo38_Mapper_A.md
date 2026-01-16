# User:Zzo38/Mapper A

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AZzo38/Mapper_A) | View [other pages](Special_AllPages.xhtml#User_Zzo38_Mapper_A)

This is the "AY-3-8910 only" mapper. Registers are mapped to the PPU address space $3000-$3FFF, and only the address is used; the data isn't used for writing the mapper registers. This can be accessed by writing $2006. 

## Submappers

Bit0 specifies the mirroring mode. If clear, uses hardwired H or V mirroring. If set, it specifies software-controlled one-screen mirroring; the mirroring bit in the iNES header is then used to specify which bit of which register is used to control it (H=bit7 of port A, V=bit7 of port B). 

The other bits are used to select the output levels and filters of each channel output; currently only zero is defined, meaning that all three channels are mixed with the 2A03 and microphone audio at equal volume. 

## Registers

Registers are write-only and are not accessible during rendering. To write to a register: Set the PPU address to $300x or $330x (bit10 and bit11 are ignored; the registers are mirrored), where x is the register number. Set the PPU address to $32xx, where x is the value to write. Something like this may work (untested): 
    
    
      ; Write A to port Y
      ldx #$33
      stx PPUADDR
      sty PPUADDR
      dex
      stx PPUADDR
      sta PPUADDR
    

These are the registers of the AY-3-8910 when the BDIR pin is fixed high. 

The bankswitching is done using the two I/O ports of the AY-3-8910. The mapper will not work properly unless both ports are in output mode, so after reset this should be done at first. To keep the high address lines from floating while the I/O ports are still in input mode, PRG bank lines should have pull-up resistors. An emulator should assume these resistors are present, even though some cartridges might not. Still, all 32K PRG ROM banks should contain at least some reset code because the player can press Reset at any time. 

  * 32K PRG ROM banks are selected by writing the bank number to the low bits of port A.
  * 8K PRG RAM banks are mapped to $6000-$7FFF and are selected by writing the bank number to the high bits of port A.
  * 8K CHR ROM or CHR RAM banks are selected by writing the bank number to the low bits of port B.
  * If both CHR ROM and CHR RAM is present, and four-screen mirroring is specified in the iNES header, then there is 8K RAM split at $0000-$0FFF and $2000-$2FFF, and 8K ROM split at $1000-$1FFF and $3000-$3EFF. The RAM bank is then selected by the high bits of port B.
  * If both CHR ROM and CHR RAM is present, and four-screen mirroring isn't specified in the iNES header, then the high bit of port B selects between ROM and RAM in $0000-$1FFF.



Note: Normally there is PRG ROM and CHR ROM, but no PRG RAM and CHR RAM. Nevertheless, use of RAM is well-defined and with some extra logic it can be added to the board (if there is no RAM, then all the decoding logic is either part of the AY-3-8910 or it is already decoded in the cartridge connector; therefore a 60-pins cartridge will have three chips and a 72-pins cartridge will have four chips). 
