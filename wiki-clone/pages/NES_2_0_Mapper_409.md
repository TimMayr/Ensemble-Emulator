# NES 2.0 Mapper 409

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_409) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_409)

**NES 2.0 Mapper 409** denotes the **retroUSB DPCMcart** circuit board, used for the [homebrew music cartridge "A Winner is You"](https://sergioelisondo.bandcamp.com/album/a-winner-is-you) released by [retroUSB.](https://retrousb.com/product_info.php?cPath=36&products_id=143). It is basically an extremely oversized version of [UNROM](UxROM.xhtml "UNROM") that latches twelve bits of the address, rather than the data, bus for selecting banks from a 64 MiB PRG address space. 

Despite being called "DPCMcart", it cannot actually bank DPCM samples because of its fixed bank. Instead, _A Winner is You_ uses the large ROM for PCM sample storage, played back with timed CPU code. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 Address Latch ($C000-$CFFF, write)
  * 3 See also



# Banks

  * CPU $8000-$BFFF: 16 KiB PRG bank, switchable
  * CPU $C000-$FFFF: 16 KiB PRG bank, fixed to last bank
  * PPU $0000-$1FFF. 8 KiB CHR RAM, unbanked
  * Mirroring: Hard-wired



# Registers

## Address Latch ($C000-$CFFF, write)
    
    
    Register Mask: $8000 ?
       Value Mask: $0FFF
    
    
    
    A~FEDC BA98 7654 3210
      -------------------
      1... BBBB BBBB BBBB
           ++++-++++-++++- Selects bank via PRG A25..A14 at CPU $8000-BFFF (CPU A14=0)
    

This register may respond to all of $8000-FFFF, but it is unknown whether it does. _A Winner is You_ only uses the $C000-CFFF range. 

# See also

  * <https://forums.nesdev.org/viewtopic.php?t=15188>
  * [https://web.archive.org/web/20191024003230/nintendoage.com/forum/messageview.cfm?catid=34&threadid=160050](https://web.archive.org/web/20191024003230/nintendoage.com/forum/messageview.cfm?catid=34&threadid=160050)



Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml)
