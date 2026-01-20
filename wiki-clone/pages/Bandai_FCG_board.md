# Bandai FCG board

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Bandai_FCG_board) | View [other pages](Special_AllPages.xhtml#Bandai_FCG_board)

Bandai FCG boards are used largely by Bandai for _Dragon Ball_ and _Gundam_ games, as well as a few others. One Irem title uses it as well. 

All of these games were originally assigned to [INES Mapper 016](INES_Mapper_016.xhtml "INES Mapper 016") before the subtle differences, that are nonetheless relevant for fully functional emulation, became known. Please refer to the [INES Mapper 016](INES_Mapper_016.xhtml "INES Mapper 016") page for a general description of all registers, and to the [INES Mapper 153](INES_Mapper_153.xhtml "INES Mapper 153"), [INES Mapper 157](INES_Mapper_157.xhtml "INES Mapper 157") and [INES Mapper 159](INES_Mapper_159.xhtml "INES Mapper 159") pages for information on the differences of those particular mappers compared to mapper 16. 

  * [INES Mapper 016](INES_Mapper_016.xhtml "INES Mapper 016") submapper 4: FCG-1/2 ASIC, no serial EEPROM, banked CHR-ROM
  * [INES Mapper 016](INES_Mapper_016.xhtml "INES Mapper 016") submapper 5: LZ93D50 ASIC and no or 256-byte serial EEPROM, banked CHR-ROM
  * [INES Mapper 159](INES_Mapper_159.xhtml "INES Mapper 159"): LZ93D50 ASIC and 128-byte serial EEPROM, banked CHR-ROM
  * [INES Mapper 153](INES_Mapper_153.xhtml "INES Mapper 153"): LZ93D50 ASIC and 8 KiB battery-backed WRAM, unbanked CHR-RAM
  * [INES Mapper 157](INES_Mapper_157.xhtml "INES Mapper 157") (Datach Joint ROM System): LZ93D50 ASIC and 256-byte serial EEPROM on Datach Main Unit, optional additional 128-byte serial EEPROM on game cartridge, unbanked CHR-RAM, barcode reader

Name | ASIC | CHR Memory | Save Data | Mapper.Submapper | [NES 2.0 Byte 10](NES_2_0.xhtml#Byte_10_.28RAM_size.29 "NES 2.0")  
---|---|---|---|---|---  
_Akuma-kun: Makai no Wana_ | FCG-2 | CHR-ROM | - | 16.4 | $00   
_Crayon Shin-chan: Ora to Poi Poi_ | LZ93D50 | CHR-ROM | - | 16.5 | $00   
_Dragon Ball: Daimaou Fukkatsu_ | FCG-1 | CHR-ROM | - | 16.4 | $00   
_Dragon Ball 3: Gokuu Den_ | FCG-2 | CHR-ROM | - | 16.4 | $00   
_Dragon Ball Z II: Gekishin Freezer!!_ | LZ93D50 | CHR-ROM | 24C02 | 16.5 | $20   
_Dragon Ball Z III: Ressen Jinzou Ningen_ | LZ93D50 | CHR-ROM | 24C02 | 16.5 | $20   
_Dragon Ball Z Gaiden: Saiya-jin Zetsumetsu Keikaku_ | LZ93D50 | CHR-ROM | 24C02 | 16.5 | $20   
_Famicom Jump: Hero Retsuden_ | FCG-2 | CHR-ROM | - | 16.4 | $00   
_Meimon! Dai-3 Yakyuu-bu_ | FCG-1 | CHR-ROM | - | 16.4 | $00   
_Nishimura Kyoutarou Mystery: Blue Train Satsujin Jiken_ | FCG-1 | CHR-ROM | - | 16.4 | $00   
_Rokudenashi Blues_ | LZ93D50 | CHR-ROM | 24C02 | 16.5 | $20   
_Sakigake!! Otoko Juku: Shippu 1-gou Sei_ | FCG-1 | CHR-ROM | - | 16.4 | $00   
_SD Gundam Gaiden - Knight Gundam Monogatari 2: Hikari no Kishi_ | LZ93D50 | CHR-ROM | 24C02 | 16.5 | $20   
_SD Gundam Gaiden - Knight Gundam Monogatari 3: Densetsu no Kishidan_ | LZ93D50 | CHR-ROM | 24C02 | 16.5 | $20   
_Dragon Ball Z: Kyoushuu! Saiya-jin_ | LZ93D50 | CHR-ROM | 24C01 | 159 | $10   
_Magical Taruruuto-kun: Fantastic World!!_ | LZ93D50 | CHR-ROM | 24C01 | 159 | $10   
_Magical Taruruuto-kun 2: Mahou Daibouken_ | LZ93D50 | CHR-ROM | 24C01 | 159 | $10   
_SD Gundam Gaiden - Knight Gundam Monogatari_ | LZ93D50 | CHR-ROM | 24C01 | 159 | $10   
_Famicom Jump II: Saikyou no 7-nin_ | LZ93D50 | CHR-RAM | 8 KiB WRAM | 153 | $70   
_Datach Crayon Shin-chan: Ora to Poi Poi_ | LZ93D50 | CHR-RAM | Datach Main Unit's 24C02 | 157 | $00   
_Dragon Ball Z: Gekitou Tenkaichi Budoukai_ | LZ93D50 | CHR-RAM | Datach Main Unit's 24C02 | 157 | $00   
_J-League Super Top Players_ | LZ93D50 | CHR-RAM | Datach Main Unit's 24C02 | 157 | $00   
_SD Gundam Wars_ | LZ93D50 | CHR-RAM | Datach Main Unit's 24C02 | 157 | $00   
_Ultraman Club: Spokon Fight!!_ | LZ93D50 | CHR-RAM | Datach Main Unit's 24C02 | 157 | $00   
_Yuu Yuu Hakusho - Bakutou Ankoku Bujutsu-kai_ | LZ93D50 | CHR-RAM | Datach Main Unit's 24C02 | 157 | $00   
_Battle Rush: Build up Robot Tournament_ | LZ93D50 | CHR-RAM | Datach Main Unit's 24C02+24C01 on cartridge | 157 | $10   
  
Categories: [ASIC mappers](Category_ASIC_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml)
