# Miracle Piano

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Miracle_Piano) | View [other pages](Special_AllPages.xhtml#Miracle_Piano)

The Miracle Piano is a 49-key MIDI synthesizer built by Antex on behalf of The Software Toolworks with options for using RS-232 serial and a synchronous clocked-bit protocol as MIDI transport. It is part of The Miracle Piano Teaching System, which had software implementations for at least NES, SNES, Genesis, PC, and Macintosh. 

The NES implementation of the software expects the "Miracle Cable" to be connected to controller port 1, and a standard controller to port 2. 

## Keyboard Interfacing

The keyboard interface is bi-directional, and appears to be via clocked shift registers. Data is comprised of octets sent and received MSB-first. 

Data is polled from the keyboard by raising OUT0 for 12 CPU cycles (about 6.7 μSec), lowering it, and clocking a bit in by reading $4016. May need to burn six cycles (about 3.3 μSec) before clocking the bit in. If this bit is set, there are an additional eight (inverted) bits to clock in. This process strobes OUT0, so polling the status of a controller connected to the other port is possible at this time. 

Data is sent to the keyboard by raising OUT0 for about 79 CPU cycles (about 44.1 μSec) (from OUT0 raised to OUT0 set with the first data bit). After this, eight data bits are sent by setting OUT0 to each bit and clocking the bits out by reading $4016. 

Inter-bit time for transmission is about 16 cycles (about 8.9 μSec). Inter-bit time for reception is about 18 cycles (about 10 μSec). 

## References

  * [[1]](http://weltenschule.de/TableHooters/Mindscape_MiraclePiano.html) (has some description of features and a review of the hardware).
  * [[2]](http://www.pianoeducation.org/pnompfqn.html) (has some manual scans, schematics for two circuit boards in the piano itself, link for information on building replacement "miracle cables", and a FAQ).


