# INES Mapper 228

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_228) | View [other pages](Special_AllPages.xhtml#INES_Mapper_228)

  
**iNES Mapper 228** represents the board used by Active Enterprises for _Action 52_ and _Cheetahmen II_. 

## Registers
    
    
    Address           Data
    FEDCBA98 76543210 76543210
    1.MHHPPP PPS.CCCC ......CC
      |||||| ||| ||||       ||
      |||||| ||| ++++-------++- Select 8 KiB CHR ROM bank at PPU $0000
      |||||| ||+--------------- PRG bank size
      |||||| ||                 0: Put bank with bit 0 false in $8000 and
      |||||| ||                    bank with bit 0 true in $C000
      |||||| ||                 1: Put same 16 KiB bank in $8000 and $C000
      |||+++-++---------------- Select 16 KiB PRG ROM bank at CPU $8000
      |++---------------------- Select which 512 KiB PRG ROM chip to enable
      |                         (A52 uses 0, 1, and 3; bank 2 is open bus)
      +------------------------ 0: Vertical mirroring; 1: Horizontal mirroring
    

_Action 52_ is highly uncommon in that its PRG ROM has a [non-power-of-two ROM size](Non_power_of_two_ROM_size.xhtml "Non-power-of-two ROM size"): three 512 KiB PRG ROMs alongside one 512 KiB CHR ROM. 

It is claimed that there are four 4-bit RAM locations at $4020-$4023, mirrored throughout $4020-$5FFF. This 16-bit RAM is definitely not present on either cartridge, Nestopia does not implement it at all, and neither cartridge ever writes to these addresses. 

Incidentally, this is very similar to [iNES Mapper 225](INES_Mapper_225.xhtml "INES Mapper 225") with a few differences: 

  * Two of the CHR bank bits have been shifted to the data bus.
  * PRG bank size bit moved to other side of PRG bank number.
  * High bit moved to other side of mirroring and applies only to PRG ROM, not CHR ROM.
  * The rumored 4x4-bit RAM is at a different address.


    
    
     Here are Disch's original notes:
     ========================
     =  Mapper 228          =
     ========================
     
     
     Example Games:
     --------------------------
     Action 52
     Cheetah Men II
     
     
     Notes:
     ---------------------------
     Cheetah Men II is infamous for how freaking terrible it is.  Action 52 is none better.  These games are SO
     bad, it's hilarious.
     
     Action 52's PRG size is weird (not a power of 2 value).  This is because there are 3 seperate 512k PRG chips.
     PRG Setup section will cover details.
     
     
     Powerup and Reset:
     ---------------------------
     Apparently the games expect $00 to be written to $8000 on powerup/reset.
     
     
     Registers:
     ---------------------------
     
       $4020-4023:  [.... RRRR]  RAM  (readable/writable)
                     (16 bits of RAM -- 4 bits in each of the 4 regs)
       $4024-5FFF:    mirrors $4020-4023
     
       $8000-FFFF:    [.... ..CC]   Low 2 bits of CHR
                    A~[..MH HPPP PPO. CCCC]
     
         M = Mirroring (0=Vert, 1=Horz)
         H = PRG Chip Select
         P = PRG Page Select
         O = PRG Mode
         C = High 4 bits of CHR
     
     CHR Setup:
     ---------------------------
     
           $0000   $0400   $0800   $0C00   $1000   $1400   $1800   $1C00 
         +---------------------------------------------------------------+
         |                             $8000                             |
         +---------------------------------------------------------------+
     
     
     PRG Setup:
     ---------------------------
     
     'H' bits select the PRG chip.  Each chip is 512k in size.  Chip 2 does not exist, and when selected, will
     result in open bus.  The Action 52 .nes ROM file contains chips 0, 1, and 3:
     
     chip 0:  offset 0x000010
     chip 1:  offset 0x080010
     chip 2:  -- non existant --
     chip 3:  offset 0x100010
     
     'P' selects the PRG page on the currently selected chip.
     
                    $8000   $A000   $C000   $E000  
                  +-------------------------------+
     PRG Mode 0:  |            <$8000>            |
                  +-------------------------------+
     PRG Mode 1:  |     $8000     |     $8000     |
                  +---------------+---------------+
    

Categories: [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
