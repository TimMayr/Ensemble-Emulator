# MK5060 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/MK5060_pinout) | View [other pages](Special_AllPages.xhtml#MK5060_pinout)

This chip was used at least in Micro Genius IQ-201 (MK5060) [[1]](http://forums.nesdev.org/viewtopic.php?f=9&t=7912&view=next) and an aftermarket mod board for the PC Engine (NEC MK5060-A) [[2]](http://notinmame.blogspot.com/2015/11/pc-engine-core-grafx-pal-rf-to-rgb.html). Its role was to halt the PPU for 3.3 ms at the end of frame (but before the VBLANK). This was achieved by disabling the PPU's primary clock for that time. That way, frame period (NMI frequency) was extended from 16.7 ms (60 Hz) to 20 ms (50 Hz), slowing down the game and audio speed, but keeping PPU/CPU clock ratio and thus saving NTSC compatibility intact. 

External components are used to extract from the composite video signal frame and scanline start pulses, which are used to clock the internal counter. At the last render scanline, chip puts PPU into sleep mode, which lasts 3.3ms. MK5060 also watches the PPU/CE line, probably in case if CPU requested to read/write from PPU at the time it is put into sleep, it would be immediately woken up. Additionally, if PPU rendering is disabled during frame, it fools MK5060. 

Early Hong Kong Famicoms (HVC-CPU-NPC-18-01) use the MK5060, but later ones (HVC-CPU-NPC-26-01) use a different, Nintendo-branded chip (20-pin N NPC26) that serves a similar purpose. 
    
    
                                           .--\/--.
    high during frame sync & back porch ?? |01  24| -- +5V
                                   GND  -- |02  23| ?? wired to pin 1
                                    NC  ?? |03  22| -- GND
                   colorbust shift 240* <- |04  21| <- saturation input?
                   colorbust shift 120* <- |05  20| -- +5V
                   colorbust shift 0*   <- |06  19| -> \ 21.x MHZ clock  to PPU
            21.x MHz clock from famicom -> |07  18| -> / 
             high during frame sync     <- |08  17| -> 4.43365 MHz clock output
              low during frame sync     <- |09  16| <- 17.7345 MHz clock input
                             PPU /CE    -> |10  15| <- hue input? (?)
                    60Hz (H) / 50Hz (L) -> |11  14| <- scanline sync pulse (?)
                                    GND -- |12  13| -- +5V
                                           '------'
    

Categories: [Pinouts](Category_Pinouts.xhtml)
