# NES 2.0 submappers/Proposals

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_submappers/Proposals) | View [other pages](Special_AllPages.xhtml#NES_2_0_submappers_Proposals)

This page collects proposals for [NES 2.0 submappers](NES_2_0_submappers.xhtml "NES 2.0 submappers") that are not yet ready for implementation. 

  1. Explain what game or ROM is incompatible with existing submappers.
  2. Explain how the proposed submapper should be implemented.
  3. Allow one or more other members of the community to independently verify that both 1 and 2 are correct. (We'll perform peer review, commentary, and possible revision/iteration here.)
  4. Allocate and document the new submapper, listing the relevant game/ROM.



When allocating new submappers, please consult kevtris' original proposal before choosing a number. If it is something he already assigned that we have just not adopted yet, use his existing assignment: [submappers.txt](http://blog.kevtris.org/blogfiles/nes/submappers.txt)

If there is no existing game or ROM that requires a submapper, it should not yet be allocated. There is no end to possible variations of hardware, and there is no need to speculate on the future. If you want to work on a project that will require a new mapper, engage the community and/or seek help from others. Do not pre-emptively add a new mapper to the spec until there is something we can run with it. The spec will still be here when you're finished your project. 

## Contents

  * 1 005: MMC5
    * 1.1 005: 0
    * 1.2 005: 1
    * 1.3 Bitfield Wishlist
  * 2 070: Bandai UNROM/GNROM hybrid
  * 3 083: Cony
    * 3.1 083: 0
    * 3.2 083: 1
    * 3.3 083: 2
  * 4 086: Jaleco JF-13
    * 4.1 086: 0
    * 4.2 086: 1
  * 5 References



## 005: [MMC5](MMC5.xhtml "MMC5")

Status: MMC5A is unimplemented in emulators, and all existing games are also valid as MMC5. Seems to be de-facto low priority. 

### 005: 0

MMC5 

### 005: 1

MMC5A 

MMC5A is a known variant of the chip with some added features. However, all existing games with MMC5A also appear to have existed with MMC5. Nothing deliberately uses these features, but behaviour of a bug could differ by the selected chip. 

### Bitfield Wishlist

A previous proposal included this wishlist to use it as a bitfield for other mapper configurations that were never used in games. With the discovery of MMC5A variations, there can no longer enough bits to accommodate all of these. 

Vertical split mode:   
0: SL (all known hardware)   
1: CL   


If only one kind (battery or non-battery) of PRG-RAM present:   
0: PRG-RAM is contiguous (EKROM, EWROM)   
2: PRG-RAM is not contiguous; is split in half across two chips   


If both kinds of PRG-RAM present:   
0: Chip 0 is battery-backed (ETROM (note: verify this))   
4: Chip 1 is battery-backed   


Pulse waves volume:   
0: R1 is 6.8kΩ (as in all games that use expansion audio)   
8: R1 is 15kΩ (the nominal value of this resistor)   


## 070: Bandai UNROM/GNROM hybrid

Status: Problem outline, needs followup? 

There is [a report](https://forums.nesdev.org/viewtopic.php?t=16916) of a pirate copy of a game that seems to want mapper 70 without bus conflicts, even though Bandai's original hardware should have them. 

Tentatively, we could use the same submappers as those standardized for mappers 2, 3, & 7\. 

## [083](INES_Mapper_083.xhtml "INES Mapper 083"): Cony

Status: Needs documentation of affected games and implementation testing. 

"There's actually three different versions of the hardware, all assigned to the same mapper. Unfortunately, we have no idea which is which."

Kevtris's assignments: 

### 083: 0

"Bog-standard Cony mapper. 1K CHR ROM banks, no WRAM."

### 083: 1

"Same, but with 2K CHR ROM banks instead."

### 083: 2

"This is the standard Cony mapper with the following changes: 

  1. 1K CHR ROM banks (like 83.0)


  1. a 4 bit 256K CHR/PRG bank select register: 
     * B000h: bits 6 and 7 select the 256K superbank


  1. 1 byte of RAM at 5103h (stores the last game played) Game will not start without this RAM byte.


  1. WRAM at 6000-7FFFh. WRAM is banked with the PRG/CHR superbank. This gives a total of 32K. It is battery backed.



## [086](INES_Mapper_086.xhtml "INES Mapper 086"): Jaleco JF-13

Status: Needs documentation of behaviour differences and chip emulation, a way to dump sample data for emulatable ROMs, and relevant games. 

There is a bootleg variant that uses a UM5100 (DPCM) instead of µPD7756C (ADPCM). 

### 086: 0

Uses µPD7756C (Standard). 

### 086: 1

Uses UM5100 (Bootleg). 

## References

  * [Atari Age forum post](http://atariage.com/forums/topic/242970-fpga-based-videogame-system/?p=3687219) \- Kevtris' Analogue NT Mini firmware notes including a slightly updated submapper list.


