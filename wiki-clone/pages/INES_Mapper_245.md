# INES Mapper 245

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_245) | View [other pages](Special_AllPages.xhtml#INES_Mapper_245)

**iNES Mapper 245** denotes the Waixing **F003** circuit board. It mounts an [MMC3](MMC3.xhtml "MMC3") clone with 8 KiB of battery-backed WRAM and 8 KiB of unbanked CHR-RAM, making it functionally similar to [TNROM](TxROM.xhtml "TNROM"). Additionally however, in a similar fashion to [SUROM](SxROM.xhtml "SUROM"), the MMC3's CHR address outputs are repurposed as higher PRG address lines to extend the PRG address space from the MMC3's own 512 KiB to 1 MiB. 

Its features are used on a single game: "勇者斗恶龙 VII - Dragon Quest" (Yǒngzhě dòu è lóng VII - Dragon Quest), Waixing's Chinese translation of Dragon Quest IV. Waixing used that PCB however for other MMC3/CHR-RAM games with smaller PRG-ROM sizes as well, for which the board reduces to TNROM-like functionality. 

# Pin Connections
    
    
    Normal MMC3 pin function  F003 pin function
    -------------------------------------------
    PPU A12 ->                GND ->
    -> CHR A10                none
    -> CHR A11                -> PRG A19
    -> CHR A12                none
    -> CHR A13                none
    -> CHR A14                none
    -> CHR A15                none
    -> CHR A16                none
    -> CHR A17                none
    

CHR-RAM's A10-A12 inputs are connected directly to PPU A10-A12; as a result, the MMC3's CHR bank registers have no effect at all on pattern tables. The MMC3's PPU A12 input is connected to GND, meaning that only MMC3 CHR bank registers 0/1 ($8000.7=0) or 2-5 ($8000.7=1) are functional. The MMC3's CHR A11 output is connected to the PRG-ROM chip's A19 input. This means that bit 1 (bit value $02) of the currently active MMC3 CHR bank register selects the 512 KiB PRG-ROM bank that applies to the entire CPU $8000-$FFFF address range. 

# Registers
    
    
    $8001 when $8000=$x0: [.... ..B.]
              Select 512 KiB PRG-ROM bank at CPU $8000-$FFFF
              when PPU is rendering from BG or OBJ tiles $00-$7F
              and $8000.7=0
    $8001 when $8000=$x1: [.... ..B.]
              Select 512 KiB PRG-ROM bank at CPU $8000-$FFFF
              when PPU is rendering from BG or OBJ tiles $80-$FF
              and $8000.7=0
    
    $8001 when $8000=$x2: [.... ..B.]
              Select 512 KiB PRG-ROM bank at CPU $8000-$FFFF
              when PPU is rendering from BG or OBJ tiles $00-$3F
              and $8000.7=1
    $8001 when $8000=$x3: [.... ..B.]
              Select 512 KiB PRG-ROM bank at CPU $8000-$FFFF
              when PPU is rendering from BG or OBJ tiles $40-$7F
              and $8000.7=1
    $8001 when $8000=$x4: [.... ..B.]
              Select 512 KiB PRG-ROM bank at CPU $8000-$FFFF
              when PPU is rendering from BG or OBJ tiles $80-$BF
              and $8000.7=1
    $8001 when $8000=$x5: [.... ..B.]
              Select 512 KiB PRG-ROM bank at CPU $8000-$FFFF
              when PPU is rendering from BG or OBJ tiles $C0-$FF
              and $8000.7=1
    

Registers 0/1 or 2-5 must specify the same 512 KiB PRG-ROM bank, otherwise the bank would switch as the PPU is rendering. The only game using this mapper has boot code and reset vectors in both 512 KiB PRG halves, making the initial CHR register content irrelevant. 

Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
