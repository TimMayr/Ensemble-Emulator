# INES Mapper 095

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_095) | View [other pages](Special_AllPages.xhtml#INES_Mapper_095)

Mapper 95 represents **NAMCOT-3425** , a board used only for the game [Dragon Buster (J)](https://nescartdb.com/profile/view/1806/dragon-buster). 

It is to the ordinary Namco 108 family boards ([mapper 206](INES_Mapper_206.xhtml "INES Mapper 206")) as TKSROM and TLSROM ([mapper 118](INES_Mapper_118.xhtml "INES Mapper 118")) is to ordinary [MMC3](MMC3.xhtml "MMC3") boards. Instead of having hardwired mirroring like mapper 206, it has CHR A15 directly controlling CIRAM A10, just as CHR A17 controls CIRAM A10 on TxSROM. Only horizontal mirroring and 1-screen mirroring are possible because the Namco 108 lacks the C bit of MMC3. 

_Dragon Buster_ has only 32 KiB of CHR ROM, but homebrew using this mapper could use the full 64 KiB through careful arrangement of CHR ROM to put game background tiles in one half (which would always use one nametable) and status bar and menu tiles in the other half (which would always use the other nametable). 

Disch's older notes described an extended and mistaken version of this mapper with a full MMC3, so some emulators may be doing it that way. That extended version is identical to mapper 118, except CHR A15 (bank bit 5) controls CIRAM A10 instead of CHR A17 (bank bit 7). 

Here's a terse pseudocode: 
    
    
    chr_rom_addr = namco108_chrmap(ppu_addr&0x1FFF) & 0xFFFF;
    ciram_addr = ((namco108_chrmap(ppu_addr&0x1FFF)>>15)<<10) | (ppu_addr&0x03FF);
    

### Bank select ($8000-$9FFE, even)
    
    
    7  bit  0
    ---- ----
    xxxx xRRR
          |||
          +++- Specify which bank register to update on next write to Bank Data register
               0: Select 2 KB CHR bank at PPU $0000-$07FF and nametable at PPU $2000-$27FF
               1: Select 2 KB CHR bank at PPU $0800-$0FFF and nametable at PPU $2800-$2FFF
               2: Select 1 KB CHR bank at PPU $1000-$13FF
               3: Select 1 KB CHR bank at PPU $1400-$17FF
               4: Select 1 KB CHR bank at PPU $1800-$1BFF
               5: Select 1 KB CHR bank at PPU $1C00-$1FFF
               6: Select 8 KB PRG ROM bank at $8000-$9FFF
               7: Select 8 KB PRG ROM bank at $A000-$BFFF
    

### Bank data ($8001-$9FFF, odd)
    
    
    7  bit  0
    ---- ----
    ..ND DDDD
      || ||||
      |+-++++- New bank value, based on last value written to Bank select register
      |          All registers as standard [Namco 108](INES_Mapper_206.xhtml "INES Mapper 206")
      |
      +------- Nametable select, based on last value written to Bank select register
                 0: Select Nametable A
                 1: Select Nametable B
    

Categories: [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
