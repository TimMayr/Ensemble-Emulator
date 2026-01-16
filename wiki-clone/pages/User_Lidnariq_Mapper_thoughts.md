# User:Lidnariq/Mapper thoughts

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3ALidnariq/Mapper_thoughts) | View [other pages](Special_AllPages.xhtml#User_Lidnariq_Mapper_thoughts)

## Contents

  * 1 EtherNES 1
    * 1.1 Option 1
      * 1.1.1 Parts
      * 1.1.2 Memory map
    * 1.2 Option 2
  * 2 Easier nametables, finer palette zones
  * 3 Pixel-perfect IRQs with clockslides
  * 4 Fake sprite 0 IRQs
  * 5 Very fine-grained CHR banking
  * 6 8×8 attributes
    * 6.1 8x16 attributes, compatible with all famiclones
  * 7 Hardware vertical parallax
  * 8 MMC3 minimal cost bankable CHR RAM with 4-screen nametables
  * 9 Oversize PRG for some ASIC mappers
    * 9.1 MMC4
    * 9.2 Sunsoft 3 and Sunsoft 4
    * 9.3 VRC3
    * 9.4 LZ93D50
    * 9.5 TAM-S1
    * 9.6 FCG-1 and -2



## EtherNES 1

A cartridge designed to provide ethernet in as simple a manner as practical, for use with [Contiki](http://www.contiki-os.org/). 

### Option 1

The design is intentionally similar to the [64NIC+ ethernet cartridge for the Commodore 64](http://www.go4retro.com/products/64nic/), even though the CS8900A is comparatively expensive. 

#### Parts

  * 1 × SST39SF010A, 128 KiB FLASH EEPROM, for booting
  * 2 × AS6C1008, 128 KiB static RAM, one each for PRG RAM and CHR RAM
  * 2 × 74'161, both set up to clear on reset. 
    * One selects a 32 KiB slice of PRG
    * The other selects an 8 KiB slice of CHR RAM.
  * 1 × 74'139
  * 1 × 74'20 
    * One NAND4 produces /INTSEL=NAND4(M2,/ROMSEL,A14,A13) and one decoder produces /RD6xxx, /WR6xxx, /RD7xxx, and /WR7xxx.
    * Other NAND4 produces /RD.
    * 2nd decoder selects RAM or ROM as appropriate.
  * CS8900A or LAN91C96 already have drivers in the Contiki source tree 
    * Other cheaper options: RTL8029, CP2200, ENC424J600, AX88796
  * Ethernet transformer



#### Memory map

  * $7xxx - CS8900A 
    * <http://www.cirrus.com/en/products/cs8900a.html>
    * Unlike the C64 NIC, use the memory-mappable portion of the CS8900A.
    * CS8900A does not have a separate enable, just /{MEM,IO}{R,W}; the address bus must be stable before these signals fall.
    * Why does the 64NIC+ not connect /IRQ ?
    * Obviously no DMA controller.
  * READS from $6xxx - latching CPU A0..A3: PRG bank '161 (0,1,2,3 = ROM; C,D,E,F=RAM). Q3 determines which IC. Cleared on hardware reset.
  * WRITES to $6xxx - latching CPU D0..D3: CHR bank '161
  * $8000-$FFFF: PRG ROM or RAM, according to value at $7xxx.



### Option 2

The ENCx24J600 is a still-actively manufactured parallel I/O network IC made by Microchip. Fortunately, it can support the 6502's R/W interface, but unfortunately it uses +ve sense signalling for all other signals. 

  1. Possibility: This IC supports a 15-bit address memory-mapped form; allowing mapping the NIC into $8000-$FFFF. 
     * Problem: the 6502 vectors now point at garbage. Ask Microchip what reading from those addresses do? If not acceptable, invert A14?
     * Another problem: The driver has to either fit in the NIC's RAM, or do everything via trampolines
     * Nifty feature: stream DPCM over ethernet. (Packets will have to be realigned, but this IC has an internal DMA feature that could do copies accordingly)
  2. Possibility: Re-use the PRG or CHR bank to select a 4 KiB slice to map. 
     * Weird feature: stream OAM over ethernet (why?)
  3. Possibility: Only allow I/O to the the 512 bytes starting at $7Exx; all access to the internal memory is via the indirect registers. 
     * If the NIC's A9..A11 are tied high, a latch and an OR gate lets us hijack OAM DMA (which writes to $2004) to write data to the NIC (pointer writes via 0x7E84) ... but is it useful to be able to transfer data to the NIC rapidly?
  4. Possibility: Use multiplexed bus, latching the 8 LSBs while directly driving the 7 MSBs. Need a 4th write trigger (ethernet ALE, ethernet CE, CHR bank, PRG bank) or to combine PRG/CHR banking register.
  5. Simplest possible option #1: NOT self-writable NROM with a 74'21 (quad AND) to map the NIC from $6000-$7FFF. Slightly clever rewiring (CPU A12→NIC A14,CPU A11→NIC A13,12,11) allows 6 KiB of the 24 KiB of RAM inside the NIC (0x0000-0x07FF, 0x3800-0x47FF) to be directly available to the NES, while ensuring that the NIC registers (0x7E00-0x7FFF) are always available.
  6. Option #1a: 74'20 and 74'04, allows self-writable NROM, and increasing available RAM to 12 KiB. Maps NIC 0x0XXX to $7XXX, 0x3XXX to $6XXX, 0x4XXX to $5XXX, and NIC registers at $4E00-$4FFF.
  7. Simplest possible option #2: use NIC RAM in lieu of CHR-RAM. Addressing NIC becomes a total PITA because of the PPU's auto-increment. But streaming graphic data becomes very easy.



Unfortunately, the shape of the front-loader means that the ethernet jack itself will have to be mounted both rotated 90°, and at the very top of the cartridge (22mm clearance, 16mm wide jacks) 

## Easier nametables, finer palette zones

Replace standard PPU nametable/attribute table with just one nametable, 32x30 tiles in size, each tile is 16x16 and uses 8x1 palette zones. 

  * using 32×30 allows us to reuse the scroll registers, rather than having fine X be magically somewhere else
  * Each tile is thus 72 bytes (256 pixels × (18 bits/8 pixels)).
  * Total memory for tiles in this manner: 18 KiB.



## Pixel-perfect IRQs with clockslides

An interface to fire an IRQ at a specific X/Y coordinate that works by 

  * firing the IRQ approximately 14cy too early
  * uses an injected clockslide to fix up any slop in initial IRQ firing time



Since there's already 3 pixels of intrinsic slop, the two axes can be X/2 and Y. Y is ever-so-slightly larger than a byte, so setting (255=scanline before NMI) and not allowing interrupts during the first 6 scanlines of vblank is an ok compromise. 

## Fake sprite 0 IRQs

  1. IRQ when PPU address reads from a specific address, or
  2. Snoop on CPU reads, monitoring for reads from $2002, and fire an IRQ



## Very fine-grained CHR banking

64 banks of 64 bytes (4 tiles) each, allowing changing the bank a sprite comes from mid-render. Not clear if useful for anything but masking sprites. 

Could be combined with some amount of address-line math to prevent sprite pop-on on PAL systems, and with a barrel shifter to prevent sprite pop-on on the left side of the screen without using the PPU's native masking. 

## 8×8 attributes

Assuming a mapper with 4 banks of 8 KiB each of CHR-RAM. 

On nametable reads: latch y and x in address: 1d YXYY YYyX XXXx 

Detect attribute reads: 1d YX11 11YY YXXX 

On attribute reads, disable NTRAM and select one of four banks depending on y and x. 

  * /AttrIO = NAND4(PPU A9..A6)
  * CIRAM/OE = NAND2(PPU A13,/AttrIO)
  * NTread = '139 (/RD=0, PPU A13=1, /AttrIO=1) 
    * NTread: latch PPU A0, A5 in a 74'75 or whatever
  * ATTRread = '139 (/RD=0, PPU A13=1, /AttrIO=0)
  * CHRRAM/OE = PPU A13 AND /AttrIO
  * CHRRAM A13,A14 = multiplex(ATTRread,BankedA13..A14,LatchedA0A5)
  * Separately, use other half of NAND4 and/or 74'139 and 74'75 to latch requested CHR bank from CPU



Total parts: ½ 74'20, ½ 74'139, 74'00, 74'75, 74'153 

With both 4KiB and 8KiB banking, all four nametables will have different attributes, ignoring what CIRAM A10 is connected to. If used with a mapper that supported 1 or 2 KiB banking, it would allow something with more finesse. 

### 8x16 attributes, compatible with all famiclones

Compatible with all famiclones because CIRAM/CE is still tied to ground. 

Using three 74xx ICs: 

  * 74'21: AttrIO = AND4(PPU.A9..A6)
  * 74'161: 74161./PE = AttrIO ; 74161.CP = PPU./RD ; 74161.D0 = PPU.A0 ; LA0 = 74161.Q0
  * 74'153: CIRAMA10 = AttrIO?LA0:PPUA11



Using PPU A5 instead of A0 allows for 16x8 attribute zones. 

Latching both A0 and A5, changing AttrIO to include PPU A13, and connecting to CHR ROM address lines allows for 1024 different tiles per nametable. 

  


## Hardware vertical parallax

Putting an N-bit adder between PPU and CHR A0…A2,A4…A11 would allow for tiles to be scrolled independently. Inverting the result of that adder would flip that set of tiles vertically. 

In combination with a mapper that provided 1 KiB banks, and depending on how many address lines passed through the adder vs just directly connected to CHR, it would allow 1,2,4,8,16,32, or 64 independent 8-pixel-wide scrolling regions that were 512,256,128,64,32,16, or 8 pixels high, respectively. 

## MMC3 minimal cost bankable CHR RAM with 4-screen nametables

Connect MMC3 PPU A10 IN instead to PPU A13. Disconnect MMC3 CHR A10 OUT. Connect PPU A10 directly to CHR RAM A10. Disable NES-internal NTRAM, permanently enable cart RAM. User should always write to $8000 with "CHR A12 inversion" bit set. Resulting memory map is: 

bank name | CHR A12 inversion | without   
---|---|---  
R0 | 2KB bank at $1000 | both $0000 and $2000   
R1 | 2KB bank at $1800 | both $0800 and $2800   
R2 | 2KB bank at $0000 | $1000   
R3 | 2KB bank at $2000 | $3000, not useful   
R4 | 2KB bank at $0800 | $1800   
R5 | 2KB bank at $2800 | $3800, not useful   
  
LSbit of CHR/NT banks written to $8001 are ignored. Vertical mirroring and 4-screen layout are possible, from any location in CHR RAM. The original mirroring control register is ignored. 

This is similar to [mapper 76](INES_Mapper_076.xhtml "INES Mapper 076"), but doesn't break the MMC3's IRQ. 

## Oversize PRG for some ASIC mappers

Almost all ASIC mappers that support 16+16F banking can trivially be extended to a full 8 bits of PRG address space. 

For example: 

### [MMC4](MMC4.xhtml "MMC4")

74'138, 4+ bit latch, 74'32. 74'138 decodes writes to $Axxx. Latch latches D4 through D7 during those writes. 74'32 implements fixed upper bank. 

### [Sunsoft 3](INES_Mapper_067.xhtml "INES Mapper 067") and [Sunsoft 4](INES_Mapper_068.xhtml "INES Mapper 068")

Same hardware. 74'138 decodes writes to $F8xx or $Fxxx, respectively. Otherwise identical. 

Both ASICs already latch D4 and present the latched value on the bottom-right pin (SS3: 22, SS4: 21). An external 74'1g32 will expand PRG capacity to 512KB. 

### [VRC3](VRC3.xhtml "VRC3")

Same hardware, decode writes to $Fxxx. ASIC only implements three bits, so to maximize the address space, a full five bits of latch and OR gates are needed. 

### [LZ93D50](Bandai_FCG_board.xhtml "Bandai FCG board")

Same hardware. Decode writes when (A & 0x800F) == 0x8008 

### [TAM-S1](INES_Mapper_097.xhtml "INES Mapper 097")

TAM-S1 already supports 512KB PRG, just requiring re-connecting data bus pins. An external resistor can further extend support to 1MB. 

However, I'm proud of this previous design, so it gets to stay: 74'161, 74'00. Invert D5 and D6 using two NAND gates. Latch /D5 and /D6 on /ROMSEL=0 & R/W=0 (standard discrete logic mapper '161 circuit). Generate PRG A19 and A20 as NAND2(/Q5,A14) and NAND2(/Q6,A14). 

### [FCG-1 and -2](Bandai_FCG_board.xhtml "Bandai FCG board")

Decode writes when (A & 0x200F) == 0x2008; this just fits into the 7 inputs on a 74'138 + 74'161. One mirror of this register overlaps a mirror of the PPU, and two mirrors overlap the PRG, but that shouldn't matter. 
