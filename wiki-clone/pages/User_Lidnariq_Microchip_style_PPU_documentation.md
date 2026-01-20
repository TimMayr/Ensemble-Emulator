# User:Lidnariq/Microchip-style PPU documentation

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3ALidnariq/Microchip-style_PPU_documentation) | View [other pages](Special_AllPages.xhtml#User_Lidnariq_Microchip_style_PPU_documentation)

## Contents

  * 1 Microchip-style documentation format
    * 1.1 REGISTER: PPUCTRL: PPU properties
    * 1.2 REGISTER: PPUMASK: PPU rendering modifications
    * 1.3 REGISTER: PPUSTATUS: PPU status



## Microchip-style documentation format

### REGISTER: PPUCTRL: PPU properties

W-0/0 | W-0/0 | W-0/0 | W-0/0 | W-0/0 | W-0/0 | W-0/0 | W-0/0   
---|---|---|---|---|---|---|---  
NMIEN | EXTOUTEN | SPR8x16 | BKGD | SPR | INC32 | Y8 | X8   
bit 7 |  | bit 0   
**Legend**  
---  
R = Readable bit | W = Writable bit | U = Unimplemented bit, read as open bus   
u = bit is unchanged | x = bit is unknown | -n/n = value on Power-on/all other resets   
1 = bit is set | 0 = bit is cleared | q = value depends on conditions   
bit 7 | **NMIEN** : NMI enable  
1 = NMI is triggered at start of vblank, after 1 (or 51) post-render scanlines  
0 = NMI is not triggered   
---|---  
bit 6 | **EXTOUTEN** : External output Enable  
1 = output bottom 4 bits of color on EXT pins  
0 = accept bottom 4 bits of backdrop on EXT pins (background colors 1-3 are opaque; color 0 is transparent and from the external port)   
bit 5 | **SPR8x16** : Sprites size  
1 = All sprites are 8x16  
0 = All sprites are 8x8   
bit 4 | **BKGD** : Background tile location  
1 = Background tiles are fetched from PPU $1xxx  
0 = Background tiles are fetched from PPU $0xxx   
bit 3 | **SPR** : Sprite tile location  
_8x16 sprites:_  
Don't care.  
_8x8 sprites:_  
1 = Sprite tiles are fetched from PPU $1xxx  
0 = Sprite tiles are fetched from PPU $0xxx   
bit 2 | **INC32** : PPU address autoincrement step size  
1 = PPU address increments by 32 after reads or writes to PPUDATA  
0 = PPU address increments by 1 after reads or writes to PPUDATA   
bit 1 | **Y8** : 240s bit of Y scroll position  
1 = Rendering will start in nametable at $2800 or $2C00  
0 = Rendering will start in nametable at $2000 or $2400   
bit 0 | **X8** : 256s bit of X scroll position  
1 = Rendering will start in nametable at $2400 or $2C00  
0 = Rendering will start in nametable at $2000 or $2800   
  
### REGISTER: PPUMASK: PPU rendering modifications

W-0/0 | W-0/0 | W-0/0 | W-0/0 | W-0/0 | W-0/0 | W-0/0 | W-0/0   
---|---|---|---|---|---|---|---  
BLUE | GREEN | RED | SPREN | BKGDEN | SPRCLIP | BKGDCLIP | MONO   
bit 7 |  | bit 0   
**Legend**  
---  
W = Writable bit  | 0 = bit is cleared  | -n/n = value on Power-on/all other resets   
bit 7 | **BLUE** : Blue emphasis  
1 = (2C02, 2C07, Dendy) red and green dimmed, blues emphasized; (2C03,2C04,2C05) blue set to maximum brightness   
0 = normal   
---|---  
bit 6 | **GREEN** : Green emphasis  
1 = (2C02) red and blue dimmed, green emphasized; (2C03,2C04,2C05) green set to maximum brightness; (2C07, Dendy) **green** and blue dimmed, **red** emphasized  
0 = normal   
bit 5 | **RED** : Red emphasis  
1 = (2C02) green and blue dimmed, red emphasized; (2C03,2C04,2C05) red set to maximum brightness; (2C07, Dendy) **red** and blue dimmed, **green** emphasized  
0 = normal   
bit 4 | **SPREN** : Sprite rendering enable  
1 = Sprites are drawn to screen, and PPU bus is active  
0 = Sprites are not drawn   
bit 3 | **BKGDEN** : Background rendering enable  
1 = Background is drawn to screen, and PPU bus is active  
0 = Background is not drawn. If both **SPREN** and **BKGDEN** are clear, then the PPU bus is inactive   
bit 2 | **SPRCLIP** : Draw sprites in leftmost 8 pixels  
1 = Sprites are visible in leftmost 8 columns  
0 = Sprites are NOT visible in leftmost 8 columns. Sprite 0 doesn't trigger on pixels that are not drawn.   
bit 1 | **BKGDCLIP** : Draw backgrounds in leftmost 8 pixels  
1 = Backgrounds are visible in leftmost 8 columns  
0 = Backgrounds are NOT visible in leftmost 8 columns. Sprite 0 doesn't trigger on pixels that are not drawn.   
bit 0 | **MONO** : Monochrome display  
1 = All PPU palette reads for any purpose (rendering or via PPUDATA) are masked with $30.  
0 = PPU palette reads function as normal, specifying any value of the normal palette   
  
### REGISTER: PPUSTATUS: PPU status

R/HC-x/q | R/HC-0/u | R/HC-x/u | U-q | U-q | U-q | U-q | U-q   
---|---|---|---|---|---|---|---  
VBL | SPR0 | SPROV | — | — | — | — | —   
bit 7 |  | bit 0   
**Legend**  
---  
R = Readable bit | W = Writable bit | U = Unimplemented bit, read as open bus   
u = bit is unchanged | x = bit is unknown | -n/n = value on Power-on/all other resets   
0 = bit is cleared | q = value depends on conditions | HC = Cleared by hardware   
bit 7 | **VBL** : Vertical blanking  
1 = An NMI would occur (or has occured)  
0 = This register has already been read this frame or it's more than 20 scanlines after the start of vertical blanking.   
---|---  
bit 6 | **SPR0** : Sprite 0 collision  
1 = A pixel in sprite 0 has overlaid some opaque pixel from the background  
0 = No pixel in sprite 0 has overlaid any opaque pixel in the background since 20 scanlines after the start of vertical blanking.   
bit 5 | **SPROV** : Sprite overflow  
1 = More than 8 sprites attempted to be rendered on a scanline (but see [Errata](Errata.xhtml "Errata"))  
0 = No scanline has had more than 8 sprites on it this frame.   
bit 4-0 | **Unimplemented** : Read as PPU's open bus  

