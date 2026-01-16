# NES 2.0 submappers

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_submappers) | View [other pages](Special_AllPages.xhtml#NES_2_0_submappers)

**Submapper** is a term used in the [NES 2.0](NES_2_0.xhtml "NES 2.0") header for 4-bit codes designating functionally distinct variants of iNES mappers that cannot be distinguished by the memory size fields alone. Most emulators using [iNES](INES.xhtml "INES") format distinguish these using CRC, SHA-1, or other hashes of the PRG ROM and CHR ROM, but this works only for games published prior to 1997, not for fan translations or ROM hacks, and not for new games on the same mapper. 

Submapper 0 represents the default iNES behavour, so that backward compatibility is maintained with existing ROMs. 

Submapper allocations that are listed as "deprecated" were assigned by kevtris' original proposal, but have no known use cases. The deprecation reserves these unused allocations to maintain continuity and compatibility. 

**This document is a living specification. Proposals for new submappers should be made at:** [Proposals](NES_2_0_submappers_Proposals.xhtml "NES 2.0 submappers/Proposals"). 

## Contents

  * 1 001: MMC1
    * 1.1 001: 0
    * 1.2 001: 1, 2, 4 = SUROM, SOROM, SXROM
    * 1.3 001: 3
    * 1.4 001: 5 Fixed PRG
    * 1.5 001: 6 = 2ME
  * 2 002, 003, 007: UxROM, CNROM, AxROM
    * 2.1 002: 0, 1, 2 UxROM
    * 2.2 003: 0, 1, 2 CNROM
    * 2.3 007: 0, 1, 2 AxROM
  * 3 004: MMC3
    * 3.1 004: 0 Sharp MMC3
    * 3.2 004: 1 MMC6
    * 3.3 004: 2
    * 3.4 004: 3 MC-ACC
    * 3.5 004: 4 NEC MMC3
    * 3.6 004: 5 T9552
  * 4 016: Bandai FCG board
  * 5 019: Namco 129/163
  * 6 021, 023, 025: VRC2 / VRC4
    * 6.1 Submapper assignment
    * 6.2 021 / 023 / 025: 0
    * 6.3 021 / 023 / 025: 1, 2 VRC4
    * 6.4 023: 1 VRC4f
    * 6.5 023: 3 VRC2b
    * 6.6 025: 3 VRC2c
    * 6.7 Test ROMs
  * 7 032: Irem G101
    * 7.1 032: 0
    * 7.2 032: 1 Major League
  * 8 034: BNROM / NINA-001
    * 8.1 034: 0
    * 8.2 034: 1 NINA-001
    * 8.3 034: 2 BNROM
  * 9 068: Sunsoft 4
    * 9.1 068: 0
    * 9.2 068: 1 Dual Cartridge System
  * 10 071: Codemasters
    * 10.1 071: 0
    * 10.2 071: 1 Fire Hawk
  * 11 078: Cosmo Carrier / Holy Diver
    * 11.1 078: 0
    * 11.2 078: 1 Cosmo Carrier
    * 11.3 078: 2
    * 11.4 078: 3 Holy Diver
  * 12 085: Konami VRC7
  * 13 091: Super Fighter III
  * 14 114: Sugar Softec/Hosenkan, fixed MMC3 scrambling pattern
  * 15 178.1: Gameinis Infrared Sensor
  * 16 185: CNROM used to selectively disable CHR-ROM
    * 16.1 185: 0
    * 16.2 185: 4
    * 16.3 185: 5
    * 16.4 185: 6
    * 16.5 185: 7
  * 17 206: Namco 118 PCB variants
    * 17.1 206: 0
    * 17.2 206: 1
  * 18 210: Namco 175 and 340
    * 18.1 210: 0
    * 18.2 210: 1 N175
    * 18.3 210: 2 N340
  * 19 215: Sugar Softec, selectable MMC3 scrambling pattern
  * 20 232: Quattro
    * 20.1 232: 0
    * 20.2 232: 1 Aladdin Deck Enhancer
  * 21 NES 2.0 Mapper 256
  * 22 268: Coolboy/Mindkids
  * 23 References



## [001](MMC1.xhtml "INES Mapper 001"): [MMC1](MMC1.xhtml "MMC1")

Most [MMC1 boards](MMC1.xhtml "SxROM") are compatible with the standard mapper 1 behavior (submapper 0). 

Boards with CHR-RAM usually reuse the CHR banking lines to address other things. 

  * SUROM, SOROM, and SXROM implement extra PRG-ROM and PRG-RAM banking (_deprecated_ submappers 1, 2, 4).
  * SNROM implements a redundant PRG-RAM enable (no assigned submapper).



The variant MMC1A chip was assigned to [mapper 155](MMC1.xhtml "INES Mapper 155") (_deprecated_ submapper 3). 

Most boards with 32k of PRG-ROM have no PRG banking: SEROM, SHROM, SH1ROM (submapper 5). 

### 001: 0

Normal behavior. 

### 001: 1, 2, 4 = SUROM, SOROM, SXROM

**Deprecated.**

These submappers are a redundancy. The difference between these boards and the "normal" MMC1 implementation is exclusively dependent on the sizes of CHR, PRG RAM, and PRG ROM. Because of this, the addition of PRG RAM to the NES 2.0 specification was enough to make these compatible with submapper 0. 

These three boards used 8k CHR RAM, and reused the CHR banking bits to bank PRG ROM and RAM instead. The specific boards can be detected by these sizes, or simply emulated together in submapper 0: 
    
    
    $A000 and $C000:
    4bit0
    -----
    EDCBA
    |||||
    ||||+- CHR A12
    |||+-- CHR A13 if CHR >= 16k
    ||+--- CHR A14 if CHR >= 32k; and PRG RAM A13 if PRG RAM = 32k (submapper 4)
    |+---- CHR A15 if CHR >= 64k; and PRG RAM A13 if PRG RAM = 16k (submapper 2)
    |                              or PRG RAM A14 if PRG RAM = 32k (submapper 4)
    +----- CHR A16 if CHR = 128k; and PRG ROM A18 if PRG ROM = 512k (submappers 1, 4)
    

The following games are on SUROM (submapper 1): 

  * _[Dragon Warrior III](http://bootgod.dyndns.org:7777/profile.php?id=1273)_
  * _[Dragon Warrior IV](http://bootgod.dyndns.org:7777/profile.php?id=1276)_ / _[Dragon Quest IV](http://bootgod.dyndns.org:7777/profile.php?id=1526)_
  * _[Ninjara Hoi!](http://bootgod.dyndns.org:7777/profile.php?id=1531)_



SOROM (submapper 2): 

  * _[Genghis Khan](http://bootgod.dyndns.org:7777/profile.php?id=919)_ / _[Aoki Ookami to Shiroki Mejika: Genghis Khan](http://bootgod.dyndns.org:7777/profile.php?id=1528)_
  * _[Nobunaga's Ambition](http://bootgod.dyndns.org:7777/profile.php?id=497)_ / _[Nobunaga no Yabou: Zenkokuban](http://bootgod.dyndns.org:7777/profile.php?id=1529)_
  * _[Romance of the Three Kingdoms](http://bootgod.dyndns.org:7777/profile.php?id=730)_ / _[Sangokushi](http://bootgod.dyndns.org:7777/profile.php?id=3170)_



SXROM (submapper 4): 

  * _[Final Fantasy I& II](http://bootgod.dyndns.org:7777/profile.php?id=3642)_
  * _[The Best Play Pro Yakyuu Special](http://bootgod.dyndns.org:7777/profile.php?id=3757)_
  * _[PR8](http://forums.nesdev.org/viewtopic.php?f=22&t=7231)_. If extra RAM is missing, it gives a ["DOES NOT WORK IN THIS EMULATOR" error](http://forums.nesdev.org/viewtopic.php?f=2&t=7629).



If any NES 2.0 ROMs are found using these deprecated submappers, the CHR, PRG RAM, and PRG ROM sizes must appropriately match to be a valid header. 

### 001: 3

**Deprecated.**

This originally described a submapper that was already implemented as [iNES Mapper 155](MMC1.xhtml "INES Mapper 155"). 

### 001: 5 Fixed PRG

[SEROM](SxROM.xhtml "SEROM"), [SHROM](SxROM.xhtml "SHROM"), [SH1ROM](https://www.nesdev.org/w/index.php?title=SH1ROM&action=edit&redlink=1 "SH1ROM \(page does not exist\)") use a fixed 32k PRG ROM with no banking support. (This is distinct from [SIROM](https://www.nesdev.org/w/index.php?title=SIROM&action=edit&redlink=1 "SIROM \(page does not exist\)") which has 32k of bankable PRG ROM.) 

PRG ROM A14 is connected directly to CPU A14 (and MMC1 A14 input) instead of MMC1 A14 output. 

Existing games are compatible with submapper 0 if $8000-BFFF is initialized to the low bank, and $C000-FFFF is initialized to the high bank. These boards were used in several games: [SEROM](http://bootgod.dyndns.org:7777/search.php?keywords=serom&kwtype=pcb) [SHROM](http://bootgod.dyndns.org:7777/search.php?keywords=shrom&kwtype=pcb) [SH1ROM](http://bootgod.dyndns.org:7777/search.php?keywords=sh1rom&kwtype=pcb)

Test ROM: 

  * _[serom.zip](http://forums.nesdev.org/download/file.php?id=3753)_ \- validates non-existence of PRG banking control. ([forum post](http://forums.nesdev.org/viewtopic.php?p=153298#p153298))



### 001: 6 = 2ME

The 2ME board is a Famicom Network System MMC1B variant used by _JRA-PAT_ revisions FCN027-04 and later. It supports 64 big-endian 16-bit words of EEPROM and up to 32 KiB of battery-backed PRG-RAM banked like SXROM. Because the Network System does not expose the PPU bus to the software card, all CHR-related outputs are repurposed to support this storage. See [MMC1 2ME](MMC1.xhtml#2ME "MMC1") for full details. 

## [002](UxROM.xhtml "INES Mapper 002"), [003](CNROM.xhtml "INES Mapper 003"), [007](AxROM.xhtml "INES Mapper 007"): [UxROM](UxROM.xhtml "UxROM"), [CNROM](CNROM.xhtml "CNROM"), [AxROM](AxROM.xhtml "AxROM")

Mappers 2, 3, and 7 describe [discrete logic mappers](Category_Discrete_logic_mappers.xhtml "Category:Discrete logic mappers") that are usually subject to [bus conflicts](Bus_conflict.xhtml "Bus conflict"). Most of these games are programmed in a way that does not rely on the bus conflict behaviour, but software bugs may expose the difference (e.g. _Cybernoid_[1]). Because these differences are obscure, the default iNES implementation for mappers 2 and 3 has been inconsistent across emulators, some with bus conflicts, some without. Mapper 7 is normally emulated without bus conflicts, because of a small number of games using the [ANROM](AxROM.xhtml "ANROM") board which rely on this. 

Two submappers allow this ambiguity to be resolved, by selecting a specific behaviour: 

0: Default iNES behaviour (Emulators should warn the user and/or enforce bus conflicts for mappers 2 and 3, and should warn the user and/or fail to enforce bus conflicts for mapper 7)  
1: Bus conflicts do not occur  
2: Bus conflicts occur, producing the bitwise AND of the written value and the value in ROM 

Although all Nintendo-manufactured games using normal CNROM (mapper 3), normal UxROM (mapper 2), and inverted UxROM (mapper 180) had bus conflicts, apparently several unlicensed games require their absence, as does the updated version of _Donkey Kong_ with the pie factory level[2]. 

AxROM (mapper 7) is the only known licensed discrete logic mapper to unreliably come with [bus conflict](Bus_conflict.xhtml "Bus conflict") prevention circuitry. While no game documented in NesCartDB was released in one region on multiple board variants, several games did change boards when localized. 

New ROMs are encouraged to specify submapper 2 to verify correct behaviour with bus conflicts. 

CNROM with security diodes (mapper 185) has a different set of submapper definitions. 

### 002: 0, 1, 2 [UxROM](UxROM.xhtml "UxROM")

Test ROMs: ([forum post](https://forums.nesdev.org/viewtopic.php?p=157804#p157804)) 

  * _2_test_0.nes_ \- Unspecified.
  * _2_test_1.nes_ \- No bus conflicts.
  * _2_test_2.nes_ \- AND bus conflicts.



### 003: 0, 1, 2 [CNROM](CNROM.xhtml "CNROM")

Test ROMs: ([forum post](https://forums.nesdev.org/viewtopic.php?p=154555#p154555)) 

  * _3_test_0.nes_ \- Unspecified.
  * _3_test_1.nes_ \- No bus conflicts.
  * _3_test_2.nes_ \- AND bus conflicts.



### 007: 0, 1, 2 [AxROM](AxROM.xhtml "AxROM")

Test ROMs: ([forum post](https://forums.nesdev.org/viewtopic.php?p=157804#p157804)) 

  * _7_test_0.nes_ \- Unspecified.
  * _7_test_1.nes_ \- No bus conflicts.
  * _7_test_2.nes_ \- AND bus conflicts.



## [004](MMC3.xhtml "INES Mapper 004"): [MMC3](MMC3.xhtml "MMC3")

[iNES Mapper 004](MMC3.xhtml "INES Mapper 004") represents the most common boards using these four ICs: early [MMC3](MMC3.xhtml "MMC3"), late MMC3, MC-ACC, and [MMC6](MMC6.xhtml "MMC6"). 

There are three known kinds of IRQ: 

  1. NEC MMC3: IRQ is asserted on A12 rise, and loading the latch with 0 disables IRQ. At least one unlicensed game relies on this "old style" behavior.
  2. Sharp MMC3: IRQ is asserted on A12 rise, and loading the latch with 0 produces an IRQ on every scanline. MMC6 also behaves this way. Some later games rely on this "new style" behavior.
  3. MC-ACC: IRQ is asserted on A12 fall, typically four pixels later than Sharp MMC3. Interrupts can be produced every scanline, like the Sharp MMC3.[3]



There are two known kinds of PRG RAM enable: 

  1. MMC3: One set of enable bits controls the entire chip.
  2. MMC6: The first and second enables control the first and second half of PRG RAM, and an additional enable in bit 5 of $8000 controls the whole PRG RAM.



The [TEROM and TFROM](TxROM.xhtml "TxROM") boards have two jumpers that can respectively disable IRQs and force hard-wired mirroring. It is believed that nothing was ever released that used them. 

### 004: 0 Sharp MMC3

Normal. 

  * _Star Trek 25th Anniversary_



If the header is not using [NES 2.0](NES_2_0.xhtml "NES 2.0"), an implementation without PRG-RAM write protection might be desired instead of a full MMC3. See: [iNES Mapper 004 and MMC6](MMC3.xhtml#iNES_Mapper_004_and_MMC6 "MMC3")

### 004: 1 MMC6

[MMC6](MMC6.xhtml "MMC6") has an alternative PRG-RAM enable and write protection scheme designed for its internal 1k PRG RAM. Uses Sharp MMC3 IRQ behavior. 

  * _StarTropics_
  * _StarTropics 2_



### 004: 2

This describes MMC3C with hard-wired mirroring. No licensed games are known to require this, but a number of hacks and plug-and-play console extracts depend on it, and it is a valid configuration of the NES-TFROM circuit board. 

### 004: 3 MC-ACC

MC-ACC 

The MC-ACC is found in [13 second-source PCBs manufactured by Acclaim](http://bootgod.dyndns.org:7777/search.php?keywords=MC-ACC&kwtype=pcb): 

  * _Alien³_
  * _George Foreman's KO Boxing_
  * _The Incredible Crash Dummies_
  * _Mickey's Safari in Letterland_
  * _Roger Clemens' MVP Baseball_
  * _Rollerblade Racer_
  * _The Simpsons: Bart vs. The World_
  * _The Simpsons: Bartman Meets Radioactive Man_
  * _Spider-Man: Return of the Sinister Six_
  * _T &C Surf Designs 2: Thrilla's Surfari_
  * _T2: Terminator 2: Judgment Day_
  * _WWF King of the Ring_
  * _WWF WrestleMania: Steel Cage Challenge_



### 004: 4 NEC MMC3

Loading the latch with 0 disables IRQ. 

  * SuperGame's _Aladdin_ (Mapper 4 version)



### 004: 5 T9552

Indicates the presence of a [T9552](T9552.xhtml#ROM_file_bank_order "T9552") scrambling chip. 

## [016](INES_Mapper_016.xhtml "INES Mapper 016"): Bandai FCG board

[INES Mapper 016 submapper table](INES_Mapper_016_Submapper_table.xhtml "INES Mapper 016/Submapper table")  
---  
Submapper # | Meaning | Note   
0 | Unspecified | Emulate both FCG-1/2 and LZ93D50 chips in their respective CPU address ranges.   
_1_ | _LZ93D50 with 128 byte serial EEPROM (24C01)_ | _Deprecated, use[INES Mapper 159](INES_Mapper_159.xhtml "INES Mapper 159") instead._  
_2_ | _Datach Joint ROM System_ | _Deprecated, use[INES Mapper 157](INES_Mapper_157.xhtml "INES Mapper 157") instead._  
_3_ | _8 KiB of WRAM instead of serial EEPROM_ | _Deprecated, use[INES Mapper 153](INES_Mapper_153.xhtml "INES Mapper 153") instead._  
4 | FCG-1/2 | Responds only in the CPU $6000-$7FFF address range; IRQ counter is not latched.   
5 | LZ93D50 with no or 256-byte serial EEPROM (24C02) | Responds only in the CPU $8000-$FFFF address range; IRQ counter is latched.   
  
## [019](INES_Mapper_019.xhtml "INES Mapper 019"): Namco 129/163

The NES 2.0 submapper is used to specify the mixing resistor that determines the relative volume of the expansion audio channels against the APU's audio channels: 

[INES Mapper 019 submapper table](INES_Mapper_019_Submapper_table.xhtml "INES Mapper 019/Submapper table")  
---  
Submapper # | Meaning | Note   
0 | Default | Expansion sound volume unspecified   
_1_ | _Deprecated_ | Internal 128b RAM is battery backed, no external PRG-RAM is present. No expansion sound. (Equivalent to submapper 2 with 0 in PRG-NVRAM field.)   
2 | No expansion sound |   
3 | N163 expansion sound: **11.0-13.0 dB** louder than NES APU |   
4 | N163 expansion sound: **16.0-17.0 dB** louder than NES APU |   
5 | N163 expansion sound: **18.0-19.5 dB** louder than NES APU |   
  
## [021](VRC2_and_VRC4.xhtml "INES Mapper 021"), [023](VRC2_and_VRC4.xhtml "INES Mapper 023"), [025](VRC2_and_VRC4.xhtml "INES Mapper 025"): [VRC2](VRC2_and_VRC4.xhtml "VRC2") / [VRC4](VRC2_and_VRC4.xhtml "VRC4")

These three mappers collect various configurations of [VRC2](VRC2_and_VRC4.xhtml "VRC2") and [VRC4](VRC2_and_VRC4.xhtml "VRC4") boards. 

VRC2 is mostly a subset of VRC4, with differences including: 

  * VRC2 has a serial EEPROM interface which no Konami game ever connected an actual EEPROM to, but one PCB is wired so that it functions as a single bit of memory mapped at $6000-6FFF[4]
  * VRC4 supports horizontal, vertical and one-screen nametable mirroring while VRC2 supports only horizontal and vertical mirroring[5]
  * VRC4 supports two PRG ROM banking modes, similar to MMC3's. VRC2 only has one PRG ROM banking mode.
  * VRC4 has an interrupt device that VRC2 does not.



Additionally, different boards connect the address lines for the registers in various arrangements. The three iNES mappers 21, 23, and 25 each combine multiple boards whose addresses overlap. This is enough for game compatibility, but it creates an ugly combination that does not accurately describe the original hardware of either board. 

### Submapper assignment

Konami's [VRC2 and VRC4](VRC2_and_VRC4.xhtml "VRC2 and VRC4") mappers have several variations of how the board connects low CPU address lines among A7-A0 to the port select lines of the mapper. Mappers [21](VRC2_and_VRC4.xhtml "INES Mapper 021"), [23](VRC2_and_VRC4.xhtml "INES Mapper 021") and [25](VRC2_and_VRC4.xhtml "INES Mapper 021") each combine two non-overlapping addressing variations, and neglect to specify VRC2 vs VRC4. 

Mapper [22](VRC2_and_VRC4.xhtml "INES Mapper 021") is related, but only supports one variation (VRC2a) and has a different CHR banking arrangement than the others. It does not require submapper disambiguation. 

There are three variations of the [VRC2](VRC2_and_VRC4.xhtml "VRC2") boards, and five of [VRC4](VRC2_and_VRC4.xhtml "VRC4"): 

VRC2   
---  
Nickname | PCB | A0 | A1 | Registers | iNES mapper | submapper   
VRC2a | 351618 | A1 | A0 | $x000, $x002, $x001, $x003 | 22 | 0   
VRC2b | many† | A0 | A1 | $x000, $x001, $x002, $x003 | 23 | 3   
VRC2c | 351948 | A1 | A0 | $x000, $x002, $x001, $x003 | 25 | 3   
VRC4   
---  
Nickname | PCB | A0 | A1 | Registers | iNES mapper | submapper   
VRC4a | 352398 | A1 | A2 | $x000, $x002, $x004, $x006 | 21 | 1   
VRC4b | 351406 | A1 | A0 | $x000, $x002, $x001, $x003 | 25 | 1   
VRC4c | 352889 | A6 | A7 | $x000, $x040, $x080, $x0C0 | 21 | 2   
VRC4d | 352400 | A3 | A2 | $x000, $x008, $x004, $x00C | 25 | 2   
VRC4e | 352396 | A2 | A3 | $x000, $x004, $x008, $x00C | 23 | 2   
VRC4f | - | A0 | A1 | $x000, $x001, $x002, $x003 | 23 | 1   
  
    †  The VRC2b PCBs include: 350603, 350636, 350926, 351179

This submapper assigment uses the following symmetrical arrangement: 

  * 0\. Both addressing
  * 1\. Lower addressing, VRC4
  * 2\. Higher addressing, VRC4
  * 3\. Lower addressing, VRC2 _(no known examples for mapper 21)_
  * 4\. _Higher addressing, VRC2 (no known examples)_



### 021 / 023 / 025: 0

The default implementation acts as a VRC4 (mostly compatible superset of VRC2), and responds to register writes in one or more configurations simultaneously (supporting both boards at once). 

### 021 / 023 / 025: 1, 2 VRC4

These allocations each request a single specific addressing scheme for VRC4, rather than the combined version used by submapper 0. 

  * 021: 1 VRC4a
  * 025: 1 VRC4b
  * 021: 2 VRC4c
  * 025: 2 VRC4d
  * 023: 2 VRC4e
  * 023: 1 VRC4f



### 023: 1 VRC4f

Some unlicensed games used a second-source VRC4 clone with register addressing identical to VRC2b (the simplest contiguous in-order interpretation: 0,1,2,3). This was used in _World Hero_. 

### 023: 3 VRC2b

Games on PCB 350926 (and its chip-on-board equivalents) rely on a single bit of RAM mapped in the region from $6000-$6FFF. Prior to NES 2.0, these games were supported by implementing WRAM in this region (despite this board not having WRAM). This board was used in _[Contra (J)](http://bootgod.dyndns.org:7777/profile.php?id=3986)_ , _[Ganbare Goemon 2](http://bootgod.dyndns.org:7777/profile.php?id=1568)_ and _[Konami Wai Wai World](http://bootgod.dyndns.org:7777/profile.php?id=2274)_. 

There are other VRC2 boards with the same register addressing as 350926 but without this feature ($6000-$6FFF is effectively open bus on them), but no game relies on the _lack_ of the single bit of memory, so a separate submapper has not been allocated for them. These boards were used in _[Dragon Scroll](http://bootgod.dyndns.org:7777/profile.php?id=3802)_ , _[Getsu Fuuma Den](http://bootgod.dyndns.org:7777/profile.php?id=2276)_ , and _[Jarinko Chie](http://bootgod.dyndns.org:7777/profile.php?id=2278)_. 

Since this submapper represents VRC2, it should not implement the VRC4-only one-screen mirroring, PRG ROM banking modes, or IRQ capabilities. Wai Wai World depends on the lack of one-screen mirroring; it will have corrupt backgrounds in the vertically-scrolling stages and the ending if emulated with VRC4 capabilities. 

### 025: 3 VRC2c

_[Ganbare Goemon Gaiden: Kieta Ougon Kiseru](http://bootgod.dyndns.org:7777/profile.php?id=3823)_ uses the same register addressing as VRC4b, but is VRC2, so this submapper should not implement VRC4-only capabilities. 

### Test ROMs

  * [VRC2/4 test program](https://forums.nesdev.org/viewtopic.php?p=203716#p203716)



## [032](INES_Mapper_032.xhtml "INES Mapper 032"): Irem G101

A variation of this mapper requires hardwired one-screen mirroring and entirely ignores writes to $9000. 

### 032: 0

Normal (H/V mapper-controlled mirroring) 

### 032: 1 Major League

CIRAM A10 is tied high (fixed one-screen mirroring) and PRG banking style is fixed as 8+8+16F 

  * _[Major League](http://bootgod.dyndns.org:7777/profile.php?id=3800)_



## [034](INES_Mapper_034.xhtml "INES Mapper 034"): [BNROM](INES_Mapper_034.xhtml "BNROM") / [NINA-001](INES_Mapper_034.xhtml "NINA-001")

This iNES mapper unfortunately combines the unrelated [BNROM](INES_Mapper_034.xhtml "BNROM") and [NINA-001](INES_Mapper_034.xhtml "NINA-001") mappers. 

### 034: 0

Normal. 

To disambiguate the two mappers, emulators have taken various approaches: 

  * The presense of CHR larger than 8 KiB unambiguously requires NINA-001, as BNROM has no CHR banking.
  * The presence of CHR-RAM is taken to imply BNROM, because both extant BNROM games use CHR-RAM.
  * CRC tests may be used to select a mapper for previously known ROMs.
  * Implement both mappers simultaneously. This is compatible with existing games.



Selecting a single implementation based on CHR results in greater accuracy, since no game was ever intended for the combined definition. 

New programs should not attempt to use a combined BNROM + NINA-001 mapper because this is not reliably available across emulators. Unusual combinations like NINA-001 with CHR-RAM are theoretically possible, but unlikely to be emulated consistently. 

### 034: 1 NINA-001

[NINA-001](INES_Mapper_034.xhtml "NINA-001") only. 

Test ROM: ([forum post](https://forums.nesdev.org/viewtopic.php?p=153334#p153334)) 

  * _34_test_1.nes_ \- NINA-001 with CHR ROM.



### 034: 2 BNROM

[BNROM](INES_Mapper_034.xhtml "BNROM") only. 

Some unlicensed boards by Union Bond were a variation of BNROM that included PRG RAM. These may also use this submapper if PRG RAM is specified in the NES 2.0 header. 

Test ROM: ([forum post](https://forums.nesdev.org/viewtopic.php?p=153334#p153334)) 

  * _34_test_2.nes_ \- BNROM with CHR RAM.



## [068](INES_Mapper_068.xhtml "INES Mapper 068"): Sunsoft 4

In addition to its normal function, the Sunsoft 4 IC was used in _Nantettatte!! Baseball_ , which allowed a second expansion cartridge to be plugged into it. 

### 068: 0

0: Normal (max 256KiB PRG) 

### 068: 1 Dual Cartridge System

1: Sunsoft Dual Cartridge System a.k.a. NTB-ROM (max 128KiB PRG, licensing IC present, external option ROM of up to 128KiB should be selectable by a second menu) 

  * [_Nantettatte!! Baseball_](http://bootgod.dyndns.org:7777/profile.php?id=2265)



## [071](INES_Mapper_071.xhtml "INES Mapper 071"): Codemasters

Some games use this with 1-screen [mirroring](Mirroring.xhtml "Mirroring"), where the mapper's mirroring control bit is wired directly to CIRAM A10. Others have hardwired horizontal or vertical mirroring. 

Another variation of this mapper was used in the _Quattro_ multicarts, but these have been reassigned to [mapper 232](INES_Mapper_232.xhtml "INES Mapper 232"). 

### 071: 0

Hardwired horizontal or vertical mirroring. 

### 071: 1 Fire Hawk

Mapper controlled single-screen mirroring. 

  * _[Fire Hawk](http://bootgod.dyndns.org:7777/profile.php?id=733)_



## [078](INES_Mapper_078.xhtml "INES Mapper 078"): Cosmo Carrier / Holy Diver

This mapper unfortunately combines two games with incompatible mirroring control. 

One game uses this with 1-screen [mirroring](Mirroring.xhtml "Mirroring"), where the mapper's mirroring control bit is wired directly to CIRAM A10. The other can switch between horizontal and vertical mirroring, using a multiplexer between PPU A10 and PPU A11 whose output is sent to CIRAM A10. 

Historically, pre-NES 2.0 ROM image headers set the 4-screen VRAM flag for Holy Diver and cleared it for Cosmo Carrier. 

### 078: 0

Unspecified. 

### 078: 1 Cosmo Carrier

Single-screen mirroring (nibble-swapped [mapper 152](INES_Mapper_152.xhtml "INES Mapper 152")). 

  * _[Uchuusen: Cosmo Carrier](http://bootgod.dyndns.org:7777/profile.php?id=3968)_



### 078: 2

**Deprecated.**

This described a variation with fixed vertical mirroring, and WRAM. There is no known use case. 

### 078: 3 Holy Diver

Mapper-controlled H/V mirroring. 

  * _[Holy Diver](http://bootgod.dyndns.org:7777/profile.php?id=4038)_



## [085](VRC7.xhtml "INES Mapper 085"): Konami VRC7

  * VRC7b uses A3 to select registers ($x008), used in Tiny Toon Adventures (Submapper 1).
  * VRC7a uses A4 to select registers ($x010), used in Lagrange Point (Submapper 2).



## [091](INES_Mapper_091.xhtml "INES Mapper 091"): Super Fighter III

  * Submapper 0: YY830624C/JY830848C (J.Y. Company clone board)
  * Submapper 1: EJ-006-1 (original _Super Fighter III_ board).



## [114](INES_Mapper_114.xhtml "INES Mapper 114"): Sugar Softec/Hosenkan, fixed MMC3 scrambling pattern

Registers at $6000 (and $6001, although not used for effect by existing games) 

  * Submapper 0: The Lion King, Aladdin
  * Submapper 1: Boogerman (different scrambling pattern)



## [178](INES_Mapper_178.xhtml "INES Mapper 178").1: Gameinis Infrared Sensor

  * Submapper 0: No infrared sensor present
  * Submapper 1: Gameinis Infrared Sensor present



## [185](CNROM.xhtml "INES Mapper 185"): CNROM used to selectively disable CHR-ROM

A few NROM-like games were released on CNROM boards where all four bits of the latch were solely used as an anti-piracy measure. Games would check that CHR-ROM is disabled unless the latch holds the correct one of the four possible bank values. See this [thread](http://forums.nesdev.org/viewtopic.php?f=3&t=16700) for test ROMs and example implementation code. 

### 185: 0

Correct bank value unknown. A heuristic that works with all known games that use this mapper is to return $FF on the first two reads from CPU $2007 after a reset. 

### 185: 4

Enable CHR-ROM if bits 0..1 of the latch hold the value 0, otherwise disable CHR-ROM. 

### 185: 5

Enable CHR-ROM if bits 0..1 of the latch hold the value 1, otherwise disable CHR-ROM. 

### 185: 6

Enable CHR-ROM if bits 0..1 of the latch hold the value 2, otherwise disable CHR-ROM. 

### 185: 7

Enable CHR-ROM if bits 0..1 of the latch hold the value 3, otherwise disable CHR-ROM. 

## [206](INES_Mapper_206.xhtml "INES Mapper 206"): Namco 118 PCB variants

### 206: 0

Namcot 118 ASIC with normal PRG banking 

### 206: 1

Namcot **3407** , **3417** and **3451** PCBs with unbanked 32 KiB PRG-ROM. 

  * _Family Jockey_
  * _Metro-Cross_
  * _Sky Kid_
  * _Super Chinese_
  * _Super Sky Kid_
  * _Valkyrieの冒険꞉ 時の鍵伝説_



## [210](INES_Mapper_210.xhtml "INES Mapper 210"): Namco 175 and 340

[Mapper 210](INES_Mapper_210.xhtml "INES Mapper 210") doesn't distinguish between the 175's hardwired mirroring and 340's 1/H/V mirroring. 

Also, previous confusion and compatibility code used by Namco when they were developing games means that many 175- and 340- using games are incorrectly tagged as [mapper 19](INES_Mapper_019.xhtml "INES Mapper 019"). 

### 210: 0

No advisory statement is made (use runtime heuristics suggested at [mapper 210](INES_Mapper_210.xhtml "INES Mapper 210")) 

### 210: 1 N175

Namco 175. Hardwired mirroring, no IRQ. 

  * _Dream Master_
  * _Famista '91_
  * _Family Circuit '91_
  * _Chibi Maruko-chan: Uki Uki Shopping_
  * _Heisei Tensai Bakabon / Genius Bakabon_



### 210: 2 N340

Namco 340. 1/H/V mirroring, no IRQ, no internal or external RAM. 

  * _Splatterhouse_
  * _Wagyan Land 2_
  * _Famista '92_
  * _Top Striker_
  * _Wagyan Land 3_
  * _Famista '93_
  * _Famista '94_



## [215](INES_Mapper_215.xhtml "INES Mapper 215"): Sugar Softec, selectable MMC3 scrambling pattern

Registers at $5000-$5007. 

  * Submapper 0: UNL-8237 (two non-overlapping outer bank bits for PRG/CHR each)
  * Submapper 1: UNL-8237A (three overlapping outer bank bits for PRG/CHR each)



## [232](INES_Mapper_232.xhtml "INES Mapper 232"): Quattro

Similar to #71 above, with a separate register controlling which 64 KiB outer bank of the PRG ROM is used. This is used for the [Quattro multicarts](http://bootgod.dyndns.org:7777/search.php?keywords=quattro&kwtype=game). 

The Aladdin Deck Enhancer version of these multicarts used a different banking scheme. 

### 232: 0

0: Normal 

### 232: 1 Aladdin Deck Enhancer

Aladdin Deck Enhancer variation. Swap the bits of the outer bank number. 

  * _[Quattro Adventure](http://bootgod.dyndns.org:7777/profile.php?id=553) (Aladdin Deck Enhancer)_
  * _[Quattro Sports](http://bootgod.dyndns.org:7777/profile.php?id=550) (Aladdin Deck Enhancer)_
  * _Pegasus 4-in-1_
  * _Super Adventure Quests_ (European release of _Quattro Adventure_)



## [NES 2.0 Mapper 256](NES_2_0_Mapper_256.xhtml "NES 2.0 Mapper 256")

Several plug-and-play consoles use a simple protection scheme intended to prevent interchanging the ROM data between different consoles. The protection scheme works by simply swapping the addresses of the bankswitch registers. Some patterns only change the VTxx-native register addresses, others apply to the MMC3 compatibility registers as well. 

Other plug-and-play consoles, in particular those by Jungletac, scramble CPU opcode bytes (and only those) by swapping bits. These consoles power-on with opcode scrambling enabled, but allow it to be disabled --- for example, to run a pirated licensed NES game without modifying it --- by writing to a register. 

Submapper # | Name | PPU bank affected by ... | CPU bank affected by ... | CPU opcode bytes   
---|---|---|---|---  
$2012 | $2013 | $2014 | $2015 | $2016 | $2017 | $8000.0 | $8000.1 | $8000.2 | $8000.3 | $8000.4 | $8000.5 | $4107 | $4108 | $8000.6 | $8000.7   
0 | Normal | $1000 | $1400 | $1800 | $1C00 | $0000 | $0800 | $0000 | $0800 | $1000 | $1400 | $1800 | $1C00 | $8000 | $A000 | $8000 | $A000 | none   
1 | Waixing VT03 | $1400 | $1000 | $0800 | $0000 | $1C00 | $1800 | $1C00 | $1800 | $1400 | $1000 | $0800 | $0000 | $8000 | $A000 | $8000 | $A000 | none   
2 | Power Joy Supermax | $1000 | $1400 | $1800 | $1C00 | $0000 | $0800 | $0000 | $0800 | $1000 | $1400 | $1800 | $1C00 | $A000 | $8000 | $A000 | $8000 | none   
3 | Zechess/Hummer Team | $0800 | $0000 | $1C00 | $1800 | $1000 | $1400 | $0000 | $0800 | $1000 | $1400 | $1800 | $1C00 | $8000 | $A000 | $8000 | $A000 | none   
4 | Sports Game 69-in-1 | $1800 | $0800 | $1000 | $0000 | $1C00 | $1400 | $0000 | $0800 | $1000 | $1400 | $1800 | $1C00 | $8000 | $A000 | $8000 | $A000 | none   
5 | Waixing VT02 | $1400 | $1000 | $0800 | $0000 | $0000 | $0800 | $0000 | $0800 | $1000 | $1400 | $1800 | $1C00 | $8000 | $A000 | $8000 | $A000 | none   
11 | Vibes | $1000 | $1400 | $1800 | $1C00 | $0000 | $0800 | $0000 | $0800 | $1000 | $1400 | $1800 | $1C00 | $8000 | $A000 | $8000 | $A000  | D7<->D6, D1<->D2 swapped, switched via $411C.6; D5<->D4 swapped, switched via $411C.1   
12 | Cheertone | $1000 | $1400 | $1800 | $1C00 | $0000 | $0800 | $0000 | $0800 | $1000 | $1400 | $1800 | $1C00 | $8000 | $A000 | $8000 | $A000  | D7<->D6, D1<->D2 swapped, switched via $411C.6   
13 | Cube Tech | $1000 | $1400 | $1800 | $1C00 | $0000 | $0800 | $0000 | $0800 | $1000 | $1400 | $1800 | $1C00 | $8000 | $A000 | $8000 | $A000  | D1 and D4 swapped, switched via $4169   
14 | Karaoto | $1000 | $1400 | $1800 | $1C00 | $0000 | $0800 | $0000 | $0800 | $1000 | $1400 | $1800 | $1C00 | $8000 | $A000 | $8000 | $A000  | D6 and D7 swapped, switched via $411C   
15 | Jungletac | $1000 | $1400 | $1800 | $1C00 | $0000 | $0800 | $0000 | $0800 | $1000 | $1400 | $1800 | $1C00 | $8000 | $A000 | $8000 | $A000  | D5 and D6 swapped, switched via $4169   
  
## [268](NES_2_0_Mapper_268.xhtml "NES 2.0 Mapper 268"): Coolboy/Mindkids

  * Submapper 0: Coolboy (outer bank registers at $6000-$6003)
  * Submapper 1: Mindkids (outer bank registers $5000-$5003)



## References

  * [Original submapper list by kevtris](http://blog.kevtris.org/blogfiles/nes/submappers.txt)
  * [Other definitions by kevtris](http://blog.kevtris.org/blogfiles/nes/)



  1. ↑ [NESDev post](http://forums.nesdev.org/viewtopic.php?t=7273) explaining Cybernoid's bugs.
  2. ↑ [Nintendoage thread](http://www.nintendoage.com/forum/messageview.cfm?catid=22&threadid=44613) discussing Donkey Kong Pie Factory.
  3. ↑ [MC-ACC IRQ test results](http://forums.nesdev.org/viewtopic.php?p=128148#p128148)
  4. ↑ <http://forums.nesdev.org/viewtopic.php?t=8274> VRC2 memory bit at $6000-$6FFF
  5. ↑ <http://forums.nesdev.org/viewtopic.php?t=13473> VRC2 mirroring selection tested on hardware


