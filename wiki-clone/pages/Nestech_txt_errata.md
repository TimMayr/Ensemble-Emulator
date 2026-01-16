# Nestech.txt errata

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Nestech.txt_errata) | View [other pages](Special_AllPages.xhtml#Nestech_txt_errata)

    _This document is of historical interest. For the latest community version, see[Nestech.txt](Nestech_txt.xhtml "Nestech.txt")._

**Nintendo Entertainment System Documentation** , sometimes referred to as **nestech.txt** after its filename, is a document written in October 1999 by [koitsu/Jeremy Chadwick](User_Koitsu.xhtml "User:Koitsu") that synthesized the public knowledge about the NES hardware at the time it was written. Much of it is still believed accurate; some isn't. This document explains how the public's understanding of NES hardware changed between the publication of version 2.00 of nestech.txt and the wiki.nesdev.org era. It was used to produce an updated revision of [nestech.txt](Nestech_txt.xhtml "Nestech.txt") that incorporates new discoveries. 

## Contents

  * 1 Introduction
  * 2 Acronymns
  * 3 CPU
  * 4 PPU
  * 5 pAPU
  * 6 Joypads, paddles, expansion ports
  * 7 Memory Mapping Hardware
  * 8 Registers
  * 9 File Formats
  * 10 Programming the NES
  * 11 Emulation
  * 12 Reference Material



## Introduction

A. Disclaimer
    Implies that Nintendo was still offering NES development tools in the Nintendo 64 era.
E. "Thank You"s
    People listed here who are active in the NES scene as of 2015 include at least Kevtris, Loopy and Memblers.

## Acronymns

[sic] 

pAPU
    "There is no physical IC for audio processing nor generation" may mislead. True, the APU is not a discrete component, but it's certainly physical, taking up roughly half the CPU die.
SPR-RAM
    "OAM" (object attribute memory) is more common nowadays.
SRAM
    Used to mean "save RAM", but means "static RAM" on NESdev Wiki. Going forward, the term "battery RAM" should be unambiguous.
Dandy
    Misspelling of Dendy, a PAL famiclone distributed by Steepler that uses a [long post-render period](Cycle_reference_chart.xhtml#Clock_rates "Cycle reference chart") to improve Famicom game compatibility.

## CPU

A. General Information
    The PAL CPU clock frequency is that of the Dendy, which uses a /15 divider. The PAL NES CPU is 6.25% slower because of its /16 divider.

## PPU

B. Memory Map
    The size of Name Tables in the RAM Memory Map is given as $800, which is enough for two. Some carts have one, three, or four; see [Mirroring](Mirroring.xhtml "Mirroring"). The following Programmer Memory Map is correct.
F. Palettes
    The "lookup table" interpretation is valid for RGB PPUs (2C03-2C05), but the underlying relationship with composite and YUV used by the 2C02 and 2C07 wasn't confirmed until later.
G. Name Table Mirroring
    The explanation of horizontal mirroring doesn't distinguish addresses in nametable memory ($000-$7FF) from where they are mapped in ($2000-$23FF and $2800-$2BFF). It could be misinterpreted as implying $2400 as the second nametable, which is correct for vertical arrangement on the Super NES and Game Boy Advance but not on the NES.
H. Palette Mirroring
    $3F00 defines background color only when rendering is on or the current VRAM address is outside $3F00-$3FFF. If rendering is off, and the VRAM address points into the palette, the color at the current VRAM address is used as the background color. Ignoring this can cause rainbow glitches during updates.
I. Background Scrolling
    The example uses "A" and "B" in a screen-aligned, and the implicit "arrangement" naming convention isn't clearly distinguished from the "mirroring" convention used later on. The position of nametables #2 and #3 ($2800 and $2C00) above nametables #0 and #1 ($2000 and $2400) is equally valid but not common outside this document. The reason for "negative" values (later) isn't explained.
J. Screen and Sprite Layering
    The diagram implies the naive understanding of [PPU sprite priority](PPU_sprite_priority.xhtml "PPU sprite priority"), not the more nuanced understanding needed to get occlusion working in _Super Mario Bros. 3_ and _RHDE_.
K. Sprites and SPR-RAM
    The fact that OAM "will gradually degrade" was discovered early on, but we didn't know how rapid this was. It was discovered to be so fast that the PAL NES PPU enters a refresh pattern after the first 20 lines of vblank just to keep it from decaying.
L. Sprite #0 Hit Flag
    The failure to detect at X=255 isn't mentioned.
M. Horizontal and Vertical Blanking
    The diagram implies that the NTSC PPU generates a 263-line signal with 3 lines of post-render, 20 lines of vblank, and zero pre-render. (The correct values are 1, 20, and 1.) Nor is the 341-dot length of most scanlines mentioned.
N. $2005/2006 Magic
    This section cites Loopy's "[The skinny on NES scrolling](PPU_scrolling.xhtml "The skinny on NES scrolling")", which dates nestech.txt after "Skinny".
O. PPU Quirks
    It mentions the 1-byte lag and the lack thereof in palette memory, but it doesn't mention how bits 3-0 read back when grayscale mode ($2001 bit 0) is on.

## pAPU

To be written.
    This dates nestech.txt before Brad Taylor's APU document. Frustration with lack of APU documentation was one of the things that drove [Tepples](User_Tepples.xhtml "User:Tepples") toward the GBA scene for several years, another being the lack of any way to run homebrew on an NES without [modding donors](Mask_ROM_pinout.xhtml#Converting_a_donor_board "Mask ROM pinout").

## Joypads, paddles, expansion ports

A. General Information
    "Signature" applies only to the Four Score and Satellite accessories. Its mention outside that context is misleading. In addition, first-party and most licensed third-party controllers do not return 0 after the report; they return 1. (The third-party U-Force controller does return 0.)
D. Paddles
    The description of D1 of $4016 and D1 of $4017 is for the Famicom version of the [Arkanoid controller](Arkanoid_controller.xhtml "Arkanoid controller"). The NES version uses D3 and D4 of $4017 instead.
H. Expansion ports
    Holding the strobe output ($4016 bit 0) high doesn't allow "communicating with the expansion port." Some specialized controllers, such as the Power Glove, use the strobe for outgoing serial communication, but not nearly in the manner described here.

## Memory Mapping Hardware

This section cites FireBug's mappers.nfo, which dates nestech.txt after mappers.nfo. 

## Registers

$2000 D6: PPU Master/Slave Selection
    Not "unused". The "slave mode" ($2000 bit 6 = 1) enables output of bits 3-0 of the current color on four PPU pins, which later NES RGB mods use. The "master mode" ($2000 bit 6 = 0) uses these pins as an input.
$2001 D7-D5: Full Background Colour (when D0 == 1)
    This only applies "Colour Intensity" to the grayscaled output. And plenty of games "use more than one type".
$2001 D0
    Display Type
    It doesn't specify how the monochrome display is achieved, which is by ANDing all values read from the palette with $30.
$2002 D5: Scanline Sprite Count
    Implies that the bit turns off once [PPU sprite evaluation](PPU_sprite_evaluation.xhtml "PPU sprite evaluation") no longer finds 8 sprites on the "current scanline". It actually turns off at the end of vblank. Nor does it mention the pathological diagonal fetch pattern, which was discovered later.
$2002 D4: VRAM Write Flag
    Does not exist on the NES. May have been a misinterpretation of open bus behavior.
$2003 | SPR-RAM Address Register (W)
    Does not mention glitches in the OAM DRAM controller that corrupt OAM when this is written.
$4015, $4017
    [APU Frame Counter](APU_Frame_Counter.xhtml "APU Frame Counter") was not understood, and it was confused with the Vertical Clock Signal coming from the PPU.

## File Formats

A. iNES Format
    Describes the iNES 0.7 variant of the [.NES format](INES.xhtml "INES"), without the Vs. bit, PRG RAM size, or TV system bytes.

## Programming the NES

C. PPU Notes
    The vblank wait that it recommends (spinning on $2002) is affected by a race condition. It's useful [during power-up](PPU_power_up_state.xhtml "PPU power up state"), but afterward an [NMI](NMI.xhtml "NMI") handler should be used. (It doesn't mention power-up restrictions at all.) And you don't have to "clear the internal VRAM address via $2006", which appears to be a misguided attempt to set the scroll. True, licensed games do this, but just because licensed games do something doesn't mean you have to [join the same cargo cult](https://en.wikipedia.org/wiki/Cargo_cult_programming "wikipedia:Cargo cult programming") with your own original programs. Licensed programmers didn't have perfect docs either. Make sure to finish your VRAM update code and [set the scroll position](PPU_scrolling.xhtml#The_common_case "PPU scrolling") before the end of vertical blanking, and it'll be fine.

## Emulation

A. General Information
    Open bus section contradicts itself. Reads from a nonexistent SRAM are said to "return data previously left on the bus", but emulators are advised to "return 0" instead. Returning 0 breaks [tricky games](Tricky_to_emulate_games.xhtml "Tricky-to-emulate games") that (intentionally or unintentionally) rely on values read from open bus, such as _Low G Man_ and _Earthworm Jim 2_. And the initial state of internal RAM at cold boot isn't all zeroes.
B. CPU Notes
    The 6502 core is correctly identified as the NMOS version. But the [CPU unofficial opcodes](CPU_unofficial_opcodes.xhtml "CPU unofficial opcodes") are called "bad", and it recommends that emulators "ignore" them. Treating all of them as synonymous with $EA (1 byte NOP) breaks a few later licensed games.

## Reference Material

Full of broken links, which we can forgive. 

  * [Nintendo Entertainment System documentation](https://nesdev.org/ndox200.zip)



Categories: [History](Category_History.xhtml)
