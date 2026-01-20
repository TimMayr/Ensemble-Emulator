# Serial Bootloader

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Serial_Bootloader) | View [other pages](Special_AllPages.xhtml#Serial_Bootloader)

This page specifies the protocol and behavior of a NES/Famicom bootloader that receives a 256-byte program via a [ serial cable](Serial_Cable_Construction.xhtml "Serial Cable Construction") connected to a PC. A fully-conforming example implementation that can be tried out is available: [bootloader-basic-2.zip](http://blargg.8bitalley.com/nes/bootloader-basic-2.zip). A robust, flexible library implementation and size-optimized variations is also available: [bootloader-lib-0.2.0.zip](http://blargg.8bitalley.com/nes/bootloader-lib-0.2.0.zip). 

## Contents

  * 1 Program block format
  * 2 Bootloader operation
  * 3 Execution environment
  * 4 Design rationale
  * 5 Change log



# Program block format

A program sent to the bootloader is contained in a single 256-byte program block, consisting of a signature, checksum, and code/data. 

Offset | Size | Content   
---|---|---  
+0 | 3 | Signature: `$DC $4B $D2`  
+3 | 1 | CRC (see below for calculation)   
+4 | 252 | User code/data, copied to $04-$FF in zero-page   
  
Send program block to NES via serial cable at 57600 bits per second, 8 data bits, no parity, and 1 or 2 stop bits. If there might be other data sent before the block, send at least one $FF byte just before program block data. Hardware wiring is covered in [Serial Cable Specification](Serial_Cable_Specification.xhtml "Serial Cable Specification"). 

# Bootloader operation

The bootloader waits for the signature, receives the checksum and data, verifies the checksum, then executes the program. 

Action | Bytes | Details   
---|---|---  
Wait for $DC | n | This skips over any junk data before block   
Check for $4B $D2 (optional) | 2 | If mismatch, go back to first step. This ensures it's a program block rather than a stray $DC. Avoid storing 3-byte signature in code, otherwise bootloader itself might be mistaken for program block when embedded in something being uploaded where the real header at the beginning was missed for some reason.   
Receive checksum | 1 | Either save, ignore, or calculate checksum as data is received   
Receive user data | 252 | Write to zero-page at $04-$FF   
Verify checksum (optional) |  | If checksum is wrong, go back to first step. To verify checksum, clear 8-bit CRC to 0. Then for each of the 256 bytes in order: XOR byte into CRC, shift left 1 bit (with high bit going into carry), and add $99 and carry bit. CRC should equal 0 at end. Code can take up to 8700 cycles to verify CRC (33 per byte).   
A = 0 if NTSC |  | If on NTSC system, clear A, otherwise set to any non-zero value   
Begin running program at $0004 |  | No other register or memory initialization is necessary before running program. In particular, stack pointer and first 4 bytes of zero-page may be left uninitialized.   
  
# Execution environment

A received program begins executing at $04 in zero-page, with nothing initialized except its code/data loaded into $04-$FF, and register A. 

PC | $0004   
---|---  
A | zero if NTSC, non-zero if PAL   
X, Y, P, S | Uninitialized   
$00-$03 | Uninitialized   
$04-$FF | User code/data   
$100-$7FF | Uninitialized   
  
Execution begins no more than 8700 cycles after last byte of program block is received. 

# Design rationale

The design for the bootloader was come to by a fairly exhaustive study of the various forces at work and what they naturally converge to when combined. 

**At least 256 bytes:** A loader needs to receive a good number of bytes, or else the received program won't be able to do anything useful. It could receive a small number, say 100, but there's no reason not to receive 256 bytes, given 6502 indexing. 

**No more than 256 bytes:** Beyond 256 bytes, more code is needed on the 6502. This adds quite a few bytes to the loader code. It also leads to the inevitable desire for a custom address and size and support for multiple blocks of data to different regions of memory. 

**Load into zero-page:** The received program is likely to be a loader itself, albeit more capable than the bootloader. This means that it shouldn't be in a region of memory where further code is loaded. Since most programs use zero-page for variables rather than code, that is an obvious choice. This also allows the received program to be self-modifying and use the more compact zero-page addressing to do so. Finally, the received program can be the first 256 bytes of a larger 512-byte program at 0-$1FF, with the first half receiving the rest of itself beginning at $100. 

The stack is a possible place to load program, since a PHA can be used to write each byte. But this makes it difficult to do if calling a serial receive routine, as it will overwrite some of the bytes on the stack. It doesn't even reduce code size, since the stack pointer has to be initialized. It also makes the code less clear. 

**Checksum of data:** The loader must be able to verify that it received the program block without error, so that it doesn't execute corrupt data and produce unpredictable results. Even if the received program tried to checksum itself, the checksum code itself relies on not being corrupt because it otherwise might believe the checksum is correct even when it isn't. 

**Signature at beginning:** A signature at the beginning allows a loader to ignore any other data it might receive before the program block. The checksum might seem able to handle this, and while it would prevent running the mal-formed block, it would result in the program block being ignored if it had even one extra byte before the beginning. With a signature, the loader can wait until it finds the signature, then receive the rest of the program block, and be able to handle junk data before it without ignoring the program block itself. 

**Multi-byte signature:** The signature must consist of multiple bytes, not just a single one. This greatly reduces the possibility of random data containing the signature. A two-byte signature is still somewhat likely to occur, while a three-byte signature is extremely unlikely. The particular values for the signature have been chosen after scanning lots of NES code and data for the sequences least likely to occur. The subsets of the signature occur the indicated number of times in a scan of a few hundred megabytes of NES code/data: DC4B:106 times, 4BD2:129 times, DC4BD2:0 times. The least-likely two-byte sequence, D45B, occurred 105 times, but any combinations using it had a higher incidence of the other two-byte portions. The 106 and 129 counts of the byte pairs used are near the minimums. 

**8-bit CRC:** PC communication with the NES has been very reliable, so a simple 8-bit CRC can be used. The chosen CRC has performance on par with more common LFSR-based ones, but is easier to implement with very little code. Technically it's a _non_ -linear LFSR, since it uses ADC. It detects all one-bit errors, and only misses about one in 200 two-bit errors. Given that even one-bit errors are unlikely, the likelihood of an uncaught multi-bit error is insignificant. The CRC is still useful to catch implementation problems or data corruption by the PC before sending. Several variations on simple NLFSRs were tried, with $99 being the best value to add (a close second was $0C, with slightly better error detection if the data is all zeroes, but otherwise slightly worse than $99). 

**Signature and checksum as part of 256 bytes, rather than separate:** Fundamentally, a loader must keep track of how many bytes it's received of the program block. If the block is larger than 256 bytes, it must use more than 8 bits to keep track of the position. Handling more than 8 bits requires more code, and prevents keeping the state in a single register. The signature and checksum are thus put into the 256-byte block, rather than added before and after it. This allows a minimal loader to receive exactly 256 bytes and then begin executing, without leaving any unread or having to skip it. 

**Program begins at $04 in zero-page:** Since the signature and checksum are part of the 256-byte program block, the user code size is reduced below 256 bytes. If user code began at address 0, it wouldn't go all the way to the end of zero-page. So we load the user code at $04 in zero-page, so that it covers all bytes through the end of zero-page. User code can easily receive more code at $100 and have it connect seamlessly. It can then use $00-$03 for variables. A minimal loader can easily achieve this load address by writing the first byte of the program block to $00, so that the program data at offset 4 gets written to $04 in zero-page. 

**Signature and checksum at beginning:** By putting the extra data at the beginning of the block, the user code/data is at the same offset in the block as it will be in zero-page. This simplifies thinking about loading, removing an unnecessary complication in implementation. It also eliminates the possibility that a bootloader could start executing the program before it's all been received, since the last byte of the block is part of the program rather than the checksum as before. This change didn't increase the size of any of the loaders, though it did require figuring out how to do a CRC calculation from end to beginning in order to calculate the value that would cancel out a normal beginning-to-end calculation to zero. 

**Extra time to calculate checksum:** The loader is given extra time to calculate the checksum after receiving the data. I's possible for it to calculate the checksum as it's receiving a serial byte, but it's more involved. While this additional time means that the received code must re-synchronize with serial, it must do that _anyway_ , since it already takes enough time to begin executing it that at least one serial byte will be partially lost. Synchronization is trivial: insert several $FF bytes after 256-byte program block, before following data, and then read until a non-$FF byte is received. No matter where the serial code begins waiting for bytes, it will always either miss the current $FF sync byte, or receive it as $FF, never any other value. 

**Uninitialized registers on program entry:** Not specifying initial register values on entry to the received program means that it can't assume they are cleared. If it needed A, X, and Y clear, it would need four bytes of code to do so. In most causes it might need one cleared, which adds two bytes. Also, many programmers will still clear things at the beginning anyway, just to be more robust. 

# Change log

  * Added C code to build a program block in memory.
  * Added a few more design rationale.
  * Simplified program block layout, with checksums at beginning. Sorry for breaking it, but this had to be done.
  * 2010-11-14 Redid everything, in light of feedback and discussion, the need to handle multiple inputs for serial data, and the lack of any real uses bt anyone else. Now has simpler 3-byte signature, 1-byte CRC, and 252 bytes of user data (no more 16-bit CRC, which was way more than needed). Data is transmitted without any bit reversal or complementing.


