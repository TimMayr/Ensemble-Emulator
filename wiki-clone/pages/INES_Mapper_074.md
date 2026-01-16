# INES Mapper 074

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_074) | View [other pages](Special_AllPages.xhtml#INES_Mapper_074)

**iNES Mapper 074** denotes the **43-393** /**43-406** /**860908C** PCB, used by several games from Waixing: 

  * _機甲戰士_ (Jījiǎ Zhànshì, Chinese translation of Data East's _Metal Max_)
  * _甲A - China Soccer League for Division A_
  * _第四次: 机器人大战 - Robot War IV_
  * _風雲 - Traitor Legend_ (original 1997 version)



The circuit board mounts an [MMC3](MMC3.xhtml "MMC3") clone together with a 74LS138 and 74LS139 to redirect 1 KiB CHR-ROM banks #8 and #9 to 2 KiB of CHR-RAM. 

# Notes

  * SMYNES and older versions of VirtuaNES EX/Plus use mapper 74 to denote _any_ MMC3-based mixed CHR-ROM/CHR-RAM game by simply making _all_ of CHR-ROM writable, even if those games actually use [191](INES_Mapper_191.xhtml "INES Mapper 191"), [192](INES_Mapper_192.xhtml "INES Mapper 192"), [194](INES_Mapper_194.xhtml "INES Mapper 194"), or [195](INES_Mapper_195.xhtml "INES Mapper 195"). This approach fails in the case of _甲A - China Soccer League for Division A_ , as the game writes all zeros to whatever CHR-ROM banks are mapped at power-on, overwriting CHR-ROM data in the process. As a result, the publicly available dump of that game (PRG-ROM CRC32 0x78FB3ED6) has been modified to remove those writes.
  * Nestopia emulates all of Waixing's MMC3 clones with a slight variation on the mirroring control register at $A000, additionally treating a written value of 2 or 3 as 1scA and 1scB mirroring respectively.
  * [iNES Mapper 119](INES_Mapper_119.xhtml "INES Mapper 119") is Nintendo's first-party analog. Mappers [252](INES_Mapper_252.xhtml "INES Mapper 252") and [253](INES_Mapper_253.xhtml "INES Mapper 253"), also used for Waixing's localizations, are similar but use a VRC4.



Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3 with CHR ROM and CHR RAM](Category_MMC3_with_CHR_ROM_and_CHR_RAM.xhtml)
