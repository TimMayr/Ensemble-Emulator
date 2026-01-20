# INES Mapper 159

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_159) | View [other pages](Special_AllPages.xhtml#INES_Mapper_159)

iNES Mapper 159 is used for [Bandai FCG](Bandai_FCG_board.xhtml "Bandai FCG") boards with an [LZ93D50 ASIC](Bandai_LZ93D50_pinout.xhtml "Bandai LZ93D50 pinout") and a 128-byte serial EEPROM (X24C01). The 128 bytes must be denoted as PRG-NVRAM in the [NES 2.0](NES_2_0.xhtml "NES 2.0") header using byte value $10. 

# Game List

  * _Dragon Ball Z: Kyoushuu! Saiya-jin_
  * _Magical Taruruuto-kun: Fantastic World!!_
  * _Magical Taruruuto-kun 2: Mahou Daibouken_
  * _SD Gundam Gaiden - Knight Gundam Monogatari_



# Banks

  * CPU $8000-$BFFF: 16 KiB switchable PRG ROM bank
  * CPU $C000-$FFFF: 16 KiB PRG ROM bank, fixed to the last bank
  * PPU $0000-$03FF: 1 KiB switchable CHR ROM bank
  * PPU $0400-$07FF: 1 KiB switchable CHR ROM bank
  * PPU $0800-$0BFF: 1 KiB switchable CHR ROM bank
  * PPU $0C00-$0FFF: 1 KiB switchable CHR ROM bank
  * PPU $1000-$13FF: 1 KiB switchable CHR ROM bank
  * PPU $1400-$17FF: 1 KiB switchable CHR ROM bank
  * PPU $1800-$1BFF: 1 KiB switchable CHR ROM bank
  * PPU $1C00-$1FFF: 1 KiB switchable CHR ROM bank



# Registers

All registers function the same way as [INES Mapper 016](INES_Mapper_016.xhtml "INES Mapper 016"), submapper 5, except that the serial EEPROM is not a 24C02 but Xicor's X24C01. This EEPROM has operational differences compared to both the larger 24C02 and other vendors' 24C01. Bandai's developers consistently thought the X24C01 was a little-endian device, and their code clocks in address and data bytes assuming this. However, it's actually a big-endian device, and a multi-byte read or write from EEPROM will increment the address accordingly. 

Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml)
