# User:Zzo38/Mapper 768

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AZzo38/Mapper_768) | View [other pages](Special_AllPages.xhtml#User_Zzo38_Mapper_768)

I am reserving mapper 768 for my own use. This is not currently an official specification, nor is it currently supported by any emulators at the time of this writing, which is why it is not currently in the main namespace. 

In all cases, the NES 2.0 file is arranged like (some parts may be omitted, depending on the header): 

  * Header
  * Trainer
  * PRG ROM
  * CHR ROM
  * INST-ROM
  * PROM
  * 128-byte null-terminated ASCII title
  * Extra data for mapper 768 (depending on submapper number)



Omitting parts of the file that are required (according to the header) is not allowed if mapper 768 is used. There is some discussion of extending the format to specify that extra data exists, but this is currently unknown. 

Use of [UNIF](UNIF.xhtml "UNIF") is supported by this specification, but is not recommended; it is meant to be used with [NES 2.0](NES_2_0.xhtml "NES 2.0"). 

## Contents

  * 1 Submapper 0
    * 1.1 Commands
    * 1.2 NSF Interface
    * 1.3 Examples
      * 1.3.1 Simple bankswitching
      * 1.3.2 Mapper 218
      * 1.3.3 Double bus CHR RAM
  * 2 Submapper 1



# Submapper 0

The UNIF name for this mapper is "`X-Custom`", and has the "`XTRA`" block for extra data. 

The extra data stores the code for a virtual machine, to control the mapper. The commands are sixteen bits in small endian format, and are split into subroutine blocks. At the beginning of each block is one byte to tells how many sixteen-bit commands are in this subroutine block (this includes immediate values too). 

The first block is always the initialization block. 

Anything other than what is written in the codes acts like NROM (iNES mapper 0) (only the first 16K or 32K of PRG ROM is accessible, though). 

There is sixteen registers, with sixteen bits each. R0 and R1 are local to a subroutine call, and the R2 to R15 are global registers (all general purpose). There is also two more local registers, called W (working register) and W' (shadow working register). 

Subroutine numbers are eight bits long. Each subroutine takes two parameters which is the initial assignment of R0 and R1 registers, and returns a sixteen bit number. 

## Commands

  * `0000 0000 xxxx yyyy` (XCAL) = Call a subroutine indicated in the low eight bits of Rx, with this call's R0 and R1 as its parameters, and store the result in Ry (leave Ry alone if the result is open bus or CIRAM).
  * `0000 0001 0000 xxxx` (COPW) = Copy W to Rx.
  * `0000 0001 0001 xxxx` (COPR) = Copy Rx to W.
  * `0000 0001 0010 xxxx` (ADDW) = Add W to Rx.
  * `0000 0001 0011 xxxx` (ADDR) = Add Rx to W.
  * `0000 0001 0100 xxxx` (SUBW) = Subtract W from Rx.
  * `0000 0001 0101 xxxx` (SUBR) = Subtract Rx from W.
  * `0000 0001 0110 xxxx` (ANDW) = Bitwise AND W to Rx.
  * `0000 0001 0111 xxxx` (ANDR) = Bitwise AND Rx to W.
  * `0000 0001 1000 xxxx` (IORW) = Bitwise OR W to Rx.
  * `0000 0001 1001 xxxx` (IORR) = Bitwise OR Rx to W.
  * `0000 0001 1010 xxxx` (XORW) = Bitwise XOR W to Rx.
  * `0000 0001 1011 xxxx` (XORR) = Bitwise XOR Rx to W.
  * `0000 0001 1100 xxxx` (NANW) = Bitwise NAND W to Rx.
  * `0000 0001 1101 xxxx` (NANR) = Bitwise NAND Rx to W.
  * `0000 0001 1110 xxxx` (NOTW) = Copy NOT W to Rx.
  * `0000 0001 1111 xxxx` (NOTR) = Copy NOT Rx to W.
  * `0000 0010 0000 xxxx` (RETR) = Return the value in Rx from this subroutine.
  * `0000 0010 0001 0000` (OPEN) = Return open bus from this subroutine.
  * `0000 0010 0001 0001` (RETW) = Return the value in W from this subroutine.
  * `0000 0010 0001 0010` (CIR0) = Return CIRAM bank 0 from this subroutine. (Returning CIRAM banks is the same as open bus unless the PPU is accessing this memory.)
  * `0000 0010 0001 0011` (CIR1) = Return CIRAM bank 1 from this subroutine.
  * `0000 0010 0001 0110` (CIRC) = Return CIRAM bank 0 if condition flag is set, or bank 1 if condition flag is clear.
  * `0000 0010 0001 0111` (CIRS) = Return CIRAM bank 1 if condition flag is set, or bank 0 if condition flag is clear.
  * `0000 0010 0001 1xxx` (RETL) = Return the literal value x (all other bits zero) from this subroutine.
  * `0000 0011 0000 0000` (SWAP) = Swap W with W'.
  * `0000 0011 0001 xxxx` (IMMR) = Store an immediate value into Rx. The condition flag is set to if the value changed.
  * `0000 0011 0010 xxxx` (ASLR) = Shift left Rx. The shifted out bit is stored in the condition flag.
  * `0000 0011 0011 xxxx` (ADDI) = Add an immediate value to Rx. The condition flag is set to if there is the signed overflow.
  * `0000 0011 0100 xxxx` (ASRR) = Arithmetic shift right Rx. The shifted out bit is stored in the condition flag.
  * `0000 0011 0110 xxxx` (RORR) = Rotate right Rx.
  * `0000 0011 0111 xxxx` (ANDI) = Bitwise AND an immediate value to Rx. The condition flag is set to if the value changed.
  * `0000 0011 1001 xxxx` (IORI) = Bitwise OR an immediate value to Rx. The condition flag is set toif the value changed.
  * `0000 0011 1010 xxxx` (LSRR) = Logical shift right Rx. The shifted out bit is stored in the condition flag.
  * `0000 0011 1011 xxxx` (XORI) = Bitwise XOR an immediate value to Rx. The condition flag is set to if the value changed.
  * `0000 0011 1100 xxxx` (INVR) = Bitwise invert Rx.
  * `0000 0011 1101 xxxx` (NANI) = Bitwise NAND an immediate value to Rx. The condition flag is set if and only if the value changed.
  * `0000 0011 1110 xxxx` (ZERO) = Set the condition flag if Rx is zero, or clear the condition flag otherwise.
  * `0000 0011 1111 xxxx` (EQUA) = Set the condition flag if the immediate value is equal to Rx, or clear the condition flag otherwise.
  * `0000 0100 000x xxxx` (SKIT) = Skip x+1 instructions (counting immediate values as instructions) if the condition flag is set.
  * `0000 0100 001x xxxx` (SKIF) = Skip x+1 instructions (counting immediate values as instructions) if the condition flag is clear.
  * `0000 0100 0100 0000` (TIRQ) = Trigger IRQ.
  * `0000 0100 0101 0000` (WPR1) = Force battery RAM to be write-protected.
  * `0000 0100 0101 0001` (WPR0) = Turn off the forced write-protection of battery RAM.
  * `0000 1000 xxxx yyyy` (COPY) = Copy Rx to Ry.
  * `0000 1010 xxxx yyyy` (COPT) = Copy Rx to Ry if condition flag is set.
  * `0000 1011 xxxx yyyy` (COPF) = Copy Rx to Ry if condition flag is clear.
  * `0001 0000 xxxx xxxx` (RPRG) = Using W as a start address, W' as a end address, and R15 as a mask (any set bits in the mask correspond to address bits that must match that of the start address), set the subroutine calls for reading PRG memory to the subroutine x.
  * `0001 0001 xxxx xxxx` (WPRG) = Set the subroutine calls for writing PRG memory to the subroutine x.
  * `0001 0010 xxxx xxxx` (RCHR) = Set the subroutine calls for reading CHR memory to the subroutine x.
  * `0001 0011 xxxx xxxx` (WCHR) = Set the subroutine calls for writing CHR memory to the subroutine x.
  * `0001 0100 xxxx xxxx` (ACHR) = Set the subroutine calls for what happens when the PPU address bus changes without a read or write.
  * `0001 1000 xxxx xxxx` (RSET) = Set the subroutine calls for reset to the subroutine x.
  * `0001 1001 xxxx xxxx` (DISK) = Set the subroutine calls for FDS IRQ to the subroutine x.
  * `0100 xxxx yyyy yyyy` (CTIM) = Set a timer for Rx CPU clocks to call subroutine y (just once, when the timer expires). If Rx is -1 then it is turned off.
  * `0101 xxxx yyyy yyyy` (PTIM) = Set a timer for Rx PPU reads to call subroutine y (just once, when the timer expires). If Rx is -1 then it is turned off.
  * `1000 xxxx yyyy yyyy` (CALL) = Call subroutine y with this subroutine's W and W' as parameters, and store the result in Rx (leave Rx alone if the result is open bus or CIRAM).
  * `1001 0000 xxxx yyyy` (RNSF) = Read from the NSF interface, address in Rx, into Ry.
  * `1001 0001 xxxx yyyy` (WNSF) = Write to the NSF interface, address Rx, value Ry.
  * `1001 0010 xxxx yyyy` (ROPR) = Read from the PRG ROM, address Rx, into Ry.
  * `1001 0100 xxxx yyyy` (ROCH) = Read from the CHR ROM, address Rx, into Ry.
  * `1001 0110 xxxx yyyy` (ROTR) = Read from the trainer ROM, address Rx, into Ry.
  * `1001 1000 xxxx yyyy` (RNPR) = Read from non-battery PRG RAM, address in Rx, into Ry.
  * `1001 1001 xxxx yyyy` (WNPR) = Write to non-battery PRG RAM, address Rx, value Ry.
  * `1001 1010 xxxx yyyy` (RNCH) = Read from non-battery CHR RAM, address Rx, into Ry.
  * `1001 1011 xxxx yyyy` (WNCH) = Write to non-battery CHR RAM, address Rx, value Ry.
  * `1001 1100 xxxx yyyy` (RBPR) = Read from battery PRG RAM, address Rx, into Ry.
  * `1001 1101 xxxx yyyy` (WBPR) = Write to battery PRG RAM, address Rx, value Ry.
  * `1001 1110 xxxx yyyy` (RBCH) = Read from battery CHR RAM, address Rx, into Ry.
  * `1001 1111 xxxx yyyy` (WBCH) = Write to battery CHR RAM, address Rx, value Ry.



When a subroutine call is triggered due to CPU or PPU read/write, R0 will be the address, and R1 will be the data for writing (R1 is undefined if reading). Returning CIRAM bank 0 or CIRAM bank 1 from PPU read routines causes it to read the CIRAM instead of the cartridge. Returning CIRAM banks from PPU write routines causes it to write to the CIRAM of the specified bank (the routine can do other things too). Returning open bus or any number from any write routine, or any CIRAM bank from any PRG write routine, does nothing. The default return value (if a block ends without an explicit return) is open bus. If a read routine is called for addresses below $4020, it must return open bus or CIRAM banks, otherwise the result is undefined. 

## NSF Interface

  * $0000-$00FF: General purpose RAM (stored in the cartridge; not the same as the CPU RAM, or the cartridge PRG RAM and CHR RAM mentioned in the header; this is only accessible by the NSF interface).
  * $2000,$2001 (CART_CPU_COUNT): Sixteen bit CPU clock counter; can be written as well.
  * $2002,$2003 (CART_PPU_COUNT): Sixteen bit PPU read counter; can be written as well.
  * $2005 (CART_BUS_CONFLICT): Write the number of bus conflicts that happen at one time into here. Writing zero does nothing. This might be used by an emulator which counts bus conflicts in order to detect errors in your program.
  * $2006 (CART_PRG_ROM_BANK): PRG ROM bank select.
  * $2007 (CART_CHR_ROM_BANK): CHR ROM bank select.
  * $2008 (CART_PROTECT): Clear the low bit to force battery RAM to be write-protected.
  * $2009 (CART_MUTE): Write-only. If bit0 is set, 2A03 audio is muted. If bit1 is set, MMC5 PCM is muted.
  * $200A (CART_ADC) : Read-only. An 8-bit analog to digital converter reading the microphone audio mixed with the 2A03 audio.
  * $200C (CART_PRG_RAM_BANK): Non-battery PRG RAM bank select.
  * $200D (CART_PRG_BAT_BANK): Battery PRG RAM bank select.
  * $200E (CART_CHR_RAM_BANK): Non-battery CHR RAM bank select.
  * $200F (CART_CHR_BAT_BANK): Battery CHR RAM bank select.
  * $4020-$4092: FDS registers. All registers (including audio) are available, although bit3 of $4025 has no effect if PPU read/writes are not acting in NROM mode (i.e. if they have been overridden by the mapper).
  * $4800,$F800: Namco 163 audio.
  * $5000-$5015: MMC5 audio. (The read mode acts on CPU memory, not NSF memory.)
  * $5205,$5206: MMC5 hardware multiplication register.
  * $9000-$9003,$A000-$A002,$B000-$B002: VRC6 audio.
  * $9010,$9030: VRC7 audio.
  * $C000,$E000: Sunsoft 5B audio.



Bank select registers are always eight high bits of whatever is available (for example if you have 64K ROM and 32K bank size, you should set only the high bit of the PRG ROM bank register). 

## Examples

### Simple bankswitching
    
    
    ; Simple bankswitching mapper; write to $8000-$FFFF, low bit of address selects bank.
    @INIT
    IMMR F FFFF
    COPR F
    SWAP
    IMMR F 8000
    COPR F
    ASLR F
    RPRG @READPRG
    WPRG @BANKSET
    @READPRG
    ANDI 0 7FFF
    COPR F
    IORW 0
    ROPR 0 1
    RETR 1
    @BANKSET
    7*ASLR 0
    COPY 0 F
    

### Mapper 218
    
    
    ; Single-chip mapper
    @INIT
    IMMR F FFFF
    COPR F
    SWAP
    INVR F
    COPR F
    RCHR @PPU_ACCESS
    WCHR @PPU_ACCESS
    @PPU_ACCESS
    IORI 0 2000 ; Change this line for a different CIRAM A10 connection.
    CIRC
    

### Double bus CHR RAM
    
    
    ; Write into low four bits of $5000-$5FFF to activate/deactivate CIRAM for PRG memory
    ; Write into $1000-$1FFF to copy the values written into the CHR RAM in the cartridge
    @INIT
    IMMR F 0000
    IMMR E 5FFF
    COPR E
    SWAP
    IMMR E 5000
    COPR E
    WPRG @PPU_SET
    IMMR E 3FFF
    COPR E
    SWAP
    IMMR E 0000
    COPR E
    RCHR @CHR_ACCESS
    WCHR @CHR_ACCESS
    IMMR E 1FFF
    COPR E
    SWAP
    IMMR E 1000
    COPR E
    WPRG @CHR_WRITE
    IMMR 8 0000
    IMMR 9 0040
    IMMR A 0080
    IMMR B 00C0
    IMMR C FFFF
    @PPU_SET
    COPY 8 4
    COPY 8 5
    COPT 8 6
    COPY 8 7
    COPY 1 0
    IORI 0 0001
    COPT C 4
    IORI 0 0002
    COPT C 5
    IORI 0 0004
    COPT C 6
    IORI 0 0008
    COPT C 7
    WNSF 8 4
    WNSF 9 5
    WNSF A 6
    WNSF B 7
    @CHR_ACCESS
    COPY 0 1
    ANDI 1 3000
    6*LSRR 1
    ;***TODO***
    @CHR_WRITE
    ANDI 0 0FFF
    WNCH 0 1
    

# Submapper 1

The UNIF name for this mapper is "`X-Verilog`". 

Extra data is not used. There is expected to be a file with `.nes.v` extension (otherwise having the same name), which contains a Verilog code for implementing the mapper. Some emulators might require that the Verilog code is compiled first into another file (possibly a DLL or shared object file); if so, such thing should be explained in the documentation for that particular emulator. 

The first sixty I/O ports of the `main` module of the Verilog code must correspond to the pins 01 to 60 of the 60-pin Famicom cartridge, in that order. This is followed by the pins for the PRG ROM, CHR ROM, non-battery PRG RAM, non-battery CHR RAM, battery PRG RAM, and battery CHR RAM. 

The ROM/RAM pins are only for the ROM/RAM which are existing, and is as follows: 

  * Chip enable (low to enable)
  * Write enable (low to enable; not exist for ROM)
  * Address pins (the exact number of pins needed for the ROM/RAM of the size specified in the NES 2.0 header)
  * Data pins (always eight)



The commands with `$` at front might not be implemented, but should be safely ignored if not implemented. However, if there is a trainer ROM, there will be an additional command `$trainer` to access 8-bit numbers given the 9-bit address, and `$battery_init` which tell you if you need to initialize the battery RAM. 

Analog commands may be used with the audio signals. 
