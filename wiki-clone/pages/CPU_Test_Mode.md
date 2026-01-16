# CPU Test Mode

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/CPU_Test_Mode) | View [other pages](Special_AllPages.xhtml#CPU_Test_Mode)

Pin 30 on the 2A03G and 2A03H provides special functionality in some revisions of the chip and is normally grounded. 

## Contents

  * 1 2A03G / 2A03H Test Mode
  * 2 2A03E
  * 3 2A07A
  * 4 2A03
  * 5 See also



## 2A03G / 2A03H Test Mode

Pin 30 of the 2A03G and 2A03H can be asserted to enable a special test mode for the APU. This activates registers for testing the APU at $4018-$401A at the expense of deactivating the joypad input registers at $4016-$4017: 
    
    
    R$4018: [BBBB AAAA] - current instant DAC value of B=pulse2 and A=pulse1 (either 0 or current volume)
    R$4019: [NNNN TTTT] - current instant DAC value of N=noise (either 0 or current volume) 
                          and T=triangle (anywhere from 0 to 15)
    R$401A: [.DDD DDDD] - current instant DAC value of DPCM channel (same as value written to $4011)
    W$401A: [L..T TTTT] - set state of triangle's sequencer to T, and lock all channels if L=1
                          (pulse+noise always output current volume, triangle/DPCM no longer advance)
    

Test mode disconnects the external CPU bus from the internal bus when reading from any of $4000-401F, just as normally happens when reading from $4015; this is what disables the joypads, by preventing the CPU from seeing the value they put on the bus. Test mode also causes the CPU to continue outputting M2 while held in reset. 

Connecting pin 30 to CPU A3 allows the test and controller registers to coexist, at the cost of increased likelihood of DMC DMA corruption when the CPU is reading from $4000-401F (see [DMA register conflicts](DMA.xhtml#Register_conflicts "DMA")). Because the state of A3 is effectively random when reset begins and because the address lines go high impedance during reset, a weak pulldown (10k Ohm) should be used to prevent pin 30 from floating while reset is held and restore standard M2-during-reset behavior. 

## 2A03E

On the 2A03E, pin 30 functions as an external /RDY input, and the [Playchoice 10](https://nesdev.org/Playchoice.pdf) supervisor CPU uses this to reset the 2A03E when the player runs out of time (by driving pin 30 high to halt it, then driving pin 30 low and pulling /RESET low to reset it). Driving pin 30 high and then low has been observed to simply crash the 2A03E. 

## 2A07A

Just as with the 2A03E, pin 30 on the 2A07 is an external /RDY input. 

## 2A03

In the original version of the 2A03, pin 30 is not connected to anything at all. 

## See also

  * Forum: [Breaking NES apart](http://forums.nesdev.org/viewtopic.php?f=9&t=9197)
  * See: [File:Apu address.jpg](File_Apu_address_jpg.xhtml "File:Apu address.jpg")
  * See: [CPU pin out and signal description](CPU_pinout.xhtml "CPU pin out and signal description")


