# INES Mapper 027

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_027) | View [other pages](Special_AllPages.xhtml#INES_Mapper_027)

**iNES Mapper 027** is believed to be an pirate variant of the [VRC4](VRC2_and_VRC4.xhtml "VRC4") mapper. 

  


GoodNES thinks this is used for _World Hero_. We should discover whether this mapper number is used for any other unlicensed/pirate/multicart VRC2/4 games. 

This is supported by BizHawk as of r3975 and FCEUX for some time prior but definitely as of 26-nov-2012. 

Formerly FCEUX emulated this with unique code, but as of r2722 CaH4e3 asserts that it is a duplicate of [INES Mapper 023](VRC2_and_VRC4.xhtml "INES Mapper 023"). This means the only difference between 27 and the other VRC4 mappers (21, 23, and 25) is the way the board wires up the address lines. However, FCEUX's VRC2/4 emulation is filled with hacks for pirate and multicart games which are otherwise similar to VRC2/4. 

In particular, _World Hero_ depends on the VRC4's 9-bit CHR bank numbers, where the most significant nibble of the CHR banks are actually 5 bits. 

The below notes are left until the author agrees they are describing VRC4: 

  * IRQ behavior: does it match [that of VRC family](VRC_IRQ.xhtml "VRC IRQ") or that of [MMC3](MMC3.xhtml "MMC3")?
  * $9002 behavior: is it 3x8+8F PRG banking or VRC4/MMC3 style PRG banking?



Additionally, Nestopia adds a register at $9080 that is a duplicate of $9002. 
    
    
    Mask = F00F
    
    Writes:
    9002: ......x. = If bit 2 is set, next write to register 8000 affects PRG C000-BFFF instead of PRG 8000-AFFF
    8000: xxxxxxxx = Switches 8k PRG bank 8000-9FFF (or C000-DFFF)
    9000: ......xx = Select mirroring (0 = Vertical, 1 = Horizontal, 2 = Single screen 0, 3 = Single screen 1)
    A000: xxxxxxxx = Switches 8K PRG bank A000-BFFF
    B000-E003: CHR banks.  They appear to have 9 bits.  If address written to is even (least significant bit is zero), sets the low 4 bits of the value.
      If address is odd (least significant bit is one), sets the 5 high bits of the value.
      CHR Bank number affected = Top nibble of address - B, times 2, plus 1 if address & 2.
      (B000 = 4 low bits of bank 0, B001 = 5 high bits of bank 0, B002 = 4 low bits of bank 1, B003 = 5 high bits of bank 1, C000 = 4 low bits of bank 2, etc...)
      Selects the 1K CHR bank.
    F000: 4 low bits of IRQ latch
    F001: 4 high bits of IRQ latch
    F002: ......ER = 2 bit IRQ register, bit E = enable, bit R = enable on next acknowledgement (retrigger), then sets value of IRQ counter to IRQ latch - 1.
    F003: Writing here acknowledges the IRQ, and sets bit E (enable) to bit R (retrigger).
    
    IRQ behavior:
    IRQ counter is triggered by A12, just like MMC3, and happens once per scanline.
    If enabled, IRQ counter counts up to FF.  If it would count up when the counter is at FF, it triggers an interrupt, then resets the IRQ counter to IRQ Latch + 1.
    If disabled, IRQ does not count at all.
    
    PRG E000-FFFF is probably fixed to the last bank.
    

* * *

In FCEUX r2947, Санчез subsequently reallocated this mapper to a set of games that the source refers to has "Mi Hun Che", a very simple (if conflated) mapper: 
    
    
    $8000: [.... ...X] - Select both 4 KiB CHR-ROM bank (mirrored across both PPU $0xxx and $1xxx) and 1scA/1scB mirroring.
    $8001-$FFFF: A~[1... .... .... ...X] - same
    

Categories: [INES Mappers](Category_INES_Mappers.xhtml)
