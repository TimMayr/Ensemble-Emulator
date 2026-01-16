# INES Mapper 195

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_195) | View [other pages](Special_AllPages.xhtml#INES_Mapper_195)

iNES Mapper 195 describes Waixing's **FS303** PCB, which has an MMC3 clone and a GAL that can dynamically change which CHR banks are mapped to RAM. Used by the following games: 

  * _哥伦布传 - 黄金中文版_ : Waixing's Chinese translation of Tomy's _Columbus - Ougon no Yoake_ , original release only (PRG-ROM CRC32 0xE4E842D0)
  * _天使之翼 2_ , _足球小将_ : Waixing's Chinese translation of Tecmo's _Captain Tsubasa Vol. II: Super Strike_
  * _赌神 - Numen Wager_ , _赌王 - Moses Wager_ , _洛克人 X_ : Waixing's Chinese translation of Technos' _Sugoro Quest: Dice no Senshi-tachi_



A compatible circuit board (with unknown PCB code) adds 4 KiB of non-battery-backed PRG-RAM (recognizable as such in the NES 2.0 header) mapped to CPU $5000-$5FFF: 

  * _天神之剑_ : Waixing's Chinese translation of SNK's _God Slayer_ /_Crystalis_.



  
The games write, via the PPU, to specific CHR banks in order to select both which and how many CHR banks are mapped to RAM instead of ROM. Writes to CHR ROM in multiples of 1 KiB: 
    
    
    CHRB~[1Z.D L.L.]
          || | | |
          |+-|-+-+---------------- Select first bank and size of CHR RAM:
          || |                      $80 = $28-$2B
          || |                      $82 = $00-$03
          || |                      $88 = $4C-$4F
          || |                      $8A = $64-$67
          || |                      $C0 = $46-$47
          || |                      $C2 = $7C-$7D
          || |                      $C8 = $0A-$0B
          || |                      $CA = only CHR ROM
          || +-------------------- If 1, ignore above and always enable CHR ROM / disable CHR RAM
          |+---------------------- Number of banks of CHR RAM, 0=4KiB, 1=2KiB
          +----------------------- Must be 1 
    Power-on value: $80
    

The PCB mounts 32 KiB of CHR-RAM, although address lines for only 8 KiB are connected. Since CHR A10-A12 are connected to CHR-RAM just as they are connected to CHR-ROM, setting $80 ($28-$2B) will select different 4 KiB of CHR-RAM from setting $88 ($4C-$4F), but the same 4 KiB as setting $82 ($00-$03), because CHR A13 is not connected to CHR-RAM. 

Notes: 

  * The more common ROM image of the Chinese translation of _Columbus_ (PRG-ROM CRC32 0xBFF2E9EC) is actually from a 2006 re-release of the game that uses [INES Mapper 176](INES_Mapper_176.xhtml "INES Mapper 176") instead, recognizable by the values it writes to $A001.
  * The common ROM images of the Chinese translation of _Captain Tsubasa II_ have been hacked to work with a fixed setting of $82 ($00-$03) and can be identified by having the bytes at 60 EA EA EA at .NES file offset $41CC.
  * The game _風雲 - Traitor Legend_ is commonly set to mapper 195. The ROM images to which this applies (PRG-ROM CRC32 0xA9B36A7D/0xC7E3F93A) are actually from 2005/2006 re-releases of the game that use [INES Mapper 176](INES_Mapper_176.xhtml "INES Mapper 176"), recognizable by the values they write to $A001. The original 1997 release of the game used [INES Mapper 074](INES_Mapper_074.xhtml "INES Mapper 074") instead.



Mappers [252](INES_Mapper_252.xhtml "INES Mapper 252") and [253](INES_Mapper_253.xhtml "INES Mapper 253"), also used for Waixing's localizations, are similar but use a VRC4. 

See also: [NewRisingSun's findings](https://forums.nesdev.org/viewtopic.php?p=240335#p240335)

Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3 with CHR ROM and CHR RAM](Category_MMC3_with_CHR_ROM_and_CHR_RAM.xhtml)
