# File:Apu address.jpg

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/File%3AApu_address.jpg) | View [other pages](Special_AllPages.xhtml#File_Apu_address_jpg)

[![Media as described below](../wiki-images/Apu_address.jpg)](../wiki-images/Apu_address.jpg)

The NES APU's address decoder, generating enables for all reads/writes within $4000-$401F. 

Of very special note are the 4 signals at the very top for readable registers at $4018 (pulse 0 output on D0-D3 and pulse 1 output on D4-D7), $4019 (triangle output on D0-D3, noise output on D4-D7), $401A (DPCM output on D0-D6), and a writable register at $401A (set triangle position to D0-D4, and lock channel outputs using D7); all 4 of these signals have an additional enable which seems to come from the vicinity of [pin 30](CPU_pinout.xhtml "CPU pin out and signal description"). 

Source: <http://uxul.org/~noname/chips/cpu-5/stitched/final/> <http://uxul.org/~noname/chips/cpu-2/no-metal/stitched/final/>
