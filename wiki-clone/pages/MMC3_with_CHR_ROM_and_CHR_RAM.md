# MMC3 with CHR ROM and CHR RAM

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/MMC3_with_CHR_ROM_and_CHR_RAM) | View [other pages](Special_AllPages.xhtml#MMC3_with_CHR_ROM_and_CHR_RAM)

**MMC3 with CHR ROM and CHR RAM** is an uncommon combination. 

Apart from _Pinbot_ and _High Speed_ , which use the [TQROM](INES_Mapper_119.xhtml "TQROM") board, this combination is used for games in the Chinese language using an [MMC3](MMC3.xhtml "MMC3")-clone mapper, which need to mix [logographic characters](https://en.wikipedia.org/wiki/Chinese_characters "wikipedia:Chinese characters") arbitrarily. They are thought to follow a similar strategy to _Faxanadu_ and _[Super Bat Puncher](User_Miau_Super_Bat_Puncher.xhtml "User:Miau/Super Bat Puncher")_ , reserving some CHR RAM for software-rendered text boxes. No mapper other than [MMC5](MMC5.xhtml "MMC5") has CHR ROM banks fine-grained enough to allow showing dialogue using Chinese characters with only CHR ROM without prerendering each sentence, and unlike Japanese, Chinese cannot use the common shortcut of using only [phonetic characters](https://en.wikipedia.org/wiki/kana "wikipedia:kana"). Some of these mappers differ only in the amount of CHR RAM that replaces CHR ROM. 

All MMC3-like iNES mappers known to be used with CHR RAM are listed below: 

Mapper | First RAM bank | Size of RAM  
(1024-byte banks) | RAM mirrored?   
---|---|---|---  
[004](MMC3.xhtml "INES Mapper 004") | 0 | 0 or 8* | Yes   
[074](INES_Mapper_074.xhtml "INES Mapper 074") | 8 | 2 | No   
[119](INES_Mapper_119.xhtml "INES Mapper 119") | 64, 192 | 8 | Yes   
[176](INES_Mapper_176.xhtml "INES Mapper 176") | 0 | 8 | No   
[191](INES_Mapper_191.xhtml "INES Mapper 191") | 128 | 2 | Yes   
[192](INES_Mapper_192.xhtml "INES Mapper 192") | 8 | 4 | No   
[194](INES_Mapper_194.xhtml "INES Mapper 194") | 0 | 2 | No   
[195](INES_Mapper_195.xhtml "INES Mapper 195") | 0 | 4 | No   
  
* In iNES format, mapper 4 has CHR RAM only if CHR ROM is not present. 

ROMs with the [NES 2.0](NES_2_0.xhtml "NES 2.0") header specify the size of CHR RAM separately. Because the default size is ignored, the seven mappers collapse into four distinct behaviors: 

004, 194, 195
    CHR RAM starts at bank 0 and is mirrored only if there is no CHR ROM
074, 192
    CHR RAM starts at bank 8 and is not mirrored
119
    CHR A16 switches between CHR ROM and CHR RAM
176
    $A001 bit 2 switches between CHR ROM and CHR RAM in the first 8 KiB
191
    CHR A17 switches between CHR ROM and CHR RAM

Categories: [MMC3 with CHR ROM and CHR RAM](Category_MMC3_with_CHR_ROM_and_CHR_RAM.xhtml)
