# Talk:Sachen 8259

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ASachen_8259) | View [other pages](Special_AllPages.xhtml#Talk_Sachen_8259)

## Pins

Whatever this IC looked like, if it was ever not just an epoxy blob, it probably existed in a 24-pin package. 

For the behavior described here, the following 22 signals must be attached to it: Vcc Gnd /ROMSEL CPU_A14 CPU_A8 CPU_A0 CPU_M2 CPU_D0 CPU_D1 CPU_D2 PRG_A15 PRG_A16 PRG_A17 PPU_A10 PPU_A11 PPU_A12 CHR_A11 CHR_A12 CHR_A13 CHR_A14 CHR_A15 CHR_A16 

Plus at least one of the following two: CIRAM_A10 or PPU_A13 

The behavior described on [iNES Mapper 137](INES_Mapper_137.xhtml "INES Mapper 137") is satisfied by the addition of one 74'153 as well as specifying one extra digital output for the contents of register 6. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 13:33, 31 January 2014 (MST) 

    Unfortunately, I don't know much about the chip(s); just what I've seen other emulators implement. But what you say does make sense. [Natt](User_Natt.xhtml "User:Natt") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Natt&action=edit&redlink=1 "User talk:Natt \(page does not exist\)")) 13:39, 31 January 2014 (MST)

    

    Evidently I'm wrong! The Pirated (Pirates') Games Forum had a thread ( s4.zetaboards.com/PGC_Forums/topic/9974526/1/ ) with a picture of Rockball's PCB. It was on the PCB **TC-A003-72P** and had two mask ROMs, a place for PRGRAM, a CIC stunner, and a 28-pin 0.6" DIP labelled SA8259A. ( img31.imageshack.us/img31/8521/r93f.jpg ). Cah4e3 has a [writeup](http://cah4e3.shedevr.org.ru/%5Blst%5D-sachen-mappers.txt), including a pinout, which I'll incorporate into the wiki here shortly. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 16:00, 31 January 2014 (MST)

## Indirect addressing

It'd be nice to be consistent across the wiki about how we refer to indirectly addressed mapper registers: 

  * Yamaha synthesizer IC datasheets call it address/data
  * The AY-3-8910 datasheet calls it 'latch address'/'write to psg'
  * The TI SN7649x datasheet calls it register address/data
  * [MMC3](MMC3.xhtml "MMC3") calls it bank/bank data
  * [Sachen 8259](Sachen_8259.xhtml "Sachen 8259") calls it select/(data/internal)
  * [FME-7](Sunsoft_FME_7.xhtml "FME-7") calls it command/parameter
  * [Sunsoft 5B audio](Sunsoft_5B_audio.xhtml "Sunsoft 5B audio") calls it audio register select/write
  * [VRC7 audio](VRC7_audio.xhtml "VRC7 audio") calls it register select/write



—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 13:33, 31 January 2014 (MST) 
