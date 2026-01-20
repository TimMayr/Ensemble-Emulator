# User:Fiskbit

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AFiskbit) | View [other pages](Special_AllPages.xhtml#User_Fiskbit)

I'll be staging information here before finding a place to move it to on the wiki. 

## Contents

  * 1 CPU / APU
    * 1.1 Behavior
    * 1.2 APU register reads
    * 1.3 Revision differences
      * 1.3.1 Letterless
  * 2 PPU
    * 2.1 Behavior
      * 2.1.1 OAM
      * 2.1.2 Palettes
      * 2.1.3 Fetching
      * 2.1.4 Miscellaneous
    * 2.2 Revision differences
      * 2.2.1 2C02A
      * 2.2.2 2C02B
  * 3 Mappers
    * 3.1 MMC3
    * 3.2 MMC5
    * 3.3 TV-NET MC-1200
      * 3.3.1 Address space
      * 3.3.2 Registers
        * 3.3.2.1 UART register select ($6000)
        * 3.3.2.2 UART data ($6001)
        * 3.3.2.3 VRAM window bank / Game card status ($6002)
        * 3.3.2.4 Game card ROM bank ($6003)
        * 3.3.2.5 Game card RAM bank ($6004)
        * 3.3.2.6 Firmware ROM bank ($6005)
        * 3.3.2.7 VRAM window bank ($6006)
      * 3.3.3 Game card connector
      * 3.3.4 Game card header
  * 4 Consoles
    * 4.1 RF Famicom (HVC-001)
    * 4.2 Famicom (RF, Hong Kong)
    * 4.3 Famicom (AV)
    * 4.4 Twin Famicom AN-500
    * 4.5 Twin Famicom AN-505
    * 4.6 Famicom Titler AN-510
    * 4.7 Sharp C1 TV
    * 4.8 Dataship 1200
    * 4.9 FamicomBox
    * 4.10 NES-001
    * 4.11 NES-001 (NWC)
    * 4.12 NES-101
    * 4.13 M82
  * 5 Research topics



## CPU / APU

### Behavior

  * When the CPU comes out of reset, it is random whether it is aligned on the first or second half of an APU cycle (and thus random whether it begins on a GET or PUT cycle).
  * The state of the CPU's OUT pins is only updated when transitioning from a GET to PUT cycle. Thus, changes to OUT state lasting only 1 cycle (such as when toggled with an INC instruction) may be missed by the joypad device. This can be used to determine CPU/APU alignment, but poses a problem for expansion port devices that use OUT bits to signal when to snoop the data bus.
  * Readable APU registers ($4015 and, if modded, the test registers) drive the internal CPU bus, but not the external CPU bus, so they do not affect external open bus state. Therefore, a dummy read from one of these registers on the cycle before a read from external open bus (such as by using page crossing on an indexed read) will not change the external open bus value.
  * The CPU clock divider is not affected by reset. Therefore, resetting the CPU does not change CPU/PPU sub-cycle alignment.



### APU register reads

The CPU has an internal address bus onto which separate 6502, OAM DMA, and DMC DMA buses are multiplexed. APU registers can only be read when bits [15:5] of the 6502 address bus match the $4000-$401F range. When they match during a read, bits [4:0] of the internal CPU address bus will be checked for a matching register. If those bits are $16 or $17, the relevant joypad is clocked, and if they are $15 or the CPU test pin is asserted, the internal data bus is isolated from the external data bus (preventing the register read from affecting external open bus) and fed from any matching APU register. 

Some unexpected edge cases result from this. Whether APU registers can be read depends on whether the 6502 address bus is in the APU register range, so if it is not, then OAM DMA from the $4000-40FF page will be unable to read them and will not clock joypads. However, if it is in that range when DMA occurs (because the CPU is currently executing from or reading from this range), then any DMA from an address with bits [4:0] matching a readable register or with the test pin asserted will trigger a register read, either overriding the value that would normally be read by the DMA or, in the case of non-test joypad reads, potentially causing a bus conflict. 

This has relevance to real workloads. If a joypad register is read at the same time as a DMC DMA from an address matching an APU register, the DMA will perform a register read, possibly with a bus conflict. Though not yet tested, this should also be able to cause an additional joypad clock, at least for the other joypad (as it is unknown whether additional same-joypad reads are ignored if on consecutive cycles). Thus, an audible glitch may occur if using the repeated-read method of avoiding joypad bit deletion. 

### Revision differences

#### Letterless

  * 4-cycle DMA glitches do not occur.



## PPU

### Behavior

#### OAM

  * Writes to OAM during rendering are ignored.
  * Disabling rendering outside of vblank can cause OAM to be corrupted when rendering next begins. If the disable write occurs approximately in the ranges of dots 0-125 or 254-318, then 2 sprite tiles (1 row) will be corrupted with the value of row 0 when rendering is next enabled. If the disable write occurs in the range 126-253 and rendering is enabled again outside of vblank, then corruption can occur on some CPU/PPU alignments, though this corruption is poorly understood. Dots 319-340 appear safe under all circumstances. This behavior also applies to the rendering-disable caused by resetting the PPU, so the first rendered frame after a PPU reset is likely to have OAM corruption. Note that these values may not be exact to the dot.
  * Signal reflection from a cartridge on the CPU data lines can cause OAM to corrupt. Cartridges with long or thin traces should include 30-100 ohm termination resistors in-series on these lines to suppress signal reflection. This issue has been observed on the FDS, PowerPak, and original Everdrive N8. <https://forums.nesdev.org/viewtopic.php?p=232140#p232140>



#### Palettes

  * On revisions that support reading from palette RAM, doing so via the PPUDATA register at the same time as forced greyscale, as seen in the NTSC border at dot 326, disables forced greyscale for that dot, presumably to prevent the greyscale from clearing the low 4 bits of the value. Greyscale is actually forced everywhere outside of draw range, but can only be seen at the start of the border.
  * Reads from palette RAM do not set bits 7-6, so these will return PPU open bus.
  * The [background palette hack](PPU_palettes.xhtml "PPU palettes") ignores bit 14 of v.
  * When rendering ends at the end of scanline 239, if the value left in v from rendering points into palette RAM, the NTSC border on scanlines 240-241 will automatically display the color pointed to by v (the "background palette hack"). This can be seen in the first corridor of Metal Storm.
  * The auto-increment behavior when writing to $2007 is delayed 1 dot after the write becomes effective. This means writing to palette RAM will show the newly-written color for 1 dot before advancing to the next color.
  * Disabling rendering mid-screen appears to be capable of corrupting palette RAM, but the specifics are not yet entirely known. The corrupted entry appears to depend on the value of v. [https://forums.nesdev.org/viewtopic.php?f=2&t=23209](https://forums.nesdev.org/viewtopic.php?f=2&t=23209)



#### Fetching

  * Writing to v on the first dot of a fetch can result in the low 8 bits being latched from the old v and the high 6 bits coming from the new v, resulting in a fetch from the wrong location. This can happen naturally at dot 258, where the low bits latch on dot 257 before the horizontal component of v is updated and the high bits on 258 after the update. Note that this applies to all fetches, but should only result in mixed addresses on nametable and attribute table fetches.



#### Miscellaneous

  * The PPU clock divider is affected by reset. Therefore, resetting the PPU changes the CPU/PPU sub-cycle alignment.
  * PPU writes have some amount of delay before they take effect, depending on things like where the feature is used in the pixel pipeline and potentially the CPU/PPU cycle alignment. For example, greyscale is applied at the end of the pipeline and is thus approximately instant, while writes to $2006 appear to delay about 3 dots and toggling rendering delays about 4 dots. Note that these amounts are not necessarily exact. Battletoads relies on the rendering toggle delay to skip a vertical increment; without the delay, the extra increment causes a sprite 0 hit to miss, hanging the game.



### Revision differences

#### 2C02A

  * Writes to $2001 disrupt rendering similar to toggling it off and on.
  * PPU bus write timing differs from later revisions in a way that causes graphical corruption on some modern cartridges, such as the Everdrive Pro. (Contemporary cartridges seem unaffected.)
  * On scanlines where sprite tiles are drawn, sprite pixels may be erroneously drawn at X=255.



#### 2C02B

  * On scanlines where sprite tiles are drawn, sprite pixels may be erroneously drawn at X=255.



## Mappers

### MMC3

  * The scanline counter clocks when A12 rises after at least 3 M2 clocks with PPU A12 low.
  * With background using the $1000 table, the scanline counter behavior depends on the aborted fetch performed by the PPU on the idle dot. The skipped dot on the prerender scanline can cause an extra clock because it overrides this dot 0 behavior, creating a large enough window with PPU A12 low for 3 M2 clocks to fit on some CPU/PPU whole-cycle alignments.
  * Writing to $C001, clocking the counter, and writing to $C001 again will cause the counter to be OR'ed with value $80 on the next clock and neither decremented nor reloaded.



### MMC5

  * The in-frame flag isn't set until the same address is fetched 3 times in a row, which occurs during the 3rd fetch for a scanline. This causes the first 3 fetches when using extended attributes to read incorrect data.
  * Because disabling rendering causes the in-frame flag to clear, updating palettes mid-frame will also trigger incorrect fetches when using extended attributes.



### TV-NET MC-1200

#### Address space
    
    
    $5000-57FF: VRAM window bank
    $6000-6FFF: TV-NET registers
    $7000-7FFF: Game card RAM bank
    $8000-BFFF: First 4 game card or firmware ROM banks
    $C000-DFFF: Game card or firmware ROM banks
    $E000-EFFF: Firmware ROM bank
    $F000-FFFF: Last firmware bank
    

$8000-DFFF map firmware banks if a game card is not present. 

#### Registers

8 registers, mirrored across $6000-6FFF. These are work-in-progress. 

##### UART register select ($6000)

Write: 

  * Register $00 seems to be incoming (serial?) data bytes when read, outgoing data bytes when written.
  * Register $02 seems like a 1-hot encoded status when read, $04 means something to do with data transmission, $02 some other special condition.
  * Register $05 seems to have something to do with the incoming and outgoing data validity? 
    * Bits 1, 2, and 3 are to do with incoming data.
    * Bits 5 and 6 are to do with outgoing data.



Read: 

  * Possibly required after selecting a register but before reading from that register? Could be purely timing effects, or could be doing something else. Not required for writing to modem registers.



##### UART data ($6001)

Likely read/write data for register selected by $6000. 

##### VRAM window bank / Game card status ($6002)

Write: 2 KB VRAM window bank. Unknown if bit 0 ignored.  
Read: $00 if a game card is present and non-$00 otherwise. 

##### Game card ROM bank ($6003)
    
    
    7  bit  0
    ---- ----
    xxDD DDDx
      || |||
      ++-+++-- Game card ROM bank at $C000-DFFF.
    

Unknown if bit 0 ignored. 

##### Game card RAM bank ($6004)
    
    
    7  bit  0
    ---- ----
    xxxx xDDD
          |||
          +++- Game card RAM bank at $7000-7FFF.
    

##### Firmware ROM bank ($6005)
    
    
    7  bit  0
    ---- ----
    xxxx xDDD
          |||
          +++- Firmware ROM bank at $E000-EFFF.
    

##### VRAM window bank ($6006)

Appears to control what is mapped into the $5000-57FF window. Observed writes are 4 before writing to what appears to be 1 KB of special graphics memory (MMC5-style ExRAM?) at $5000 and 3 after, and 5 to map in CHR-RAM, which is 32 KB and can be banked using $6002 in 1 KB amounts (unknown if bit 0 ignored). 

#### Game card connector
    
    
    Card |  | TV-NET 
    -----+--+--------
      1  |--| GND
      2  |  | n/c
      3  |--| +5Vcc
      4  |<-| /CS
      5  |<-| RAM/ROM
      6  |<-| R/W
      7  |<-| ROM /OE
      8  |<>| CPU D0
      9  |<>| CPU D1
     10  |<>| CPU D2
     11  |<>| CPU D3
     12  |<>| CPU D4
     13  |<>| CPU D5
     14  |<>| CPU D6
     15  |<>| CPU D7
     16  |<-| CPU A0
     17  |<-| CPU A1
     18  |<-| CPU A2
     19  |<-| CPU A3
     20  |<-| CPU A4
     21  |<-| CPU A5
     22  |<-| CPU A6
     23  |<-| CPU A7
     24  |<-| CPU A8
     25  |<-| CPU A9
     26  |<-| CPU A10
     27  |<-| CPU A11
     28  |<-| A12
     29  |<-| A13
     30  |<-| A14
     31  |<-| A15
     32  |<-| A16
     33  |<-| A17
     34  |--| n/c in JRA-PAT, +5Vcc in MC-1200 host
     35  |<-| /RAMSEL
     36  |--| +5Vcc
     37  |->| Card /present
     38  |--| GND
    

RAM/ROM selects whether /CS is sent to RAM or ROM. If it is sent to ROM, /RAMSEL is send to RAM.  
Card /present affects /CE for the TV-NET firmware ROM and tells the mapper it's present via U6.98. 

#### Game card header

Game cards are expected to have a pointer at $8000-8001 to a validation signature that must match a string at $F223 in the firmware, null-terminated on the firmware side. The game card start vector is located at $8004-8005. 

## Consoles

Information about different console models, revisions, and behaviors. Still a long way to go here. Ideally, complex consoles like the Famicom Titler and FamicomBox would be broken out into their own pages. 

### RF Famicom (HVC-001)

Joypads clock once per read cycle instead of once per consecutive set of read cycles. 

CPU revisions: 

  * RP2A03: letterless, E, G



PPU revisions: 

  * RP2C02: letterless, A, B, C, D, D-0, E-0, G-0
  * RC2C02: C



Motherboard revisions: 

  * HVC-CPU-01
  * HVC-CPU-02 (CPU: letterless, PPU: A)
  * HVC-CPU-03
  * HVC-CPU-04
  * HVC-CPU-05
  * HVC-CPU-06
  * HVC-CPU-07
  * HVC-CPU-08
  * HVC-CPU-GPM-01
  * HVC-CPU-GPM-02



Letterless CPUs appear to be used on revisions 01-06, E starting with 06, and G on 07 and later. Revision A PPUs were used on revisions 01-05, with B and D both appearing on 05-06. 

### Famicom (RF, Hong Kong)

A PAL Famicom using NTSC chips, with a 50 Hz / 60 Hz switch. The master clock is 21.3125MHz. When in 50 Hz ("Slow") mode, a Nintendo N NPC26 chip slows the PPU by stopping its clock sometime between the end of rendering and the start of vblank, inserting 50 scanlines. If the CPU accesses the PPU during this time, it is allowed to run again briefly to handle the access, shortening the length of the frame. 

CPU revisions: 

  * RP2A03: G



PPU revisions: 

  * RP2C02: G-0



Motherboard revisions: 

  * HVC-CPU-NPC-18-01
  * HVC-CPU-NPC-26-01



### Famicom (AV)

CPU revisions: 

  * RP2A03: G, H



PPU revisions: 

  * RP2C02: G-0, H-0



Motherboard revisions: 

  * HVCN-CPU-01
  * HVCN-CPU-02



### Twin Famicom AN-500

Combination Famicom/FDS console. 

CPU revisions: 

  * RP2A03: E



PPU revisions: 

  * RP2C02: E-0



### Twin Famicom AN-505

Combination Famicom/FDS console with turbo controllers. 

### Famicom Titler AN-510

An RGB console with a built-in video titler. Features various buttons and a resistive touch panel. 

CPU Revisions: 

  * RP2A03: G



PPU Revisions: 

  * RC2C05: -99



Motherboard revisions: 

  * K6285DE
  * K6285DE △2



Note that the revisions are indicated within a numbered triangle next to the PCB revision. 

### Sharp C1 TV

### Dataship 1200

A combination Famicom and Famicom Network System, but without a Famicom cartridge slot or standard controllers. Features a 14-pin Centronics port for interfacing with a printer, not present on a standard Famicom Network System modem. Compatible printers include the Chinon IJK-112 with a IF-1122 cable and the NEC PC-PR102TL3 with a PC-8894 cable. Uses a [Ricoh RF5GH05](User_Ben_Boldt.xhtml "User:Ben Boldt") to handle the Centronics port as well as generate /ROMSEL, CPU RAM /CE, and PPU /CE. 

CPU revisions: 

  * RP2A03: G



PPU revisions: 

  * RP2C02: G-0



Motherboard revisions: 

  * K4013800B
  * K4013800C



### FamicomBox

Also released in Sharp FamicomStation branding. Features 16 cartridge slots, arranged into 3 groups of 5 and one additional primary slot, which can be selected between at runtime. Includes 8 KB of RAM at each of $0000-1FFF and $6000-7FFF and registers at $5000-5007 (mirrored through $5FFF). Offers various kinds of exceptions that can reset the console and return to the primary slot. Uses a 3198A CIC and will skip past slots that don't have one. Uses a 3199A coin timer, which can mix a beep into the audio out and trigger a screen-dimming effect of approximately 40% implemented in the RF modulator. 

CPU revisions: 

  * RP2A03: E



PPU revisions: 

  * RP2C02: E-0



Motherboard revisions: 

  * SSS-CPU-03
  * SSS-CPU-04



### NES-001

CPU revisions: 

  * RP2A03: E, G
  * RP2A07: letterless, A



PPU revisions: 

  * RP2C02: E-0, G-0
  * RP2A07: -0, A, A-0



Motherboard revisions: 

  * NES-CPU (01)
  * NES-CPU-02
  * NES-CPU-03
  * NES-CPU-04
  * NES-CPU-05
  * NES-CPU-06
  * NES-CPU-07
  * NES-CPU-08
  * NES-CPU-09
  * NES-CPU-10
  * NES-CPU-11
  * K6529WE (Sharp NES TV)



Revision E chipsets appear to be featured on revisions 01-04, while G chipsets appear on 05-11 and the Sharp NES TV. 

### NES-001 (NWC)

The Nintendo World Championships used a modified NES-001, adding a Champ1 rev.A board with a 4-pin RJ port. This board features two optoisolators and is wired internally to +RESET and Joypad 2 D3. Looking into the port, the pins are: 
    
    
    +--+
    |  +-+
    |    +--+
    |      1| -- GND for optoisolators
    |      2| -- GND for optoisolators
    |      3| <- +RESET
    |      4| <- Joypad 2 /D3
    |    +--+
    |  +-+
    +--+
    

### NES-101

CPU revisions: 

  * RP2A03: G, H?



PPU revisions: 

  * RP2C02: G-0, H-0?



Motherboard revisions: 

  * NESN-CPU-01
  * NESN-CPU-JIO-01
  * NESN-CPU-JIO-02
  * NESN-CPU-AV-01



### M82

Uses a custom ASIC, the 274 Nintendo RCA Z IRSI. 

CPU revisions: 

  * RP2A03: G
  * RP2A07: letterless



PPU revisions: 

  * RP2C02: G-0
  * RP2C07: Unknown



Motherboard revisions: 

  * IRS-45-003 Rev.F



## Research topics

  * Disabling rendering mid-screen is capable of corrupting palette RAM, depending in some way on the value of v. This appears to only occur on some CPU/PPU alignments. All NTSC PPU revisions are believed to be affected, and PAL and RGB PPUs have not yet been tested. [Reference: Problem with palette discoloration when PPU is turned off during rendering.](https://forums.nesdev.org/viewtopic.php?f=2&t=23209)
  * NTSC and RGB PPUs prior to revision C have a bug where scanlines with sprites on them may also have sprite pixels mistakenly drawn at X=255. These might be tied to specific OAM rows.
  * DMC DMA timing on PAL may not be properly understood. Timing in the Years Behind demo differs between emulator and console.
  * Disabling rendering mid-screen can cause OAM corruption when rendering is reenabled, regardless of whether OAM DMA was performed in between and even across a PPU reset. The corruption behavior depends on where in the scanline rendering turns off. These regions are approximately dots 1-128, 129-256, and 257-320. The first and last regions appear to cause a row to be replaced with the data from row 0, while the middle region's corruption is not understood and appears to only occur if rendering is reenabled at certain positions within a dot (that is, only on certain CPU/PPU alignments and not if rendering enables at the beginning of a dot, such as at the start of the frame). PPUs prior to revision E seem to only show glitches for the 129-256 region. [Reference: Isolated Warrior unemulated graphical glitche](https://forums.nesdev.org/viewtopic.php?p=248607#p248607)
  * Changing CHR banks on MMC3 appears to be able to cause gliltchy slivers, but what data is being displayed is not known. [Reference: Isolated Warrior unemulated graphical glitches](https://forums.nesdev.org/viewtopic.php?p=248525#p248525)
  * The 'signal reflection' issue where a cartridge's effects on the CPU data lines can cause PPU OAM corruption is not fully understood and may be related in part to flash hardware in modern cartridges compared to contemporary ROMs. Resistors on all 8 CPU data lines near the cartridge do appear to fix this problem, but solutions that don't require this extra hardware would be preferred and so further research is warranted.


