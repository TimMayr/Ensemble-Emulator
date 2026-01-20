# INES Mapper 055

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_055) | View [other pages](Special_AllPages.xhtml#INES_Mapper_055)

[![Geniusmali-fc-pcbf.jpg](../wiki-images/Geniusmali-fc-pcbf.jpg)](File_Geniusmali_fc_pcbf_jpg.xhtml)

[](File_Geniusmali_fc_pcbf_jpg.xhtml "Enlarge")

[![Geniusmali-fc-pcbb.jpg](../wiki-images/Geniusmali-fc-pcbb.jpg)](File_Geniusmali_fc_pcbb_jpg.xhtml)

[](File_Geniusmali_fc_pcbb_jpg.xhtml "Enlarge")

iNES Mapper 055 is used for the UNIF board **BTL-MARIO1-MALEE2** , used by several bootleg versions of the first _Super Mario Brothers_ that add a great number of cheats: 

  * _Fly Merio Bros._
  * _Super Mario Bros. Malee 2_
  * _天才瑪琍_ (_Tiāncái Mǎlí_ , a.k.a. _Genius Mario_)



It adds 2 KiB of PRG-ROM and PRG-RAM each to a regular NROM-256 PCB, but has no bankswitching functionalities. 

# Banks

  * CPU $6000-$6FFF: 2 KiB PRG-ROM, mirrored once
  * CPU $7000-$7FFF: 2 KiB PRG-RAM, mirrored once
  * CPU $8000-$FFFF: 32 KiB PRG-ROM
  * PPU $0000-$1FFF: 8 KiB CHR-ROM



# ROM layout

Nestopia expects Mapper 055 ROM images to first contain the 32 KiB of PRG-ROM at $8000-$FFFF, then the data for the 2 KiB of PRG-ROM at $6000, appropriately padded or repeated to fit into the 16 KiB iNES bank size, for a total of three 16 KiB PRG-ROM banks. 

Categories: [INES Mappers](Category_INES_Mappers.xhtml)
