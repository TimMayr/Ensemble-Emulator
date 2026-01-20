# Errata

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Errata) | View [other pages](Special_AllPages.xhtml#Errata)

This page describes aspects of the NES hardware that may be considered an error in the implementation (**errata**), or just otherwise unintuitive or unexpected behaviour. 

Many of these issues are poorly emulated, and can cause [program compatibility](Program_compatibility.xhtml "Program compatibility") problems, especially for homebrew games not tested on hardware. 

## Contents

  * 1 Video
    * 1.1 OAM and Sprites
  * 2 Input
  * 3 Audio
    * 3.1 APU Pulse
    * 3.2 APU DMC
  * 4 CPU
  * 5 References



## Video

  * Setting the VRAM address using [PPUADDR](PPU_registers.xhtml "PPUADDR") ($2006) corrupts the [scroll position](PPU_scrolling.xhtml "PPU scrolling"). (Workaround: Reset the scroll position using [PPUSCROLL](PPU_registers.xhtml "PPUSCROLL") ($2005) and [PPUCTRL](PPU_registers.xhtml "PPUCTRL") ($2000) after finishing all background updates.)
  * Reading [PPUSTATUS](PPU_registers.xhtml "PPUSTATUS") ($2002) at the exact same time that [PPUSTATUS](PPU_registers.xhtml "PPUSTATUS") bit 7 goes high at the start of vertical blanking [keeps $2002.D7 from going high at all](NMI.xhtml#Race_condition "NMI") that frame. (Workaround: Use NMI to wait for vertical blanking.)
  * If the program changes the vblank NMI from disabled to enabled through [PPUCTRL](PPU_registers.xhtml "PPUCTRL") bit 7 while the vblank flag ([PPUSTATUS](PPU_registers.xhtml "PPUSTATUS") bit 7) is set, an NMI will trigger immediately. This can cause NMI to occur other than at the start of vblank, or cause more than one NMI in a single vblank, as long as it is still during vertical blanking and the program has not yet read PPUSTATUS. (Workaround: Read PPUSTATUS shortly before enabling NMIs.)
  * Writing to [PPUCTRL](PPU_registers.xhtml "PPUCTRL") ($2000) at the exact start of horizontal blanking may cause the PPU to start reading from the left name table instead of the right. Workarounds: 
    1. Use horizontal or one-screen mirroring.
    2. Don't write to [PPUCTRL](PPU_registers.xhtml "PPUCTRL") outside vertical or forced blanking except as part of a properly timed raster effect. If you are writing to [PPUCTRL](PPU_registers.xhtml "PPUCTRL") as a way of temporarily preventing the NMI handler from being called while it is already running, don't disable NMI through [PPUCTRL](PPU_registers.xhtml "PPUCTRL"). Instead, use a variable to lock out reentrant NMI, and check this variable at the beginning of your NMI handler.[1]
    3. Write to either $2000 or the mirror of [PPUCTRL](PPU_registers.xhtml "PPUCTRL") at $2100, depending on the desired value of the least significant bit that will be written to [PPUCTRL](PPU_registers.xhtml "PPUCTRL").[2]
  * After reset ends (by CIC or reset button), the [PPU refuses to accept data](PPU_power_up_state.xhtml "PPU power up state") written to the registers at [PPUCTRL](PPU_registers.xhtml "PPUCTRL"), [PPUMASK](PPU_registers.xhtml "PPUMASK"), [PPUSCROLL](PPU_registers.xhtml "PPUSCROLL") ($2005), and [PPUADDR](PPU_registers.xhtml "PPUADDR") ($2006) for about the length of one frame. NES startup code should wait at least this long after reset before using the PPU (see: [Init code](Init_code.xhtml "Init code")).
  * The VBlank flag (bit 7) of [PPUSTATUS](PPU_registers.xhtml "PPUSTATUS") is not cleared on reset, only power-up.
  * Some 6502 write instructions produce an extra read or write as part of their operation, which produces unexpected results when used with PPU registers [OAMDATA](PPU_registers.xhtml "OAMDATA") ($2004) and [PPUDATA](PPU_registers.xhtml "PPUDATA") ($2007). This includes all read-modify-write instructions (ASL, LSR, ROL, ROR, DEC, INC) as well as indexed addressing instructions (e.g. STA $2000, X). Spurious reads or writes that operate on these PPU registers are a problem, because they have the side effect of incrementing its internal write address.
  * [2C07 and Dendy only] The PAL red and green emphasis bits are swapped with respect to the 2C02 (NTSC).
  * [2A03] DMC DMA during reads from $2007 can cause an extra read signal, causing a lost byte. This issue can also affect $2002 vblank polling (though this is already unreliable, see above). Workaround: disable DMC while reading from PPU.
  * Setting bit 6 of PPUCTRL (EXTBG direction) on a stock NES to 1 (output) causes a bus conflict that can potentially damage the PPU. See _[master/slave_mode_and_the_EXT_pins](PPU_registers.xhtml#Master/slave_mode_and_the_EXT_pins "PPU registers")_.
  * A Y scroll position in 240-255 is [treated as "negative"](PPU_scrolling.xhtml#Y_increment "PPU scrolling"), rendering the attribute table as two rows of garbage tiles.
  * Setting PPUADDR to a palette address while rendering is disabled will cause the PPU to render the color at that address. This means updating palettes outside of vblank can cause a flash even if rendering is disabled.
  * Changing PPUADDR from $3Fxy (y≠0) to any non-palette address may, in unknown circumstances, corrupt some colors in the palette. See _[PPUADDR § Palette corruption](PPU_registers.xhtml#Palette_corruption "PPU registers")_
  * Color $0D in the palette is "blacker than black" and causes image stability problems on some TVs. Do not use this color.
  * In a mid-scanline first write to [PPUSCROLL](PPU_registers.xhtml "PPUSCROLL"), the PPU starts reading the value a bit too early, catching a CPU open bus value that usually comes from the high bit of the address and setting the fine X scroll to open bus D2-D0 for about a pixel. Workaround: If writing fine X early, write to a PPUSCROLL mirror at $2005, $2105, ..., or $2705 to match either the old or new fine X value.



### OAM and Sprites

  * Sprite 0 hit does not trigger at x=255.
  * Sprite overflow is unreliable due to errors in its implementation. The internal copy to secondary OAM causes a [diagonal fetch pattern](PPU_sprite_evaluation.xhtml "PPU sprite evaluation"), causing both false positives and false negatives in the sprite overflow bit. (Workaround: Make sure the ninth sprite immediately follows the eighth, and use sprite overflow only to time the top of the screen, not the bottom.) See: [Sprite overflow bug](PPU_sprite_evaluation.xhtml#Sprite_overflow_bug "PPU sprite evaluation").
  * Because OAM is DRAM, which needs to be refreshed frequently, the contents of OAM begin to decay quickly when rendering is turned off via [PPUMASK](PPU_registers.xhtml "PPUMASK") ($2001). During rendering, sprite evaluation will continually refresh the OAM data. Caveats and workarounds: 
    * The data remains reliably intact for about the length of NTSC's vblank, but longer than this and it will begin to corrupt itself.[3]
    * Using OAM DMA ($4014) during vblank is usually the best way to fill the entire OAM buffer during vblank.
    * Writes to OAMDATA ($2004) are usually too slow to fill the entire OAM buffer before it begins to decay. It can be used during vblank to update a few bytes of OAM.
    * PAL systems must execute OAM DMA early within vblank, because sprite evaluation begins partway through vblank to keep it refreshed during its extended duration.[4]
  * Reading from OAM is inconsistent or unusable depending on the hardware revision. This is due to differences in its DRAM controller, and its lack of reliability in edge cases: 
    * [2C02 through 2C02E (early Famicoms and very early NESes), 2C03B, 2C03C (Vs. System PPUs)] OAM is simply never readable.
    * [2C02G, 2C03G, and 2C02H] Writes to [OAMADDR](PPU_registers.xhtml "OAMADDR") ($2003) corrupt OAM. (Workaround: Rewrite entire OAM before rendering starts, possibly using DMA initiated by writes to $4014, or rely on [OAMADDR](PPU_registers.xhtml "OAMADDR") being 0 at end of rendering.)
    * [2C07 and 2C07A] OAM cannot be accessed from scanlines 21 through 70, after the NMI would have happened. (To compensate for PAL's longer vblank period, the 2C07 always enables the OAM refresh logic for part of the blanking period.)
  * [2C02G and 2C02H, others?] Leaving the value in [OAMADDR](PPU_registers.xhtml "OAMADDR") (either written or by autoincrement) at a value of eight or greater before rendering starts causes minor OAM corruption, copying the eight bytes at _OAMADDR &~7_ to the beginning of OAM.[5]
  * Turning rendering off or on mid-frame via [PPUMASK](PPU_registers.xhtml "PPUMASK") ($2001) has multiple issues: 
    * See: [PPU sprite evaluation: Rendering disable or enable during active scanline](PPU_sprite_evaluation.xhtml#Rendering_disable_or_enable_during_active_scanline "PPU sprite evaluation").
    * Turning rendering off before the PPU has finished [evaluating sprites for that line](PPU_sprite_evaluation.xhtml "PPU sprite evaluation") (x=192 for lines with no sprites, x=240 for lines with at least one sprite) can corrupt OAM, leading to sprite flicker.
    * Turning rendering on mid-frame has other problems not fully described yet.



## Input

  * [2A03] DMC DMA during a controller read ($4016/$4017) causes double clocking, which causes bits of the report to be skipped. A common symptom is spurious presses of Right. Workarounds: 
    * [Standard controller](Standard_controller.xhtml "Standard controller") / [Power Pad](Power_Pad.xhtml "Power Pad"): read the controller multiple times to make sure valid input is read.
    * [Four Score](Four_player_adapters.xhtml "Four Score"): check for signature.
    * [Arkanoid controller](Arkanoid_controller.xhtml "Arkanoid controller"): prevent a sudden single frame acceleration.
    * If already using DMC IRQ: read the controller in the DMC IRQ handler.
    * Run an [OAM DMA before reading the controller](Controller_reading_code.xhtml#DPCM_Safety_using_OAM_DMA "Controller reading code") to align the CPU clock to the even/odd phase of the APU clock.



## Audio

### [APU Pulse](APU_Pulse.xhtml "APU Pulse")

  * In sweep decrease mode, the carry input differs between the two channels, causing a slightly different sweep rate.
  * Channels set to low frequencies can get silenced by the sweep unit if it is left in increase mode, even if the sweep is otherwise disabled. Workaround: 
    * Write $08 to $4001 and $4005 to use decrease mode while disabling sweep.
  * Writing to $4003 or $4007 to change the high byte of the period while a note is playing causes a click as the phase resets. Workarounds: 
    1. Write $4003 and $4007 only when they have changed.
    2. The hardware sweep can be used to change the high period bits by clocking it immediately with a write to $4017.[6] (Advanced technique.)



### [APU DMC](APU_DMC.xhtml "APU DMC")

  * The length counter for DPCM samples ends up reading 1 byte past the end of an otherwise 16 byte aligned sample. This creates a need for 15 bytes of padding between samples.
  * The sample playback frequency table contains a set of 16 pitches tuned to a standard A-440 scale. These appear to have been designed for a limited wavetable synthesis using looped samples, but because of the +1 modifier on sample length, the wavelength is detuned. (Workaround: Use a 17-byte loop and the key of B major.)
  * The frequency table on PAL systems contains 2 slight tuning errors ($4 and $C).[7]
  * Playback of samples generates occasional conflicts with controller reads through $4016/4017 (see above: Input), as well as PPU reads through $2007 (see above: Video). When using DPCM samples, the code must work around these conflicts.



## CPU

The NES CPU has several hardware gotchas, most of which are inherited from the MOS 6502 it is derived from:[8][9]

  * _JMP ($xxyy)_ , or JMP indirect, does not advance pages if the lower eight bits of the specified address is $FF; the upper eight bits are fetched from _$xx00_ , 255 bytes earlier, instead of the expected following byte.
  * All of the zero page addressing modes wrap within the zero page. The _$xx,x_ , _$xx,y_ , and _($xx,x)_ addressing modes all count 254,255,0,1…; none advance 254,255,256,257…
  * The _($xx),y_ addressing mode wraps when fetching the indirect address if the lower eight bits are stored at $FF (the upper eight bits are fetched from $0000, not $0100).
  * BRK, IRQ, or NMI can mask each other under certain conditions. (see Visual6502 wiki [[1]](http://visual6502.org/wiki/index.php?title=6502_BRK_and_B_bit) and [[2]](http://visual6502.org/wiki/index.php?title=6502_Timing_of_Interrupt_Handling) ) Not all can happen on the NES.
  * Decimal mode was disconnected from the ALU in the NES's second-source 6502 to save on patent royalties. Some famiclones, however, use an authentic 6502 with a working decimal mode. (Workaround: Don't SED, and convert binary numbers to decimal when displaying them.)
  * The 6502 has several [unofficial opcodes](CPU_unofficial_opcodes.xhtml "CPU unofficial opcodes") that were not part of its specification, but do reliably exist on the Famicom and NES. Code using these opcodes may not be portable to other variants of the 6502 CPU (e.g. SNES). Some games rely on the operation of these unofficial opcodes.
  * Page wrapping behavior is reliable on the Famicom/NES but may not be portable to other variants of the 6502 CPU.



## References

  1. ↑ [Forum](http://forums.nesdev.org/viewtopic.php?f=2&t=10104): Random glitchy line in Super Mario Bros. on real hardware?
  2. ↑ [Forum](https://forums.nesdev.org/viewtopic.php?p=230434#p230434): 2nd2006_next_level test rom and extensions
  3. ↑ [Forum](http://forums.nesdev.org/viewtopic.php?f=9&t=9912): Just how cranky is the PPU OAM?
  4. ↑ [Forum](http://forums.nesdev.org/viewtopic.php?f=9&t=11041): PAL OAM reliability during vblank.
  5. ↑ [Forum](http://forums.nesdev.org/viewtopic.php?p=110019#p110019): Re: Just how cranky is the PPU OAM? (test notes by quietust)
  6. ↑ [Forum](http://forums.nesdev.org/viewtopic.php?t=231): Vibrato on square without phase reset.
  7. ↑ [Forum](http://forums.nesdev.org/viewtopic.php?p=94079#p94079): PAL DPCM frequency table contains 2 errors.
  8. ↑ [6502.org forum](http://forum.6502.org/viewtopic.php?t=770): 6502 hardware gotchas.
  9. ↑ [Wikipedia's article on the 6502](https://en.wikipedia.org/wiki/MOS_Technology_6502#Bugs_and_quirks "wikipedia:MOS Technology 6502").


