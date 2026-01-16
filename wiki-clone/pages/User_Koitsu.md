# User:Koitsu

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AKoitsu) | View [other pages](Special_AllPages.xhtml#User_Koitsu)

Previously known as Y0SHi or Yoshi. 

Home page: <https://jdc.koitsu.org/>

## Contents

  * 1 Interrupts on the NES and 6502
    * 1.1 RESET
    * 1.2 NMI and IRQ/BRK
      * 1.2.1 IRQ
      * 1.2.2 BRK
      * 1.2.3 Distinguishing IRQ from BRK
    * 1.3 The relationship between NMI and VBlank
    * 1.4 Other info



# Interrupts on the NES and 6502

The 6502 has 3 hardware interrupts: **RESET** , **NMI** , and **IRQ**. Physically, these are pins/wires that connect to something on the other end. 

When an interrupt fires, the 6502 will execute code for the respective interrupt via a "vector address". These are hard-coded addresses in the CPU that are used, containing a 16-bit pointer: 

Interrupt name | Vector (16-bit)   
---|---  
NMI | $FFFA-FFFB   
RESET | $FFFC-FFFD   
IRQ/BRK | $FFFE-FFFF   
  
"How" the interrupt fires varies per platform; from a software programming perspective it isn't relevant or of interest. If you want to understand the hardware behaviour, refer to [CPU interrupts](CPU_interrupts.xhtml "CPU interrupts"). 

## RESET

The CPU will effectively do an indirect jump, e.g. `JMP ($FFFC)`. This happens on power-on and on reset (physically pressing the reset button on the NES). 

Example: if $FFFC-FFFD contained bytes `$4A $93` then on power-on or reset, the CPU would begin executing code at address `$934A`. 

## NMI and IRQ/BRK

The CPU does something akin to an indirect jump subroutine, e.g. `JSR ($vector)`, with the addition of the `P` (CPU status register) pushed onto the stack. For exact behaviour of what the CPU does when an interrupt fires, including what gets pushed onto the stack for NMI and IRQ/BRK, refer to [Interrupts in 65xx processors - Interrupt types](https://en.wikipedia.org/wiki/Interrupts_in_65xx_processors#Interrupt_types) on Wikipedia. 

These routines should be returned from using the `rti` instruction, not `rts`. 

### IRQ

On the NES, the IRQ line is multi-purpose and complicated: 

  * Wired to the cartridge port so that mappers like MMC3 can generate an IRQ that happens once per scanline, allowing you to run code in HBlank
  * Used by the APU (audio) circuitry for PCM/DMC playback



Refer to the nesdev Wiki for details on the related mapper, or the APU, or ask someone familiar with these things -- especially PCM/DMC playback, as there are many gotchas, including quirks relating to reading from the controller port while playing a PCM sample (really!). 

The value of bit 4 of P pushed onto the stack will be 0. **The actual value of bit 4 in P is not changed, only the value that's pushed on the stack.**

### BRK

The `brk` instruction on the 6502 will actually trigger a form of software interrupt. As such, it shares the same 6502 vector as a real hardware IRQ. 

The value of bit 4 of P pushed onto the stack will be 1. **The actual value of bit 4 in P is not changed, only the value that's pushed on the stack.**

While `brk` is technically a 1-byte instruction ($00), it contains a "signature byte" which can be used for various purposes (e.g. `brk $20` could indicate a bug in a nametable routine, while `brk $3e` could refer to a bug in one's NMI routine). 

There are also some quirks pertaining to BRK and "interrupt conflicts" described in the [CPU interrupts](CPU_interrupts.xhtml "CPU interrupts") section. 

  * TODO: Add links to people who have added their own brk handlers in games (ex. Lizard, etc.).



### Distinguishing IRQ from BRK

To distinguish the difference between a hardware vs. software interrupt, the IRQ/BRK vector code can examine bit 4 of the CPU status flag that's pushed onto the stack: 0 means hardware interrupt (IRQ), 1 means software interrupt (e.g. BRK). 

For further details, please refer to [Brad Taylor's "the B flag & BRK instruction"](http://nesdev.org/the%20%27B%27%20flag%20&%20BRK%20instruction.txt) document. 

## The relationship between NMI and VBlank

On the NES, the 6502 NMI pin is physically wired to the VBlank signal from the PPU, with one extra "piece" in the middle: bit 7 of [PPUCTRL](PPU_registers.xhtml "PPUCTRL") ($2000). When set to 1, NMI will fire once per VBlank; when set to 0, NMI will essentially not fire. 

The intended goal is to allow the CPU to run code at a specific/set rate (60 times a second on NTSC, 50/sec on PAL), and before the PPU begins drawing the screen. This gives the programmer time to do things like update nametables or whatever else. 

If you don't understand what VBlank is (sometimes referred to as vertical retrace, vertical refresh, or VBI (vertical blanking interval)), please refer to these below documents. It's all about understanding how a display (CRT, LCD, etc.) draws video data fed to it: 

  * [Computer Graphics Basics](https://www.tutorialspoint.com/computer_graphics/computer_graphics_basics.htm)
  * [Wikipedia - Raster scan](https://en.wikipedia.org/wiki/Raster_scan)
  * [Wikipedia - Vertical blanking interval](https://en.wikipedia.org/wiki/Vertical_blanking_interval)



The amount of time you have available in VBlank is thus effectively limited, and varies depending on NTSC vs. PAL. Spending too much time in VBlank/NMI can result in visual screen corruption or other anomalies. Knowing how many CPU cycles your NMI routine uses can be critical. Emulators like FCEUX and Mesen can help with this. 

Refer to these documents for additional details or help: 

  * [CPU cycle counts](Cycle_reference_chart.xhtml#CPU_cycle_counts "Cycle reference chart")
  * [The frame and NMIs](The_frame_and_NMIs.xhtml "The frame and NMIs")
  * [Performance Profiling on the NES](https://www.kickstarter.com/projects/1101008925/lizard/posts/1040806)
  * [Mesen - Profiler](https://www.mesen.ca/docs/debugging/memorytools.html#profiler)
  * [Mesen - Event Viewer](https://www.mesen.ca/docs/debugging/eventviewer.html)



## Other info

  * If the CPU is in the middle of executing an instruction when an interrupt fires, that instruction will finish/complete before handling the interrupt.


  * The instructions `sei` (inhibit interrupts) and `cli` (enable interrupts) only affect IRQ; they cannot be used to inhibit NMI (hence the term "non-maskable"). To effectively "inhibit NMI", set bit 7 of [PPUCTRL](PPU_registers.xhtml "PPUCTRL") ($2000) to 0.


