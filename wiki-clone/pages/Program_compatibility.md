# Program compatibility

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Program_compatibility) | View [other pages](Special_AllPages.xhtml#Program_compatibility)

    _This page describes defects in homebrew games. For defects in games prior to 1996, see[Game bugs](Game_bugs.xhtml "Game bugs")._

Homebrew development is as subject to bugs as old software was, but many suffer from additional [compatibility problems](Accuracy.xhtml "Accuracy") with the NES hardware due to being tested on emulators exclusively. It is often the case that early homebrew ROMs will not run correctly on hardware. Since the release of the [PowerPak](PowerPak.xhtml "PowerPak") in 2008, and general improvement in emulators over time, these problems have become less frequent. 

For a partial list of homebrew projects, see: [Projects](Projects.xhtml "Projects")

## Common compatibility problems

This is a list of issues that commonly appear due to lack of popular support in emulators, or sometimes even hardware limitations of flash-carts like the [PowerPak](PowerPak.xhtml "PowerPak"). Many emulators are merely trying to be compatible with existing games, rather than accurately reflecting the hardware, making them inadequate for verifying the correctness of new software. 

  * Use of DPCM samples causes a [conflict](APU_DMC.xhtml "DPCM conflict") with controller and PPU reads during sample playback.
  * Writes to [PPU registers](PPU_registers.xhtml "PPU registers") outside of [VBlank](The_frame_and_NMIs.xhtml#VBlank,_Rendering_Time,_and_NMIs "The frame and NMIs") with rendering enabled conflicts with internal use of the PPU address, causing graphical corruption.
  * Failure to initialize registers not guaranteed by the [CPU power up state](CPU_power_up_state.xhtml "CPU power up state") or the [PPU power up state](PPU_power_up_state.xhtml "PPU power up state").
  * Failure to initialize RAM, nametables, or [mapper](Mapper.xhtml "Mapper") registers. (Few mappers have guaranteed power up states.)
  * Failure to delay use of the PPU after power/reset to avoid conflicts with its [warm up state](PPU_power_up_state.xhtml "PPU power up state").



## References
