# User:Lidnariq/MMC3 Variants

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3ALidnariq/MMC3_Variants) | View [other pages](Special_AllPages.xhtml#User_Lidnariq_MMC3_Variants)

## Contents

  * 1 Legend for bank style
    * 1.1 PRG
    * 1.2 CHR
    * 1.3 Outer banks
  * 2 ASIC mappers with simple banking
  * 3 Mappers with outer banks
  * 4 Discrete logic mappers



## Legend for bank style

### PRG

  * 32 - Banks 32 KiB at a time
  * 16+16F - Bottom 16 KiB are bankable, top 16 KiB are fixed
  * 16F+16 - Bottom 16 KiB are fixed, top 16 KiB are bankable
  * 8+24F - Bottom 8 KiB are bankable, top 24 KiB are fixed
  * 3×8+8F - Three independent 8 KiB banks at bottom, topmost 8 KiB are fixed
  * N108 - The Namco 108 set the theme that many other bank styles derive from ( a.k.a. 8+8+16F = two independent 8 KiB banks at bottom, topmost 16 KiB are fixed)
  * MMC3 - The MMC3 extended the banking style of the Namco 108 ( user switches between N108 and 8F+8+8+8F = bottommost and topmost 8 KiB are fixed, middle two 8 KiB banks are independently bankable)



### CHR

  * 8 - Banks 8 KiB at a time
  * 2×4 - Two independent 4 KiB banks
  * 4+4F - Bottom 4 KiB are bankable, top 4 KiB are fixed
  * 4×2 - Four independent 2 KiB banks
  * 8×1 - Eight independent 1 KiB banks
  * N108 - Set the trend for CHR banking style also. Equivalent to 2×2+4×1 = two independent 2 KiB banks followed by four independent 1 KiB banks
  * MMC3 - allows the user to swap which pattern table gets the "large" and "small" banks



### Outer banks

"Outer banks" are things you find in the [multicart](Multicart.xhtml "Multicart") mappers, where individual games write to the registers of the "regular" mapper and the menu writes to a separate set of registers that chooses which game will be played. For example, Nintendo World Cup appeared on two boards that extend the MMC3 in this way: one with [_SMB1_ and _Tetris_](INES_Mapper_037.xhtml "INES Mapper 037") and one with [_Super Spike V'Ball_](INES_Mapper_047.xhtml "INES Mapper 047"). 

This same structure occasionally shows up in ordinary single-game mappers: [Bandai's _Oeka Kids_ games](INES_Mapper_096.xhtml "INES Mapper 096"), [Namco's _Devil Man_](INES_Mapper_154.xhtml "INES Mapper 154"), [several other Namco games](INES_Mapper_088.xhtml "INES Mapper 088"), and a [few](INES_Mapper_012.xhtml "INES Mapper 012") HKOs [that](INES_Mapper_014.xhtml "INES Mapper 014") would like better corroboration. 

## ASIC mappers with simple banking

Also see [Comparison of Nintendo mappers](Comparison_of_Nintendo_mappers.xhtml "Comparison of Nintendo mappers")

| PRG | CHR   
---|---|---  
iNES Mapper | Short name | bank style | max (KiB) | bank style | max (KiB) | Mirroring | IRQ | WRAM | Comments   
[206](INES_Mapper_206.xhtml "INES Mapper 206") | N108 | 8+8+16F | 128 | 2×2+4×1 | 64 | fixed | none | none | none   
[4](MMC3.xhtml "INES Mapper 004") | [MMC3](MMC3.xhtml "MMC3"), [MMC6](MMC6.xhtml "MMC6") | N108; 8F+8+8+8F | 512 | N108; 4×1+2×2 | 256 | H/V | scanline | ext (MMC6 1024B int) |   
(none) | [Famicom Disk System](Family_Computer_Disk_System.xhtml "Family Computer Disk System") | 32RAM+8F | arbitrary | unbanked | 8 | H/V | cycle | none | [Expansion audio](FDS_audio.xhtml "FDS audio")  
[1](MMC1.xhtml "INES Mapper 001")/[155](MMC1.xhtml "INES Mapper 155") | [MMC1](MMC1.xhtml "MMC1") | 16+16F; 16F+16; 32 | 256 (512) | 8; 2×4 | 128 | 1/H/V | none | [sometimes bankable](MMC1.xhtml "SxROM") ext |   
[5](MMC5.xhtml "INES Mapper 005") | [MMC5](MMC5.xhtml "MMC5") | 32; 2×16; 16+8+8; 4×8 | 1024 | 8; 2×4; 4×2; 8×1 | 1024 | arbitrary | scanline | bankable ext + 1024B int | [Expansion audio](MMC5_audio.xhtml "MMC5 audio"), vertical split, hardware multiplier, 8x8 attribute zones   
[9](MMC2.xhtml "INES Mapper 009") | [MMC2](MMC2.xhtml "MMC2") | 8+24F | 128 | 2×4 | 128 | H/V | none | none | tile triggered CHR banking   
[10](MMC4.xhtml "INES Mapper 010") | [MMC4](MMC4.xhtml "MMC4") | 16+16F | 256 | 2×4 | 128 | H/V | none | ext | tile triggered CHR banking   
[16](INES_Mapper_016.xhtml "INES Mapper 016")/[159](INES_Mapper_159.xhtml "INES Mapper 159") | [LZ93D50](Bandai_FCG_board.xhtml "Bandai FCG board") (I²C) | 16+16F | 256 | 8×1 | 256 | 1/H/V | cycle | serial EEPROM |   
[18](INES_Mapper_018.xhtml "INES Mapper 018") | Jaleco SS88006 | 3×8+8F | 256? | 8×1 | 256 | 1/H/V | cycle | ext | ADPCM expansion audio   
[19](INES_Mapper_019.xhtml "INES Mapper 019") | [Namco 163](INES_Mapper_019.xhtml "Namco 163") | 3×8+8F | 512 | 12×1 | 256 | arbitrary | cycle | ext + 128B int | [Expansion audio](Namco_163_audio.xhtml "Namco 163 audio"), ROM nametables   
[21](VRC2_and_VRC4.xhtml "INES Mapper 021")/[23](VRC2_and_VRC4.xhtml "INES Mapper 023")/[25](VRC2_and_VRC4.xhtml "INES Mapper 025") | [VRC4](VRC2_and_VRC4.xhtml "VRC4") | MMC3 | 256 | 8×1 | 512 | 1/H/V | scanline | ext |   
[22](VRC2_and_VRC4.xhtml "INES Mapper 022")/[23](VRC2_and_VRC4.xhtml "INES Mapper 023")/[25](VRC2_and_VRC4.xhtml "INES Mapper 025") | [VRC2](VRC2_and_VRC4.xhtml "VRC2") | N108 | 256 | 8×1 | 256 | 1/H/V | none | serial EEPROM |   
[24](VRC6.xhtml "INES Mapper 024")/[26](VRC6.xhtml "INES Mapper 026") | [VRC6](VRC6.xhtml "VRC6") | 16+8+8F | 256 | 8×1; 6×1+2×3; 4×2+4×1; 4×1+4×2 | 256 (512) | arbitrary | scanline | ext | [Expansion audio](VRC6_audio.xhtml "VRC6 audio"), ROM nametables   
[27](INES_Mapper_027.xhtml "INES Mapper 027") | "Pirate" VRC4 | 3×8+8F | ?256? | 8×1 | 512 | 1/H/V | ? | ? |   
[31](INES_Mapper_031.xhtml "INES Mapper 031") | [NSF](NSF.xhtml "NSF") subset | 8×4 | 1024 | unbanked | 8 | fixed | none | none |   
[32](INES_Mapper_032.xhtml "INES Mapper 032") | Irem G101 | MMC3 | 256 | 8×1 | 256 | H/V | none | none |   
[33](INES_Mapper_033.xhtml "INES Mapper 033")/[48](INES_Mapper_048.xhtml "INES Mapper 048") | TC0190 | N108 | 512 | N108 | 512 | H/V | scanline(48) | ext |   
[64](RAMBO_1.xhtml "INES Mapper 064") | [RAMBO-1](RAMBO_1.xhtml "RAMBO-1") | 3×8+8F | 256 | MMC3; 8×1 | 256 | H/V | scanline/cycle | none |   
[65](INES_Mapper_065.xhtml "INES Mapper 065") | Irem H3001 | 3×8+8F | 512 | 8×1 | 256 | H/V | cycle | ext |   
[67](INES_Mapper_067.xhtml "INES Mapper 067") | Sunsoft 3 | 16+16F | 128? | 4×2 | 128? | 1/H/V | cycle | ext |   
[68](INES_Mapper_068.xhtml "INES Mapper 068") | Sunsoft 4 | 16+16F | 256 | 4×2 | 256 | 1/H/V | none | ext | ROM nametables   
[69](Sunsoft_FME_7.xhtml "INES Mapper 069") | [Sunsoft FME-7](Sunsoft_FME_7.xhtml "Sunsoft FME-7") | 4×8+8F | 256 | 8×1 | 256 | 1/H/V | cycle | ext | [Expansion audio](Sunsoft_5B_audio.xhtml "Sunsoft 5B audio")  
[73](VRC3.xhtml "INES Mapper 073") | [VRC3](VRC3.xhtml "VRC3") | 16+16F | 128 | unbanked | 8 | fixed | cycle | ext |   
[74](INES_Mapper_074.xhtml "INES Mapper 074")/[194](INES_Mapper_194.xhtml "INES Mapper 194") | TQROM-like (2kiB) | MMC3 | 1024 | MMC3 | 254 | 1/H/V? | scanline? | ext? | CHR RAM and ROM, ?like TQROM?   
[75](VRC1.xhtml "INES Mapper 075") | [VRC1](VRC1.xhtml "VRC1") | 3×8+8F | 128 | 2×4 | 128 | H/V | none | none |   
[76](INES_Mapper_076.xhtml "INES Mapper 076") | N108 (inflated CHR) | N108 | 128 | 4×2 | 128 | fixed | none | none |   
[80](INES_Mapper_080.xhtml "INES Mapper 080") | X1-005 | 3×8+8F | 256 | N108 | 256 | H/V | none | 128B int |   
[82](Taito_X1_017.xhtml "INES Mapper 082") | X1-017 | 3×8+8F | 512? | MMC3 | 256 | H/V | cycle | 5120B int |   
[85](VRC7.xhtml "INES Mapper 085") | [VRC7](VRC7.xhtml "VRC7") | 3×8+8F | 512 | 8×1 | 256 | 1/H/V | scanline | ext | [Expansion audio](VRC7_audio.xhtml "VRC7 audio")  
[90](J_Y__Company_ASIC.xhtml "INES Mapper 090") | for: Tekken 2 HKO | 8+32F; 8+16+16F; 4×8+8F; 8+32; 8+2×16; 4×8 | 2048? | 8; 2×4; 4×2; 8×1 | 2048? | 1/H/V | scanline/cycle/pixel | ? | hardware multiplier   
[209](J_Y__Company_ASIC.xhtml "INES Mapper 209")/[211](J_Y__Company_ASIC.xhtml "INES Mapper 211") | for: Tekken 2 HKO | 8+32F; 8+16+16F; 4×8+8F; 8+32; 8+2×16; 4×8 | 2048? | 8+4×1; 2×4+4×1; 4×2+4×1; 12×1 | 2048? | arbitrary | scanline/cycle/pixel | ? | ROM nametables, hardware multiplier   
[91](INES_Mapper_091.xhtml "INES Mapper 091") | for: Street Fighter 3 HKO | N108 | 128 | 4×2 | 512 | ? | ? | ? |   
[95](INES_Mapper_095.xhtml "INES Mapper 095") | N108 (nametable) | N108 | 128 | N108 | 32 | 1/H | none | none |   
[97](INES_Mapper_097.xhtml "INES Mapper 097") | Irem TAM-S1 | 16F+16 | 256 | unbanked | 8 | 1/H/V | none | none |   
[105](NES_EVENT.xhtml "INES Mapper 105") | [NES-EVENT](NES_EVENT.xhtml "NES-EVENT") | 16+16F; 16F+16; 32 | 256 | 1×8; 2×4 | 8 | 1/H/V | other | ext |   
[112](INES_Mapper_112.xhtml "INES Mapper 112") | "chinese" N108 | N108 | 256 | N108 | 256 | H/V | ? | ? |   
[118](INES_Mapper_118.xhtml "INES Mapper 118") | [TLSROM](TLSROM.xhtml "TLSROM"), [TKSROM](TKSROM.xhtml "TKSROM") | MMC3 | 512 | MMC3 | 128 | arbitrary | scanline | ext |   
[119](INES_Mapper_119.xhtml "INES Mapper 119") | TQROM (8kiB) | MMC3 | 512 | MMC3 | 64 | H/V | scanline | ext | CHR RAM and ROM   
[137](INES_Mapper_137.xhtml "INES Mapper 137") | Sachen 8259 type D | 32 | 256 | 4×1+1×4 | 32 | 1/H/V/L | none | ? |   
[153](INES_Mapper_153.xhtml "INES Mapper 153") | [LZ93D50](Bandai_FCG_board.xhtml "Bandai FCG board") (RAM) | 16+16F | 512 | unbanked | 8 | 1/H/V | cycle | ext |   
[157](INES_Mapper_157.xhtml "INES Mapper 157") | [Bandai Datach](Bandai_FCG_board.xhtml "Bandai FCG board") | 16+16F | 256 | unbanked | 8 | 1/H/V | cycle | serial EEPROMs | barcode reader   
[158](INES_Mapper_158.xhtml "INES Mapper 158") | RAMBO-1 (nametable) | 3×8+8F | 256 | MMC3; 8×1 | 128 | arbitrary | scanline/cycle | none |   
[165](INES_Mapper_165.xhtml "INES Mapper 165") | MMC2 × MMC3 | MMC3 | 512 | 2×4 | 128 | ? | ? | ? | tile triggered CHR banking, CHR RAM and ROM   
[182](INES_Mapper_114.xhtml "INES Mapper 182") | for: Pocahontas | MMC3 | 256 (512?) | MMC3 | 256 | H/V | scanline | ? | ?   
[189](INES_Mapper_189.xhtml "INES Mapper 189") | MMC3 × [BNROM](INES_Mapper_034.xhtml "BNROM") | 32 | 512 | MMC3 | 256 | ? | ? | none |   
[191](INES_Mapper_191.xhtml "INES Mapper 191") | TQROM-like (2kiB) | MMC3 | 256 (512?) | MMC3 | 128 | 1/H/V? | scanline? | ext? | CHR RAM and ROM, ?like TQROM?   
[192](INES_Mapper_192.xhtml "INES Mapper 192") | TQROM-like (4kiB) | MMC3 | 512? | MMC3 | 252 | 1/H/V? | scanline? | ext? | CHR RAM and ROM, ?like TQROM?   
[193](INES_Mapper_193.xhtml "INES Mapper 193") | NTDEC TC-112 | 8+24F | 256 | 4+2+2 | 256 | fixed | none | none |   
[195](INES_Mapper_195.xhtml "INES Mapper 195") | Waixing FS303 | MMC3 | 512 | MMC3 | 256 | 1/H/V | scanline | ext | CHR RAM and ROM, like TQROM   
[207](INES_Mapper_207.xhtml "INES Mapper 207") | X1-005 (nametable) | 3×8+8F | 256 | N108 | 128 | 1/H | none | 128B int |   
[210](INES_Mapper_210.xhtml "INES Mapper 210") | [Namco 175](INES_Mapper_019.xhtml "Namco 163") | 3×8+8F | 512 | 8×1 | 256 | fixed | none | ext |   
[210](INES_Mapper_210.xhtml "INES Mapper 210") | [Namco 340](INES_Mapper_019.xhtml "Namco 163") | 3×8+8F | 512 | 8×1 | 256 | 1/H/V | none | none |   
[246](INES_Mapper_246.xhtml "INES Mapper 246") | for: Feng Shen Bang | 4×8 | 2048 | 4×2 | 512 | fixed | none | 2048B ext |   
[252](INES_Mapper_252.xhtml "INES Mapper 252")/[253](INES_Mapper_253.xhtml "INES Mapper 253") | Waixing VRC4 CHR ROM+RAM | MMC3 | 256? | 8×1 | 254? 508? | 1/H/V | cycle | ext? | CHR RAM and ROM   
(none) | [UNIF/UNL-DripGame](UNIF_UNL_DripGame.xhtml "UNIF/UNL-DripGame") | 16+16F | 256 | 4×2 | 32 | 1/H/V | cycle | ext | Expansion audio, 8x8 attribute zones   
  
## Mappers with outer banks

| PRG | CHR |   
---|---|---|---  
iNES Mapper | Short name | inner | outer | max (KiB) | inner | outer | max (KiB) | Comments   
[12](INES_Mapper_012.xhtml "INES Mapper 012") | for: dbz5 | MMC3 | — | 512 | MMC3 | 2×256 | 512 | ?like MMC3?   
[14](INES_Mapper_014.xhtml "INES Mapper 014") | MMC3 × VRC2 | MMC3 | — | 512 | MMC3; 8×1 | 3×256 | 512 | H/V, ext WRAM, Scanline IRQ, ?like MMC3?   
[28](Action_53_mapper.xhtml "INES Mapper 028") | [Action 53](Action_53_mapper.xhtml "Action 53 mapper") | 16+16F; 16F+16; 32 | 32…256 | 8192 | 8 | — | 32RAM | 1/H/V   
[37](INES_Mapper_037.xhtml "INES Mapper 037") | MMC3+'00+'161 | MMC3 | 64; 128 | 256 | MMC3 | 128 | 256 | H/V, Scanline IRQ   
[44](INES_Mapper_044.xhtml "INES Mapper 044") | for: Super Big 7-in-1 | MMC3 | 128; 256 | 1024 | MMC3 | 128; 256 | 1024 | ?like MMC3?   
[45](INES_Mapper_045.xhtml "INES Mapper 045") | for: Super 4-in-1 | MMC3 | 8…512 | 1024 (2048?) | MMC3 | 1…256 | 1024 (4096?) | ?like MMC3?   
[47](INES_Mapper_047.xhtml "INES Mapper 047") | MMC3+'161 | MMC3 | 128 | 256 | MMC3 | 128 | 256 | H/V, Scanline IRQ   
[49](INES_Mapper_049.xhtml "INES Mapper 049") | for: Super HIK 4-in-1 | MMC3; 32 | 128 | 512 | MMC3 | 128 | 512 | ?like MMC3?   
[52](INES_Mapper_052.xhtml "INES Mapper 052") | for: Mario 7-in-1 | MMC3 | 128; 256 | 1024 | MMC3 | 128; 256 | 1024 | ?like MMC3?   
[88](INES_Mapper_088.xhtml "INES Mapper 088")/[154](INES_Mapper_154.xhtml "INES Mapper 154") | N108 (pattern split) | N108 | — | 128 | 2×2/64 + 4×1/64 | 2×64 | 128 | none   
[115](INES_Mapper_115.xhtml "INES Mapper 115") | for: Thunderbolt 2 (ch) | MMC3; 16+16F; 16+8+8F | — | 128 (256?) | MMC3 | 256 | 512 | ?like MMC3?   
[116](INES_Mapper_116.xhtml "INES Mapper 116") | MMC1 × MMC3 × VRC2 | MMC3; 16+16F; 16F+16; 32 | — | 256 | MMC3; 2×4; 8×1 | 256 | 512 | 1/H/V, ?like MMC3?   
[138](Sachen_8259.xhtml "INES Mapper 138") | [Sachen 8259](Sachen_8259.xhtml "Sachen 8259") type B | 32 | — | 256 | 4×2 | 16 | 128 | 1/H/V/L   
[139](Sachen_8259.xhtml "INES Mapper 139") | [Sachen 8259](Sachen_8259.xhtml "Sachen 8259") type C | 32 | — | 256 | 4×2 | 32 | 256 | 1/H/V/L   
[141](Sachen_8259.xhtml "INES Mapper 141") | [Sachen 8259](Sachen_8259.xhtml "Sachen 8259") type A | 32 | — | 256 | 4×2 | 64 | 512 | 1/H/V/L   
[205](INES_Mapper_205.xhtml "INES Mapper 205") | for: 3-in-1 | MMC3 | 128; 256 | 512 | MMC3 | 128; 256 | 512 | ?like MMC3?   
[245](INES_Mapper_245.xhtml "INES Mapper 245") | MMC3 × [SUROM](MMC1.xhtml#Higher_CHR_lines "SxROM") | MMC3 | 512 | 1024 | unbanked | — | 8 | ?like MMC3?   
(none) | [UNIF/COOLBOY](NES_2_0_Mapper_268.xhtml "UNIF/COOLBOY") | MMC3; 16; 32 | 128…2048 | 32768 | MMC3; 8 | 128; 256 | 256RAM | like MMC3   
  
## Discrete logic mappers

Only the ones not made by Nintendo. 

This table describes the mappers as they existed, as opposed to any obvious oversize extensions. 

See also [Comparison of Nintendo mappers](Comparison_of_Nintendo_mappers.xhtml "Comparison of Nintendo mappers") and [User:Lidnariq/Discrete Logic Table](User_Lidnariq_Discrete_Logic_Table.xhtml "User:Lidnariq/Discrete Logic Table"). 

You probably don't actually want to use these. 

iNES | Chips | Max PRG (KiB) | PRG bank style | Max CHR (KiB) | CHR bank style | Mirroring | PRG RAM? | Bus conflicts?   
---|---|---|---|---|---|---|---|---  
[Color Dreams](Color_Dreams.xhtml "Color Dreams") (11, [144](INES_Mapper_144.xhtml "INES Mapper 144")) | 1 | 128 | 32 | 128 | 8 | V/H hardwired | No | Yes   
[29](INES_Mapper_029.xhtml "INES Mapper 029") (RET-CUFROM) | 3 | 128 | 16+16F | 32 | 8 | V hardwired | Yes | No   
RET-[UNROM 512](UNROM_512.xhtml "UNROM 512") (30) | 3 or 4 | 512 | 16+16F | 32RAM | 8 | 1 or V/H hardwired | No | Optional (extra IC)   
[NINA-001](INES_Mapper_034.xhtml "NINA-001") ([34](INES_Mapper_034.xhtml "INES Mapper 034")) | 6 | 64 | 32 | 64 | 4+4 | V hardwired | Yes | No   
[38](INES_Mapper_038.xhtml "INES Mapper 038") | 2 | 128 | 32 | 32 | 8 | V/H hardwired | Impossible | No   
[40](INES_Mapper_040.xhtml "INES Mapper 040") / [50](INES_Mapper_050.xhtml "INES Mapper 050") (w/ IRQ) | 7-8 | 128 | 24F+8+8F | 8 |  | V/H hardwired | Impossible | No   
[41](INES_Mapper_041.xhtml "INES Mapper 041") | 7 | 256 | 32 | 128 | 8 inner / 32 outer | V/H switchable | Impossible | Partly   
[70](INES_Mapper_070.xhtml "INES Mapper 070") | 3 | 256 | 16+16F | 128 | 8 | V/H hardwired | No | Likely   
[72](INES_Mapper_072.xhtml "INES Mapper 072") | 4+speech | 256 | 16+16F | 128 | 8 | V/H hardwired | No | Yes   
[77](INES_Mapper_077.xhtml "INES Mapper 077") | 4 | 512 | 32 | 32+6RAM | 2+6RAM | 4 | No | Likely   
[78.1](INES_Mapper_078.xhtml "INES Mapper 078") | 3 | 128 | 16+16F | 128 | 8 | 1 | No | Likely   
[78.3](INES_Mapper_078.xhtml "INES Mapper 078") | 5 | 128 | 16+16F | 128 | 8 | V/H switchable | No | Yes   
[79](NINA_003_006.xhtml "INES Mapper 079") ([146](NINA_003_006.xhtml "INES Mapper 146")) | 2 | 64 | 32 | 64 | 8 | V/H hardwired | No | No   
[86](INES_Mapper_086.xhtml "INES Mapper 086") | 3+speech | 128 | 32 | 64 | 8 | V/H hardwired | Impossible | No   
[87](INES_Mapper_087.xhtml "INES Mapper 087") ([101](INES_Mapper_101.xhtml "INES Mapper 101")) | 2 | 32 |  | 32 | 8 | V/H hardwired | Impossible | No   
[89](INES_Mapper_089.xhtml "INES Mapper 089") | 2† | 128 | 16+16F | 128 | 8 | 1 | No | Yes   
[92](INES_Mapper_092.xhtml "INES Mapper 092") | 5+speech | 256 | 16F+16 | 128 | 8 | V/H hardwired | No | Yes   
[93](INES_Mapper_093.xhtml "INES Mapper 093") | 2† | 128 | 16+16F | 8‡ |  | V/H hardwired | No | Yes   
[94](INES_Mapper_094.xhtml "INES Mapper 094") | 2 | 128 | 16+16F | 8 |  | V/H hardwired | No | Yes   
[96](INES_Mapper_096.xhtml "INES Mapper 096") | 3 | 128 | 32 | 32RAM | 4+4F inner / 16 outer | V/H hardwired | No | Likely   
[99](INES_Mapper_099.xhtml "INES Mapper 099") | 0* | 40 | 8+24F | 16 | 8 | 4 | No | No   
[107](INES_Mapper_107.xhtml "INES Mapper 107") | 1? | 128 | 32 | 64 | 8 | V hardwired | No | Likely   
[133](INES_Mapper_133.xhtml "INES Mapper 133") | 2? | 64 | 32 | 32 | 8 | V/H hardwired | No | No   
[140](INES_Mapper_140.xhtml "INES Mapper 140") | 3 | 128 | 32 | 128 | 8 | V/H hardwired | Impossible | No   
[143](INES_Mapper_143.xhtml "INES Mapper 143") | 2? | 32 |  | 8 |  | V/H hardwired | No | No   
[145](INES_Mapper_145.xhtml "INES Mapper 145") | 2? | 32 |  | 16 | 8 | V/H hardwired | No | No   
[147](INES_Mapper_147.xhtml "INES Mapper 147") | 2? | 128 | 32 | 128 | 8 | V/H hardwired | No | No   
[148](INES_Mapper_148.xhtml "INES Mapper 148") | 1? | 64 | 32 | 64 | 8 | V/H hardwired | No | Yes   
[149](INES_Mapper_149.xhtml "INES Mapper 149") | 1? | 32 |  | 16 | 8 | V/H hardwired | No | Yes   
[152](INES_Mapper_152.xhtml "INES Mapper 152") | 3 | 128 | 16+16F | 128 | 8 | 1 | No | Likely   
[168](INES_Mapper_168.xhtml "INES Mapper 168") (w/ IRQ) | 7 | 64 | 16+16F | 64RAM | 4F+4 | V hardwired | No | No   
[174](INES_Mapper_174.xhtml "INES Mapper 174") | 3 | 128 | 16 or 32 | 64 | 8 | V/H switchable | No | No   
[184](INES_Mapper_184.xhtml "INES Mapper 184") | 3† | 32 |  | 32 | 4+4 | V/H hardwired | Impossible | No   
[185](CNROM.xhtml "INES Mapper 185") | 1 | 32 |  | 8 |  | V/H hardwired | No | Yes   
[218](INES_Mapper_218.xhtml "INES Mapper 218") | -1§ | 32 |  | 1RAM |  | 1 hardwired | No | No   
  
† Mappers [89](INES_Mapper_089.xhtml "INES Mapper 089"), [93](INES_Mapper_093.xhtml "INES Mapper 093"), and [184](INES_Mapper_184.xhtml "INES Mapper 184") exist as a single IC, however their functions are trivially described using a small number of 7400-series ICs, and likely contain multiple silicon dice that were wire bonded together in the same package. 

‡ Mapper [93](INES_Mapper_093.xhtml "INES Mapper 093") is technically the same [89](INES_Mapper_089.xhtml "INES Mapper 089") other than mirroring, but it only commercially existed using 8kB of CHR-RAM 

* the Vs System distributed its original games as five or six 8 KiB ROMs, and decoding on its mainboard allowed banking of CHR like [CNROM](CNROM.xhtml "CNROM"). It is a little disingenuous to claim that 0 ICs were necessary for banking since the same functionality is not possible on a Famicom, however, banking was incrementally free. 

§ Doesn't have CHRROM, hence "-1" ICs 
