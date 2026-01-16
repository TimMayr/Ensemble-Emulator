# Interrupt forwarding

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Interrupt_forwarding) | View [other pages](Special_AllPages.xhtml#Interrupt_forwarding)

Instead of loading each build of an NES program onto a cartridge, some developers prefer to send code directly to RAM for more rapid development. Because the NMI and IRQ vectors are in ROM, the program in ROM needs to forward these to vectors in RAM. There are at least three known protocols for NMI and IRQ forwarding. 

The Famicom Disk System uses [this protocol](FDS_BIOS.xhtml#Interrupt/Reset_vector_controls "FDS BIOS"): 

  * NMI is JMP ($DFF6), JMP ($DFF8), or JMP ($DFFA) based on a control flag stored at $0100
  * Reset is JMP ($DFFC) so long as a signature at $0102-$0103 matches
  * IRQ and BRK are JMP ($DFFE)



A simplified version of this protocol is used in the [romless NES program format](http://slack.net/~ant/old/nes-code/romless/), a subset of [iNES](INES.xhtml "INES") format designed by Blargg to support a [bootloader cable](http://slack.net/~ant/old/nes-code/bootloader/). 

  * NMI is JMP ($07FA)
  * Reset is JMP ($07FC)
  * IRQ and BRK are JMP ($07FE)
  * Detection of a cold boot by the bootloader is unspecified



The Super FX coprocessor for the Super NES implements a similar protocol, but with each vector in RAM being an actual jump instruction instead of just an address. As described in [Martin Korth's FullSNES doc](http://problemkaputt.de/fullsnes.txt), while the GSU is running, it takes the PRG ROM off the S-CPU's bus and switches in a 16-byte pseudo-ROM that contains only vectors to RAM: 
    
    
    .addr $0100,$0100,$0104,$0100,$0100,$0108,$0100,$010C
    

This produces effects equivalent to the following, which let the S-CPU run in RAM and handle interrupts while the GSU accesses ROM. 

  * Reset and BRK (native) are JML $000100
  * COP is JML $000104
  * NMI is JML $000108
  * IRQ and BRK (emulation) are JML $00010C


