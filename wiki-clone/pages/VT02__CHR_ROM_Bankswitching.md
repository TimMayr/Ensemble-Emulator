# VT02+ CHR-ROM Bankswitching

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/VT02%2B_CHR-ROM_Bankswitching) | View [other pages](Special_AllPages.xhtml#VT02__CHR_ROM_Bankswitching)

In OneBus mode, VT02+ consoles combine the Famicom cartridge connector's CPU and PPU address lines into one 32 MiB address space, with separate CPU and PPU bankswitch registers pointing to appropriately-placed code and picture data. The bankswitching scheme is based on, and indeed backwards-compatible to, the Nintendo [MMC3](MMC3.xhtml "MMC3")'s. The PPU address range reserved for CHR pattern data ($0000-$1FFF) is divided into 2x2 KiB and 4x1 KiB banks (2x4 KiB and 4x2 KiB in 4bpp modes), with the bank numbers always specified with 1 KiB granularity (2 KiB granularity in 4bpp modes). The final bank number is made up of five components: 

  * an Extended Video Address (EVA) if [Address Extension](VT02__Video_Modes.xhtml#Address_Extension "VT02+ Video Modes") is active,
  * an Inner Bank that resembles the MMC3's bank registers,
  * a Middle Bank that can replace zero to eight bits of the lower bank number,
  * an Intermediate Bank if Address Extension if **not** active,
  * an Outer Bank that extend the address range up to 32 MiB,



## Contents

  * 1 Extended Video Address
  * 2 Inner CHR Bank number
  * 3 Middle CHR Bank number
  * 4 Intermediate CHR Bank number
  * 5 Outer CHR Bank number
  * 6 Final CHR Bank Number
  * 7 Final effective CHR address
  * 8 Mapping into 16-bit PPU address space



## Extended Video Address

If either Background or Sprite Address Extension is **active** while the respective pattern data are fetched, or either is active while data is read or written via $2007, the Extended Video Address (EVA) provides the lowest three bits of the CHR bank number. Please refer to the [VT02+ Video Modes](VT02__Video_Modes.xhtml#Address_Extension "VT02+ Video Modes") article for information on the way that the three bits of the Extended Video Address are derived. 

## Inner CHR Bank number

The lower bits bits of the 8 KiB PRG-ROM bank number, constituting the Inner Bank number, are normally the main ones that are manipulated by individual games. 
    
    
    PPU $0000-$03FF: Selected by register $2016 (RV4) AND $FE, akin to MMC3 register 0.
    PPU $0400-$07FF: Selected by register $2016 (RV5) OR $01, akin to MMC3 register 0.
    PPU $0800-$0BFF: Selected by register $2017 (RV5) AND $FE, akin to MMC3 register 1.
    PPU $0C00-$0FFF: Selected by register $2017 (RV5) OR $01, akin to MMC3 register 1
    PPU $1000-$13FF: Selected by register $2012 (RV0), akin to MMC3 register 2.
    PPU $1400-$17FF: Selected by register $2013 (RV1), akin to MMC3 register 3.
    PPU $1800-$1BFF: Selected by register $2014 (RV2), akin to MMC3 register 4.
    PPU $1C00-$1FFF: Selected by register $2015 (RV3), akin to MMC3 register 5.
    

If $4105 bit 7 (COMR7) is 1, then the sources of the $0000-$0FFF bank numbers are swapped with the $1000-$1FFF banks', just as on the MMC3, or in other words, PPU A12 is inverted. 

## Middle CHR Bank number

The Middle Bank is normally only used on multicarts. It allows masking off and replacing bits of the Inner Bank number, so that several games may be put into one Outer Bank. Bits 0-2 of register $201A (VB0S) select the AND mask that is applied to the Inner Bank number. Only the bits that have been masked off that way are then replaced with the respective bits from register $201A bits 3-7 (RV6): 
    
    
    $201A     Inner Bank  Middle Bank Effective
    bits 0-2  AND Mask    AND Mask    Inner Bank Size
    --------  ----------  --------    ---------------
    0         FF          00          256 KiB
    1         7F          80          128 KiB
    2         3F          C0          64 KiB
    3         invalid
    4         1F          E0          32 KiB
    5         0F          F0          16 KiB
    6         07          F8          8 KiB
    7         invalid
    

A second use of the Middle CHR Bank number is to eschew 1 KiB CHR-ROM granularity and switch 8 KiB of CHR-ROM at once in a CNROM-like fashion, by setting the Inner Bank AND Mask to zero and then using the Middle CHR Bank number as an 8 KiB bank number. A few multicarts use this technique to include CNROM games with minimal modification. 

## Intermediate CHR Bank number

The Intermediate CHR Bank is only used if Background or Sprite Address Extension is **not active** while the respective pattern data are fetched, or neither is active while data is read or written via $2007. It provides three bits that go between the Middle and Outer CHR Bank number. A single Intermediate Bank number applies to all six CHR banks. 
    
    
    PPU $0000-$1FFF: Selected by register $2018 bits 4-6 (VA18-20).
    

## Outer CHR Bank number

The Outer Bank number is used mostly by multicarts, but also by semi-large games for which the maximum Inner Bank size of 256 KiB is insufficient. A single Outer CHR Bank number applies to all six CHR banks. 
    
    
    PPU $0000-$1FFF: Selected by register $4100 bits 0-3 (VA21-24).
    

## Final CHR Bank Number

The final 1 KiB (2 KiB in 4bpp modes) CHR Bank number therefore is: 

If Address Extension not active: 
    
    
    BankNumber =                          ((InnerBank &InnerBankMask) | (MiddleBank &~InnerBankMask) | (IntermediateBank <<8)) | (OuterBank <<11);
    

If Address Extension is active: 
    
    
    BankNumber = ExtendedVideoAddress | ( ((InnerBank &InnerBankMask) | (MiddleBank &~InnerBankMask)                    ) <<3) | (OuterBank <<11);
    

This scheme implies that when Address Extension is active, the Inner and Middle Bank number registers must be loaded with values SHR 3 compared to the values they would have if Address Extension were inactive. It also shows that using only Background or Sprite Address Extension, but not both, becomes difficult to use if the Middle Bank is to be used. The Outer Bank is not affected by Address Extension. 

## Final effective CHR address

The actual CHR-ROM address being accessed depends on the number of bits per pixel, the data bus width, and whether address extension is enabled: 2 bpp, address extension disabled: 
    
    
           Address bit
    -------------------------
    2222211111111110000000000
    4321098765432109876543210
    OOOOIIIMMMMMMMMTTTTTTPRRR
    ||||||||||||||||||||||+++- Tile row
    |||||||||||||||||||||+---- Bit plane
    |||||||||||||||++++++----- Tile number D0..D5
    |||||||++++++++----------- MMC3-compatible inner bank number,
    |||||||                    Bank register selected by Tile number D6..D7 and $2000.3-4,
    |||||||                    Masked via $201A.0-2, substituted via $201A.3-7
    ||||+++------------------- Intermediate bank number ($2018.4-6)
    ++++---------------------- Outer bank number ($4100.0-3)
    

2 bpp, address extension enabled: 
    
    
           Address bit
    -------------------------
    2222211111111110000000000
    4321098765432109876543210
    OOOOMMMMMMMMEEETTTTTTPRRR
    ||||||||||||||||||||||+++- Tile row
    |||||||||||||||||||||+---- Bit plane
    |||||||||||||||++++++----- Tile number D0..D5
    ||||||||||||+++----------- Enhanced Video Address
    ||||++++++++-------------- MMC3-compatible inner bank number,
    ||||                       Bank register selected by Tile number D6..D7 and $2000.3-4,
    ||||                       Masked via $201A.0-2, substituted via $201A.3-7
    ++++---------------------- Outer bank number ($4100.0-3)
    

4 bpp, address extension disabled, 8-bit bus: 
    
    
           Address bit
    -------------------------
    2222211111111110000000000
    4321098765432109876543210
    OOOIIIMMMMMMMMTTTTTTPPRRR
    ||||||||||||||||||||||+++- Tile row
    ||||||||||||||||||||++---- Bit plane
    ||||||||||||||++++++------ Tile number D0..D5
    ||||||++++++++------------ MMC3-compatible inner bank number,
    ||||||                     Bank register selected by Tile number D6..D7 and $2000.3-4,
    ||||||                     Masked via $201A.0-2, substituted via $201A.3-7
    |||+++-------------------- Intermediate bank number ($2018.4-6)
    +++----------------------- Outer bank number ($4100.0-2)
    

4 bpp, address extension enabled, 8-bit bus: 
    
    
           Address bit
    -------------------------
    2222211111111110000000000
    4321098765432109876543210
    OOOMMMMMMMMEEETTTTTTPPRRR
    ||||||||||||||||||||||+++- Tile row
    ||||||||||||||||||||++---- Bit plane
    ||||||||||||||++++++------ Tile number D0..D5
    |||||||||||+++------------ Enhanced Video Address
    |||++++++++--------------- MMC3-compatible inner bank number,
    |||                        Bank register selected by Tile number D6..D7 and $2000.3-4,
    |||                        Masked via $201A.0-2, substituted via $201A.3-7
    +++----------------------- Outer bank number ($4100.0-2)
    

4 bpp, address extension disabled, 16-bit bus: 
    
    
           Address bit
    -------------------------
    2222211111111110000000000
    4321098765432109876543210
    OOOIIIMMMMMMMMTTTTTTPRRRp
    ||||||||||||||||||||||||+- Bit plane D0
    |||||||||||||||||||||+++-- Tile row
    ||||||||||||||||||||+----- Bit plane D1
    ||||||||||||||++++++------ Tile number D0..D5
    ||||||++++++++------------ MMC3-compatible inner bank number,
    ||||||                     Bank register selected by Tile number D6..D7 and $2000.3-4,
    ||||||                     Masked via $201A.0-2, substituted via $201A.3-7
    |||+++-------------------- Intermediate bank number ($2018.4-6)
    +++----------------------- Outer bank number ($4100.0-2)
    

4 bpp, address extension enabled, 16-bit bus: 
    
    
           Address bit
    -------------------------
    2222211111111110000000000
    4321098765432109876543210
    OOOMMMMMMMMEEETTTTTTPRRRp
    ||||||||||||||||||||||||+- Bit plane D0
    |||||||||||||||||||||+++-- Tile row
    ||||||||||||||||||||+----- Bit plane D1
    ||||||||||||||++++++------ Tile number D0..D5
    |||||||||||+++------------ Enhanced Video Address
    |||++++++++--------------- MMC3-compatible inner bank number,
    |||                        Bank register selected by Tile number D6..D7 and $2000.3-4,
    |||                        Masked via $201A.0-2, substituted via $201A.3-7
    +++----------------------- Outer bank number ($4100.0-2)
    

This scheme implies that when 4bpp are used, all bank register numbers must be loaded with values SHR 1 compared to the values they would have if 2bpp were used. It also becomes clear that choosing bank numbers properly when using 4bpp only for background or sprites, but not both, becomes quite difficult and requires careful planning of one's CHR-ROM layout. 

## Mapping into 16-bit PPU address space

When accessing pattern tables via $2007 in 4bpp modes, the now-16 KiB per pattern tables are spread across two address ranges: 
    
    
    PPU $0000-$1FFF: CHR pattern data, bit planes 0 and 1
    PPU $4000-$5FFF: CHR pattern data, bit planes 2 and 3
    

In other words, PPU A14 becomes bit 1 of the bit plane number. 
