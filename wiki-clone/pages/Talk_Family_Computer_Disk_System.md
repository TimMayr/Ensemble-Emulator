# Talk:Family Computer Disk System

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AFamily_Computer_Disk_System) | View [other pages](Special_AllPages.xhtml#Talk_Family_Computer_Disk_System)

## Contents

  * 1 Typo?
  * 2 $eb13
  * 3 VRAM Buffer
  * 4 Pinout
  * 5 QDI format
  * 6 Disk Status Register ($4030) behavior
    * 6.1 Bit 3
  * 7 Battery bit in register $4033
  * 8 FDS Control ($4025) register
  * 9 FDS cycle time
  * 10 Documentation updates
    * 10.1 FDS
    * 10.2 FDS audio
    * 10.3 FDS disk format
    * 10.4 FDS file format(s)
    * 10.5 FDS BIOS
    * 10.6 Misc.



## Typo?

I think there was a mistake in the document, it said that GetDiskInfo was at E23A, and that makes no sense since it overlaps with another function. Another document said it was E32A (not E23A), so I changed it. --[Dwedit](https://www.nesdev.org/w/index.php?title=User:Dwedit&action=edit&redlink=1 "User:Dwedit \(page does not exist\)") 21:37, 11 October 2010 (UTC) 

Yes, it was a typo. Thank you for correcting it Bregalad 11:14, 12 October 2010 (UTC) 

## $eb13

Given a few minutes of analysis, the routine at $eb13 appears to read the [Family basic keyboard](Family_BASIC_Keyboard.xhtml "Family basic keyboard") state into $00-$08. --[Quietust](User_Quietust.xhtml "User:Quietust") 13:31, 13 October 2010 (UTC) 

Great finding Q ! What about completing the article about it ? Now only $e8b3 and $e94f are still somewhat obscure. Bregalad 21:03, 13 October 2010 (UTC) 

## VRAM Buffer

I'm trying to learn how the FDS works (hence my efforts to tidy up descriptions and such), and I'm a little confused. There's a buffer at $3xx which is what the various PrepareVRAMString routines and such copy to/from, but it doesn't support fills and doesn't support 32-byte increment? WriteVRAMBuffer is what you use to actually copy the $3xx buffer into VRAM, correct? 

That seems a little less useful than just simply using VRAMStructWrite, pointing to $0302, and gaining access to the extra features like fill and 32-byte increment. --[Drag](User_Drag.xhtml "User:Drag") 17:27, 26 February 2012 (PST) 

  
You are correct. The missing future are fill, 32-byte increment, but also sub-structures. The advantage, however, is that the second routine (WriteVRAMBuffer) is way faster than the first (VRAMStructWrite), as it doesn't have to check for those flags (which aren't available any longer), and uses direct addressing $302,X instead of indirect addressing to access the buffer. So the second routine is more suited to do VRAM updates during VBlank, while the first routine is more suited to do bulk updates during forced blanking, although technically both can do both. BTW thanks for fixing/correcting my work.Bregalad 12:33, 27 February 2012 (PST) 

    VRAM updates during VBlank like writing a column of a horizontally scrolling background? --[Tepples](User_Tepples.xhtml "User:Tepples") 12:38, 27 February 2012 (PST)

Well....... Then you either upload it a tile at a time or call the slow routine (both will be slow) or write your own (fast).Bregalad 13:18, 27 February 2012 (PST) 

Thanks! I wanted to see if it would be better to use the one in the BIOS first, before I went straight to just using my own. Thank you for clearing that up for me. :D --[Drag](User_Drag.xhtml "User:Drag") 18:01, 27 February 2012 (PST) 

## Pinout

Do you have information of Famicom Disk System pinout? --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 11:43, 28 September 2012 (MDT) 

## QDI format

I want to mention this here in case someone finds it useful. A .QDI file is a raw disk image for a single side of a single disk (including gaps, checksums, full size, etc). In order for an emulator to support it, I recommend the following: 

  * If loading a .NES ROM image with mapper 20, it should treat it as a BIOS ROM and start the FDS with no disk inserted. If a hard or soft reset is requested, and any .QDI is inserted, it should boot that disk side; if none is inserted, it should boot from no disk.
  * If loading a .FDS disk image, it should enable the "insert disk 1 side A" and so on, and treat it as it already does. (Some emulators may make a private copy in the user's home directory for saving (maybe even converting to .QDI upon first loading!); others may overwrite the existing .FDS instead.)
  * If loading a .QDI disk image, it should boot from that one, and have a "insert .QDI" option.
  * If possible, the emulator should use the underlying filesystem's read-only flag (or otherwise determine if it is read-only) to determine how to set the write-protect flag in the FDS registers.



\--[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 10:31, 4 November 2013 (MST) 

## Disk Status Register ($4030) behavior

I have built my [new dumper](https://github.com/ClusterM/famicom-dumper-writer). So I'm reverse-engineering FDS now. My dumper can read and write FDS cards already. But I noticed some inconsistencies with this page. In particular, I found out something about the $4030 (Disk Status) register. Bits 2 and 5 are set after any write to $4025 (FDS Control) register. Bit 2 is cleared when disk becomes ready ($4032.1 cleared) or after any transfer ($4024 or $4031 is serviced). Bit 2 is cleared only after any transfer ($4024 or $4031 is serviced). Also bit 7 is set every time $4031 is ready to be read or $4024 is ready to be written, e.g. it's messed up with bit 1 on this page. There is full trace of my work with FDS: 
    
    
     PRG(FDS_IRQ_CONTROL) = 0x00; // disable timer IRQ
     PRG(FDS_MASTER_IO) = 0x01; // enable disk registers
     // FDS_DISK_STATUS == 0x00 (0b00000000) here
     PRG(FDS_CONTROL) = 0b00001110; // reset
     // FDS_DISK_STATUS == 0x24 (0b00100100) here
     PRG(FDS_CONTROL) = 0b00001101; // monor on, unreset
     // FDS_DISK_STATUS == 0x24 (0b00100100) here
     // waiting until drive is rewinded
     do
     {
     } while (PRG(FDS_DRIVE_STATUS) & 2);
     // FDS_DISK_STATUS == 0x20 (0b00100000) here, bit 2 is cleared!
     PRG(FDS_CONTROL) = 0b00001101; // monor on without transfer
     // FDS_DISK_STATUS == 0x24 (0b00100100) here, bit 2 is set again!
     delay_clock(500000); // delay ~500000 clock cycles before first block
     // FDS_DISK_STATUS == 0x24 (0b00100100) here
     PRG(FDS_CONTROL) = 0b11001101; // enable transfer and interrupt
     // FDS_DISK_STATUS == 0x24 (0b00100100) here
     // waiting for IRQ
     // IRQ fired, first byte of data arrived
     // FDS_DISK_STATUS == 0xA4 (0b10100100) here, bit 7 is set when IRQ fired
     output = PRG(FDS_DATA_READ); // reading $4031, IRQ acknowledge
     // FDS_DISK_STATUS == 0x00 (0b00000000) here, everything is cleared when IRQ was acknowledged
     // waiting for IRQ
     // IRQ fired, second byte of data arrived
     // FDS_DISK_STATUS == 0x80 (0b10000000) here, bit 7 is set when IRQ fired
     output = PRG(FDS_DATA_READ); // reading $4031, IRQ acknowledge
     // FDS_DISK_STATUS == 0x00 (0b00000000) here, bit 7 is cleared
     ...and so on
    

Waiting for IRQ can be replaced with waiting for bit 7 in FDS_DISK_STATUS. Anyway, seems like there are no games that test these bits. FDS BIOS using interrupts. [Cluster](https://www.nesdev.org/w/index.php?title=User:Cluster&action=edit&redlink=1 "User:Cluster \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Cluster&action=edit&redlink=1 "User talk:Cluster \(page does not exist\)")) 04:39, 2 December 2020 (MST) 

### Bit 3

I did some poking around in a custom FDS port of nesmon (wozmon for NES) after SCSR brought up $4030.d3 on Discord, and it appears that this bit is the nametable arrangement/mirroring bit last written to $4025.d3. This doesn't seem to be documented elsewhere. --[TakuikaNinja](User_TakuikaNinja.xhtml "User:TakuikaNinja") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:TakuikaNinja&action=edit&redlink=1 "User talk:TakuikaNinja \(page does not exist\)")) 00:53, 11 March 2025 (UTC) 

## Battery bit in register $4033

Battery bit in register $4033 is messed up too. Actually 1 = good, 0 = voltage is low. [fceux source code](https://github.com/TASVideos/fceux/blob/master/src/fds.cpp) as proof: 
    
    
       static DECLFR(FDSRead4033) {
           return 0x80; // battery
       }
    

[Cluster](https://www.nesdev.org/w/index.php?title=User:Cluster&action=edit&redlink=1 "User:Cluster \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Cluster&action=edit&redlink=1 "User talk:Cluster \(page does not exist\)")) 05:22, 2 December 2020 (MST) 

## FDS Control ($4025) register

Actually bit 1 controls motor, not bit 0. Bit 1: 1 = stop motor (and reset?), 0 = start motor. Seems like it messed up because FDS BIOS always sets bit 0 after bit 1 clear. I'm not sure what actually bit 0 does: disk reading works fine when bit 0 is not set. Disk writing works only with small blocks otherwise motor actually stops during writing and data is corrupted. Also if set bit 1 (turn off motor) while bit 0 is also set, motor will be stopped after ~1 second, not instantly. So bit 0 should always be set when bit 1 is cleared and cleared when bit 1 is set. Further investigation required. Bit 5 is always set by FDS BIOS but seems like it doesn't do anything. Both reading and writing works fine when it's not set. [Cluster](https://www.nesdev.org/w/index.php?title=User:Cluster&action=edit&redlink=1 "User:Cluster \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Cluster&action=edit&redlink=1 "User talk:Cluster \(page does not exist\)")) 09:18, 2 December 2020 (MST) It looks like interrupts are not working correctly when bit 5 is not set. [Cluster](https://www.nesdev.org/w/index.php?title=User:Cluster&action=edit&redlink=1 "User:Cluster \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Cluster&action=edit&redlink=1 "User talk:Cluster \(page does not exist\)")) 04:20, 6 December 2020 (MST) 

    I've run into some odd behaviour regarding these bits, and it matches your description. If the drive is set to write mode ($4025.d3 = 0), then setting $4025.d0 = 1 stops the motor without actually turning it off - $4025.d1 must be set to 1, then 0, to restart the motor in this scenario. This does not occur in read mode ($4025.d3 = 1) - $4025.d0 has no effect on the motor. Regarding bit 5: SCSR on Discord recently brought up how the "always set" $4025.d5 appears to control what bits appear in $4033. I tested this in a custom port of nesmon (NES wozmon) and I can confirm that $4025.d5 = 1 results in normal $4033 behaviour but $4025.d5 = 0 changes bits 1 and 2 of $4033 (presumably still affected by open-collector behaviour). The contents of said bits seem to be affected by $4025 writes but I don't see an obvious pattern yet. I'm not sure about the interrupt behaviour yet, either. --[TakuikaNinja](User_TakuikaNinja.xhtml "User:TakuikaNinja") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:TakuikaNinja&action=edit&redlink=1 "User talk:TakuikaNinja \(page does not exist\)")) 01:22, 11 March 2025 (UTC)

## FDS cycle time

>>A complete cycle through the entire disc takes about 7 seconds 

After some tests with the original licensed disk, I found that 7600 milliseconds is the fastest possible speed to read the disk correctly and 8700 milliseconds is the slowest possible speed. So seems like the optimal value is (7600 + 8700) / 2 ~= 8150 milliseconds. But I'm not sure about it. [Cluster](https://www.nesdev.org/w/index.php?title=User:Cluster&action=edit&redlink=1 "User:Cluster \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Cluster&action=edit&redlink=1 "User talk:Cluster \(page does not exist\)")) 15:59, 22 April 2021 (MDT) 

## Documentation updates

Articles covering the FDS here have unfortunately been neglected for some time now. As a result, much of the information is either outdated, unverified, or undocumented. The following lists are comprised of work which needs to be done to improve the quality of these pages as of writing. 

### [FDS](Family_Computer_Disk_System.xhtml "FDS")

Registers: 

  * Verify the behaviour of $4025, including the unknown bit 5.
  * Verify & document CRC behaviour. 
    * [Famicom Network System](Family_Computer_Network_System.xhtml "Famicom Network System") has its own CRC behavior documented by [User:Joe](https://www.nesdev.org/w/index.php?title=User:Joe&action=edit&redlink=1 "User:Joe \(page does not exist\)"); it may be similar or the same. FDS and FNS have a lot in common.  
[Ben Boldt](User_Ben_Boldt.xhtml "User:Ben Boldt") ([talk](User_talk_Ben_Boldt.xhtml "User talk:Ben Boldt")) 23:37, 22 April 2024 (UTC)
  * Investigate $4023.D7, which is set by the FDS BIOS on reset.
  * Verify the behaviour of the other registers.



Physical components: 

  * Document the Mitsumi Quick Disk in more detail. (e.g. disk drive compatibility)
  * Create a new schematic of the RAM adapter.
  * Update the [RP2C33 pinout](RP2C33_pinout.xhtml "RP2C33 pinout"). 
    * Also include [DAC schematics](https://forums.nesdev.org/viewtopic.php?t=25226) from forums? -[ Persune](User_Persune.xhtml "User:Persune") Apr 20 2024, 22:37 (PhST)
  * Document the physical protection against unofficial disks, and its workaround.
  * Document the write protection measures added to later units, and methods to bypass them. 
    * How many bytes can be written before the protection kicks in?



### [FDS audio](FDS_audio.xhtml "FDS audio")

(leave to [Persune](User_Persune.xhtml "User:Persune") et al.) 

### [FDS disk format](FDS_disk_format.xhtml "FDS disk format")

  * Investigate the unknown fields of the info block.
  * Document known copy protection/anti-tamper measures.
  * Measure/determine the true capacity of a disk side.



### [FDS file format(s)](FDS_file_format.xhtml "FDS file format")

  * Document the IPS patching method commonly used by emulators for disk saving.
  * Document the .qd file format.
  * Investigate the FDS related flags in the [TNES](TNES.xhtml "TNES") header.



### [FDS BIOS](FDS_BIOS.xhtml "FDS BIOS")

  * Create a new BIOS disassembly. (ideally for all 3 known revisions)
  * Improve the documentation of the power-on/reset & post-load state.
  * ~~List commercial/bootleg games which skip the license message check.~~
  * ~~Link to software for dumping the FDS BIOS.~~



### Misc.

  * Archive the Famicom Hacking Manuals as they contain hardware/software information from during the commercial lifespan of the Famicom/FDS.
  * Document the I2 Souseiki Fami/Fammy as it appears to use a custom disk dump format.
  * ~~Document the behaviour of the[Namco IPL Interface](Namco_IPL_Interface.xhtml "Namco IPL Interface").~~ Are there other known cases of IPLs/bootloaders/bootstrappers in FDS games?



Let's work together to preserve the FDS. If those Japanese Famicom modders from back then have taught us anything, it's that the FDS deserved better. --[TakuikaNinja](User_TakuikaNinja.xhtml "User:TakuikaNinja") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:TakuikaNinja&action=edit&redlink=1 "User talk:TakuikaNinja \(page does not exist\)")) 08:12, 20 April 2024 (UTC) 
