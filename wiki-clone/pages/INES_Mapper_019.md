# INES Mapper 019

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_019) | View [other pages](Special_AllPages.xhtml#INES_Mapper_019)

**Namco 129/163**

**Company** | Namco   
---|---  
**Games** | [20 in NesCartDB](https://nescartdb.com/search/advanced?ines=19)  
**Complexity** | ASIC   
**Boards** | 11xF0, 111F0S   
**Pinout** | [Namcot 163 family pinout](Namcot_163_family_pinout.xhtml "Namcot 163 family pinout")  
**PRG ROM capacity** | 512K   
**PRG ROM window** | 8Kx3 + 8K fixed   
**PRG RAM capacity** | 8K   
**PRG RAM window** | 8K   
**CHR capacity** | 256K   
**CHR window** | 1Kx8 (PT) + 1Kx4 (NT)   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | arbitrary, up to 226 source nametables   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | No   
**IRQ** | CPU cycle counter   
**Audio** | [Yes](Namco_163_audio.xhtml "Namco 163 audio")  
**iNES[mappers](Mapper.xhtml "Mapper")** | 019  
  
**iNES Mapper 019** is used to denote Famicom boards bearing the **Namco 129** and **Namco 163** ASICs. These chips contain 128 bytes of internal RAM that can be used either for [expansion audio](Namco_163_audio.xhtml "Namco 163 audio") or, together with a battery, for 128 bytes of save RAM. Games may also come with the regular 8 KiB of additional battery-backed WRAM mapped into CPU address space at $6000-$7FFF. 

In [NES 2.0](NES_2_0.xhtml "NES 2.0"), the 128 bytes of internal chip RAM are not included in the PRG-RAM and PRG-NVRAM fields. NES 2.0 Mapper 19 ROM images therefore either have: 

  * no battery and no WRAM: battery bit clear, PRG-RAM/PRG-NVRAM both set to zero.
  * a battery but no WRAM: battery bit set, PRG-RAM/PRG-NVRAM both set to zero. The game writes its save data into the 128 byte internal RAM.
  * a battery and WRAM: battery bit set, PRG-RAM set to zero and PRG-NVRAM set to 8192.



The NES 2.0 submapper is used to specify the mixing resistor that determines the relative volume of the expansion audio channels against the [APU](APU.xhtml "APU")'s audio channels: 

[INES Mapper 019 submapper table](INES_Mapper_019_Submapper_table.xhtml "INES Mapper 019/Submapper table")  
---  
Submapper # | Meaning | Note   
0 | Default | Expansion sound volume unspecified   
_1_ | _Deprecated_ | Internal 128b RAM is battery backed, no external PRG-RAM is present. No expansion sound. (Equivalent to submapper 2 with 0 in PRG-NVRAM field.)   
2 | No expansion sound |   
3 | N163 expansion sound: **11.0-13.0 dB** louder than NES APU |   
4 | N163 expansion sound: **16.0-17.0 dB** louder than NES APU |   
5 | N163 expansion sound: **18.0-19.5 dB** louder than NES APU |   
  
  


## Contents

  * 1 Game list
  * 2 Banks
  * 3 Registers
    * 3.1 Chip RAM Data Port ($4800-$4FFF) r/w
    * 3.2 IRQ Counter (low) ($5000-$57FF) r/w
    * 3.3 IRQ Counter (high) / IRQ Enable ($5800-$5FFF) r/w
    * 3.4 CHR and NT Select ($8000-$DFFF) w
    * 3.5 PRG Select 1 ($E000-$E7FF) w
    * 3.6 PRG Select 2 / CHR-RAM Enable ($E800-$EFFF) w
    * 3.7 PRG Select 3 ($F000-$F7FF) w
    * 3.8 Write Protect for External RAM AND Chip RAM Address Port ($F800-$FFFF) w
  * 4 IRQ Operation
  * 5 Notes
  * 6 CHR Memory Configurations
  * 7 See also
  * 8 References



## Game list

Name | Chip | Battery | WRAM | Expansion Audio | NES 2.0 Submapper   
---|---|---|---|---|---  
_Battle Fleet_ | 163 | Yes | No | No | 2   
_Dragon Ninja_ | 163 | No | No | No | 2   
_Digital Devil Story: Megami Tensei II_ | 163 | Yes | Yes | Yes | 3   
_Dokuganryuu Masamune_ | 163 | Yes | No | No | 2   
_Erika to Satoru no Yume Bouken_ | 163 | No | No | Yes | 5   
_Famista '90_ | 163 | Yes | No | No | 2   
_Final Lap_ | 163 | No | No | Yes | 3   
_Hydlide 3: Yami kara no Houmonsha_ | 163 | Yes | No | No | 2   
_Juvei Quest_ | 163 | Yes | Yes | No | 2   
_Kaijuu Monogatari_ | 163 | Yes | No | No | 2   
_King of Kings_ | 163 | Yes | Yes | Yes | 5   
_Mappy Kids_ | 163 | No | No | Yes | 5   
_Mindseeker_ | 163 | Yes | No | No | 2   
_Namco Classic_ | 163 | No | No | No | 2   
_Namco Classic II_ | 163 | No | No | Yes | 3   
_Rolling Thunder_ | 163 | No | No | Yes | 4   
_Sangokushi: Chuugen no Hasha_ | 163 | Yes | Yes | Yes | 5   
_Sangokushi II: Haou no Tairiku_ | 163 | Yes | Yes | Yes | 3   
_Star Wars_ | 129 | No | No | No | 2   
_Youkai Douchuuki_ | 163 | No | No | Yes | 5   
  
## Banks

  * CPU $6000-$7FFF: 8 KB PRG RAM bank, if WRAM is present
  * CPU $8000-$9FFF: 8 KB switchable PRG ROM bank
  * CPU $A000-$BFFF: 8 KB switchable PRG ROM bank
  * CPU $C000-$DFFF: 8 KB switchable PRG ROM bank
  * CPU $E000-$FFFF: 8 KB PRG ROM bank, fixed to the last bank
  * PPU $0000-$03FF: 1 KB switchable CHR bank
  * PPU $0400-$07FF: 1 KB switchable CHR bank
  * PPU $0800-$0BFF: 1 KB switchable CHR bank
  * PPU $0C00-$0FFF: 1 KB switchable CHR bank
  * PPU $1000-$13FF: 1 KB switchable CHR bank
  * PPU $1400-$17FF: 1 KB switchable CHR bank
  * PPU $1800-$1BFF: 1 KB switchable CHR bank
  * PPU $1C00-$1FFF: 1 KB switchable CHR bank
  * PPU $2000-$23FF: 1 KB switchable CHR bank
  * PPU $2400-$27FF: 1 KB switchable CHR bank
  * PPU $2800-$2BFF: 1 KB switchable CHR bank
  * PPU $2C00-$2FFF: 1 KB switchable CHR bank



These ASICs have the unusual ability to select the internal 2 KB nametable RAM as a CHR bank page, allowing it to be used as CHR RAM in combination with the existing CHR ROM. 

## Registers

The 163 has 19 registers within $4800-$5FFF and $8000-$FFFF. Each register occupies a range of $800 bytes, so $4800-$4FFF all refers to one register, $5000-$57FF all refers to another register, and so on. 

### Chip RAM Data Port ($4800-$4FFF) r/w

See [Namco 163 audio](Namco_163_audio.xhtml "Namco 163 audio"). 

### IRQ Counter (low) ($5000-$57FF) r/w
    
    
    7  bit  0
    ---- ----
    IIII IIII
    |||| ||||
    ++++-++++- Low 8 bits of IRQ counter
    

### IRQ Counter (high) / IRQ Enable ($5800-$5FFF) r/w
    
    
    7  bit  0
    ---- ----
    EIII IIII
    |||| ||||
    |+++-++++- High 7 bits of IRQ counter
    +--------- IRQ Enable: (0: disabled; 1: enabled)
    

### CHR and NT Select ($8000-$DFFF) w

Value CPU writes | Behavior   
---|---  
$00-$DF | Always selects 1KB page of CHR-ROM   
$E0-$FF | If enabled by bit in $E800, use the NES's internal nametables (even values for A, odd values for B)   
Write to CPU address | 1KB CHR bank affected | Values ≥ $E0 denote NES NTRAM if   
---|---|---  
$8000-$87FF | $0000-$03FF | $E800.6 = 0   
$8800-$8FFF | $0400-$07FF | $E800.6 = 0   
$9000-$97FF | $0800-$0BFF | $E800.6 = 0   
$9800-$9FFF | $0C00-$0FFF | $E800.6 = 0   
$A000-$A7FF | $1000-$13FF | $E800.7 = 0   
$A800-$AFFF | $1400-$17FF | $E800.7 = 0   
$B000-$B7FF | $1800-$1BFF | $E800.7 = 0   
$B800-$BFFF | $1C00-$1FFF | $E800.7 = 0   
$C000-$C7FF | $2000-$23FF | always   
$C800-$CFFF | $2400-$27FF | always   
$D000-$D7FF | $2800-$2BFF | always   
$D800-$DFFF | $2C00-$2FFF | always   
  
It is believed, but untested, that a game could add a normal SRAM and use it in lieu of the nametable RAM; if so, a game would be able to get 4-screen mirroring and many more pages of CHR-RAM. 

### PRG Select 1 ($E000-$E7FF) w
    
    
    7  bit  0
    ---- ----
    AMPP PPPP
    |||| ||||
    ||++-++++- Select 8KB page of PRG-ROM at $8000
    |+-------- Disable sound if set
    +--------- Pin 22 (open collector) reflects the inverse of this value, unchanged by the address bus inputs.
    

If not battery backed, the internal sound RAM powers on in a random state, and similarly the N163 may power on with sound enabled. A short burst of sound due to these random settings may be heard at first power-on, before the game software manages to disable it. 

### PRG Select 2 / CHR-RAM Enable ($E800-$EFFF) w
    
    
    7  bit  0
    ---- ----
    HLPP PPPP
    |||| ||||
    ||++-++++- Select 8KB page of PRG-ROM at $A000
    |+-------- Disable CHR-RAM at $0000-$0FFF
    |            0: Pages $E0-$FF use NT RAM as CHR-RAM
    |            1: Pages $E0-$FF are the last $20 banks of CHR-ROM
    +--------- Disable CHR-RAM at $1000-$1FFF
                 0: Pages $E0-$FF use NT RAM as CHR-RAM
                 1: Pages $E0-$FF are the last $20 banks of CHR-ROM
    

### PRG Select 3 ($F000-$F7FF) w
    
    
    7  bit  0
    ---- ----
    CDPP PPPP
    |||| ||||
    ||++-++++- Select 8KB page of PRG-ROM at $C000
    ++-------- PPU A12, A13 and these bits control pin 44
    

Pin 44 is: 

  * if PPU A13 is high, then D
  * if PPU A13 is low, then C bitwise-or PPU A12



Additionally, choosing bank $3F here replaces the CHR bank output with debugging information for the internal audio state 

### Write Protect for External RAM AND Chip RAM Address Port ($F800-$FFFF) w
    
    
    7  bit  0
    ---- ----
    KKKK DCBA
    |||| ||||
    |||| |||+- 1: Write-protect 2kB window of external RAM from $6000-$67FF (0: write enable)
    |||| ||+-- 1: Write-protect 2kB window of external RAM from $6800-$6FFF (0: write enable)
    |||| |+--- 1: Write-protect 2kB window of external RAM from $7000-$77FF (0: write enable)
    |||| +---- 1: Write-protect 2kB window of external RAM from $7800-$7FFF (0: write enable)
    ++++------ Additionally the upper nybble must be equal to b0100 to enable writes
    

Any value outside of the range $40-$4E will cause all PRG RAM to be read-only. 

Also see [Namco 163 audio](Namco_163_audio.xhtml "Namco 163 audio"). 

## IRQ Operation

The IRQ is a 15-bit CPU cycle up-counter. $5000 and $5800 provide _direct_ access to the counter itself (i.e., this isn't a reload value). Games can read and write to these registers in realtime. 

The IRQ counter increments on each CPU cycle. Upon reaching $7FFF, an IRQ is fired, and it stops counting. Writing to $5000 or $5800 will acknowledge any pending IRQs. 

## Notes

  * Many older programs incorrectly identify this mapper by the name Namco 106. Some sources use the name Namcot instead of Namco, as some games and PCBs use this variation on the company name.
  * The N163 supports 8k of PRG-RAM but also has 128 bytes of internal memory. If there is a battery, then both will be battery backed. The internal memory is battery backed for several games that have no additional PRG-RAM.
  * If it is not battery backed, the internal RAM state will be undefined at power-on[1].
  * The only known difference between the Namco 129 and 163 is the 129's subtly different design of expansion audio. The only known game (Star Wars) using the Namco 129 can also be found in later runs with a Namco 163 ASIC.
  * An IC labelled "160" has been found with a copy of _Dokuganryuu Masamune_ : [[1]](https://forums.nesdev.org/viewtopic.php?p=248291#p248291)
  * Many [INES Mapper 210](INES_Mapper_210.xhtml "INES Mapper 210") games are incorrectly set to Mapper 019.
  * _Dokuganryuu Masamune_ is often thought as having WRAM. Tests have indicated however that the fourth glob is not SRAM but a [protection circuit for the chip-internal RAM](http://forums.nesdev.org/viewtopic.php?t=7727#p111097).



## CHR Memory Configurations

All commercial-era titles only come with CHR-ROM. There is interest in the homebrew commmunity however in creating cartridges with both CHR-ROM and CHR-RAM. The following configurations are plausible and should be emulated as follows: 

  * Only CHR-ROM: CHR banks 00-DF (or all if E800.6/7=1) go to CHR-ROM, and E0-FF go to CIRAM (normal commercial-era configuration).
  * Only CHR-RAM: CHR banks 00-DF (or all if E800.6/7=1) go to CHR-RAM, and E0-FF go to CIRAM.
  * CHR-ROM of any size, plus up to 32 KiB of CHR-RAM: CHR banks 00-DF (or all if E800.6/7=1) go to CHR-ROM, and E0-FF go to CHR-RAM.
  * CHR-ROM plus more than 32 KiB of CHR-RAM are invalid.



There is no trivial means, short of manually decoding the CHR output address lines, to distinguish three types of CHR memory, CHR-ROM+CHR-RAM+CIRAM. 

## See also

  * [Namcot 106](http://nesdev.org/namco106.txt) by goroh, fix by ZW4 and nori, english translation by nori.
  * [Comprehensive NES Mapper Document](http://nesdev.org/mappers.zip) by \Firebug\, information about mapper's initial state is inaccurate.



All of the below are in Japanese: 

  * [Namco mapper](http://nesdev.org/namco.txt) by goroh
  * [Naruko's post to the forum](http://forums.nesdev.org/viewtopic.php?p=77795#77795)
  * [Wiki on same](http://unagi.sourceforge.jp/cgi-bin/hiki/hiki.cgi?163_namco)
  * [Naruko's blog's post on the write protect register at $F800](http://d.hatena.ne.jp/na6ko/20110430/1304099059)



## References

  1. ↑ [N163 Sound RAM Initialisation Behaviour](https://forums.nesdev.org/viewtopic.php?p=285135)



Categories: [ASIC mappers](Category_ASIC_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [Mappers using $4020-$5FFF](Category_Mappers_using__4020__5FFF.xhtml), [Mappers with CHR ROM and CHR RAM](Category_Mappers_with_CHR_ROM_and_CHR_RAM.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml), [Mappers with ROM nametables](Category_Mappers_with_ROM_nametables.xhtml), [Mappers with single-screen mirroring](Category_Mappers_with_single_screen_mirroring.xhtml)
