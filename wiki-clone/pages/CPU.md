# CPU

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/CPU) | View [other pages](Special_AllPages.xhtml#CPU)

The NES CPU core is based on the 6502 processor and runs at approximately 1.79 MHz (1.66 MHz in a PAL NES). It is made by [Ricoh](http://en.wikipedia.org/wiki/Ricoh) and lacks the MOS6502's decimal mode. In the NTSC NES, the [RP2A03](http://en.wikipedia.org/wiki/Ricoh_2A03) chip contains the CPU and APU; in the PAL NES, the CPU and APU are contained within the [RP2A07](http://en.wikipedia.org/wiki/Ricoh_2A03) chip. 

## Contents

  * 1 Sections
  * 2 Frequencies
  * 3 Notes
  * 4 See also
  * 5 References



## Sections

  * [CPU instructions](6502_instructions.xhtml "6502 instructions")
  * [CPU addressing modes](CPU_addressing_modes.xhtml "CPU addressing modes")
  * [CPU memory map](CPU_memory_map.xhtml "CPU memory map")
  * [CPU power-up state](CPU_power_up_state.xhtml "CPU power up state")
  * [CPU registers](CPU_registers.xhtml "CPU registers")
  * [CPU status flag behavior](Status_flags.xhtml "CPU status flag behavior")
  * [CPU interrupts](CPU_interrupts.xhtml "CPU interrupts")
  * [Unofficial opcodes](CPU_unofficial_opcodes.xhtml "CPU unofficial opcodes")
  * [CPU pin-out and signals](CPU_pinout.xhtml "CPU pinout"), and other [hardware pin-outs](Hardware_pinout.xhtml "Hardware pinout")



## Frequencies

The CPU generates its clock signal by dividing the master clock signal. 

Rate | NTSC NES/Famicom | PAL NES | Dendy   
---|---|---|---  
Color subcarrier frequency _f sc_ (exact) | 3579545.45 Hz (315/88 MHz) | 4433618.75 Hz | 4433618.75 Hz   
Color subcarrier frequency _f sc_ (approx.) | 3.579545 MHz | 4.433619 MHz | 4.433619 MHz   
Master clock frequency 6 _f sc_ | 21.477272 MHz | 26.601712 MHz | 26.601712 MHz   
Clock divisor _d_ | 12 | 16 | 15   
CPU clock frequency 6 _f sc_/_d_ | 1.789773 MHz (~559 ns per cycle) | 1.662607 MHz (~601 ns per cycle) | 1.773448 MHz (~564 ns per cycle)   
  
* The vast majority of PAL famiclones use a chipset or NOAC with this timing. A small number have UMC UA6540+6541, which also uses PAL NES timing.[1]

## Notes

  * All illegal 6502 opcodes execute identically on the 2A03/2A07.
  * Every cycle on 6502 is either a read or a write cycle.
  * A printer friendly version covering all section is available [here](CPU_ALL.xhtml "CPU ALL").
  * Emulator authors may wish to emulate the NTSC NES/Famicom CPU at 21441960 Hz ((341×262−0.5)×4×60) to ensure a synchronised/stable 60 frames per second.[2]



## See also

  * [Cycle reference chart](Cycle_reference_chart.xhtml "Cycle reference chart")
  * [2A03 technical reference](http://nesdev.org/2A03%20technical%20reference.txt) by Brad Taylor. (Pretty old at this point; information on the wiki might be more up-to-date.)



## References

  1. ↑ [nesdev forum: Eugene.S provides a list of famiclones](https://forums.nesdev.org/viewtopic.php?f=3&t=17213#p216082)
  2. ↑ [nesdev forum: Mesen - NES Emulator](http://forums.nesdev.org/viewtopic.php?p=223679#p223679)


