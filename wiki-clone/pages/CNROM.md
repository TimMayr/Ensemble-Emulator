# CNROM

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/CNROM) | View [other pages](Special_AllPages.xhtml#CNROM)

**CNROM**

**Company** | Nintendo, others   
---|---  
**Boards** | CNROM   
**PRG ROM capacity** | 32K   
**PRG ROM window** | n/a   
**PRG RAM capacity** | None   
**CHR capacity** | 32K   
**CHR window** | 8K   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | Fixed H or V, controlled by solder pads   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | Yes   
**IRQ** | No   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | [003](CNROM.xhtml "INES Mapper 003"), [185](CNROM.xhtml "INES Mapper 185")  
  
**NESCartDB**

[iNES 003](https://nescartdb.com/search/advanced?ines=3)  
---  
[iNES 185](https://nescartdb.com/search/advanced?ines=185)  
[CNROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=CNROM)  
  
**CNROM** is a discrete-logic circuit board providing up to four 8 KB banks of CHR-ROM. Two iNES mapper numbers denote its two configurations: 

  * **iNES Mapper 3** denotes the common usage mounting 16-32 KiB of CHR-ROM. 
    * Submapper 0: Bus conflict behavior unknown
    * Submapper 1: No bus conflicts
    * Submapper 2: AND-type bus conflicts
  * **iNES Mapper 185** denotes a special usage mounting only 8 KiB of CHR-ROM. CHR-ROM is disabled unless the correct bank number has been selected. 
    * Submapper 0: CHR-ROM-enabling CS1/CS2 values unknown
    * Submapper 4: CS1/CS2=0 enables CHR-ROM, all other values disable CHR-ROM
    * Submapper 5: CS1/CS2=1 enables CHR-ROM, all other values disable CHR-ROM
    * Submapper 6: CS1/CS2=2 enables CHR-ROM, all other values disable CHR-ROM
    * Submapper 7: CS1/CS2=3 enables CHR-ROM, all other values disable CHR-ROM



Two Bandai boards extend the functionality of CNROM: 

  * [_Family Trainer: Jogging Race_](https://nescartdb.com/profile/view/4090/family-trainer-4-jogging-race%7C) is a simple oversized version of CNROM and therefore is assigned to mapper 3 as well;
  * [_Family Trainer: Aerobics Studio_](https://nescartdb.com/profile/view/3953/family-trainer-3-aerobics-studio%7C) adds a [M50805 speech chip](https://forums.nesdev.org/viewtopic.php?p=102300#p102300%7CMitsubishi) using an extra register. The chip is currently unemulated, but once it is, the speech ROM data will be included as NES 2.0 Misc. ROM data.



The Namco game [_Hayauchi Super Igo_](https://nescartdb.com/profile/view/3844/hayauchi-super-igo%7C) adds 2 KiB of PRG-RAM, denoted using mapper 3 and the appropriate value in the header's PRG-RAM size field. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 Bank Select: responds to writes to CPU $8000-$FFFF
      * 2.1.1 Regular mapper 3 with up to 32 KiB
      * 2.1.2 Oversize mapper 3 with up to 128 KiB
      * 2.1.3 Mapper 185
    * 2.2 Speech Start/Message Select: responds to writes to CPU $6000-$7FFF
  * 3 Hardware
    * 3.1 Security diodes
    * 3.2 Solder Pad Configuration



# Banks

  * CPU $6000-$7FFF: 2 KiB of PRG-RAM, mirrored three times (_Hayauchi Super Igo_ only)
  * CPU $8000-$FFFF: 32 KB unbanked PRG-ROM
  * PPU $0000-$1FFF: 8 KB switchable window into 32 KiB CHR-ROM
  * Nametable arrangement: Fixed; solder pad selects between Horizontal and Vertical



# Registers

## Bank Select: responds to writes to CPU $8000-$FFFF

### Regular mapper 3 with up to 32 KiB
    
    
    D~[..DC ..BA] A~[1... .... .... ....]
         ||   ||
         ||   ++- CHR A14..A13 (8 KiB bank)
         |+------ Output to Diode 2 (D2)
         +------- Output to Diode 1 (D1)
    

The original CNROM board is always subject to AND-type bus conflicts: the effective value is the value being written bitwise-AND'd with the PRG-ROM content at the address being written to. iNES Mapper 3's submapper indicates whether bus conflicts should actually be emulated. So far, the only .NES files requiring the absence of bus conflicts have been mapper hacks to mapper 3 from other mappers. 

### Oversize mapper 3 with up to 128 KiB
    
    
    D~[.... DCBA] A~[1... .... .... ....]
            ||||
            ++++- CHR A16..A13 (8 KiB bank)
    

Among the licensed NES/Famicom library, this configuration is only used by Bandai's _Family Trainer: Jogging Race_. Several unlicensed cartridges mount 64 KiB of CHR-ROM; although they work as oversize mapper 3 as well, they are canonically assigned to [INES Mapper 148](INES_Mapper_148.xhtml "INES Mapper 148") instead. 

### Mapper 185
    
    
    D~[..DC ..BA] A~[1... .... .... ....]
         ||   ||
         ||   |+- Chip Select 2 (CS2)
         ||   +-- Chip Select 1 (CS1)      
         |+------ Output to Diode 2 (D2)
         +------- Output to Diode 1 (D1)
    

Mapper 185 always has AND-type bus conflicts. The submapper number denotes the correct Chip Select value that enables CHR-ROM; if another value is active, CHR-ROM is disabled, meaning that reading from the pattern tables returns [open bus](Open_bus_behavior.xhtml#PPU_open_bus "Open bus behavior"). Theoretically, this should return the LSB of the address read, but real-world behavior varies, and the earlier revision of Mighty Bomb Jack in fact relies on open bus at PPU address $0000 being something other than $00 (by means of a 10k pull-up resistor on the CHR ROM's D0 pin). If the correct Chip Select value is not known -- denoted by submapper 0, which applies to all .NES files without a [NES 2.0](NES_2_0.xhtml "NES 2.0") header, the simple heuristic "Disable CHR-ROM for the first two reads $2007 after a reset and then enable it" will work with all known games. 

| bank | PPU addr | test | bank | PPU addr | test   
---|---|---|---|---|---|---  
Game | Incorrect bank | Must work   
[Bird Week](https://nescartdb.com/profile/view/1262) | $F0 | $1FF0 | ≠ $0C | $0F | $1FF0 | = $0C   
[B-Wings](https://nescartdb.com/profile/view/1264) | $00 | $0000 | ≠ $3C | $33 | $0000 | = $3C   
[Mighty Bomb Jack (J, PRG0)](https://nescartdb.com/profile/view/1261) | $00 | $0000 | ≠ $00 | $11 | untested   
Mighty Bomb Jack (J, PRG1) | $00 | $0001 | ≠ $3C | $11 | untested   
Sansuu 1 Nen | $20 | $000C | ≠ $BC | $22 | $000C | = $BC   
[Sansuu 2 Nen](https://nescartdb.com/profile/view/1263) | $20 | $0003 | ≠ $42 | $22 | $0003 | = $42   
[Othello](https://nescartdb.com/profile/view/4061) | $20 | $0006 | ≠ $3F | $22 | $0006 | = $3F   
[Sansuu 3 Nen](https://nescartdb.com/profile/view/3791) | $00 | $0006 | ≠ $34 | $2A | $0006 | = $34   
[Spy vs Spy](https://nescartdb.com/profile/view/3592) | $13 | $1F20 | ≠ $55 | $21 | untested   
[Seicross](https://nescartdb.com/profile/view/2332) | $21 | $0700…$0707 | ≠ $20,$60,$70,$70,$70,$40,$08,$38 | $20 | untested   
  
The North American and PAL versions of _Mighty Bomb Jack_ mount 32 KiB of CHR-ROM instead and so use normal [mapper 3](CNROM.xhtml "INES Mapper 003"). 

## Speech Start/Message Select: responds to writes to CPU $6000-$7FFF
    
    
    D~[.D.. .CBA] A~[011. .... .... ....]
        |    +++- Message number (0-7)
        +-------- M5085 /SYNC signal
    

A falling edge of the D bit starts the selected speech line. This register only exists on the PCB of _Family Trainer: Aerobics Studio_. 

# Hardware

The 4-bit latch register is implemented using a [74HC161](74161.xhtml "74161") binary counter. On iNES Mapper 3, the latch's A and B outputs directly connect to the CHR-ROM chip's A14 and A13 inputs. On iNES Mapper 185, the particular ROM chip model (Sharp LH2367 or equivalent) changes the functionality of pins 26 (normally A13) and 27 (normally A14) into two programmable (during manufacturing) Chip Select inputs, effectively disabling CHR-ROM and tri-stating its data outputs when the one correct bank of four possible CHR-ROM banks is not selected, as a copy-protection mechanism. 

## Security diodes

As another copy protection mechanism, the CNROM circuit board has a spot for two diodes that produce additional bus conflicts to hinder cartridge dumping attempts. Diode 1 connects latch bit 'D' to CHR-ROM A10; Diode 2 connects latch bit 'C' to CHR-ROM A12. Each latch bit must be set such that the diode does _not_ allow current to flow, because if it does, AND-type bus conflicts occur that may cause the wrong CHR-ROM A10 and A12 signals to be applied, depending on the relative output resistances of latch chip and console or dumping device. Modern Kazzo-like dumping devices have strong enough output drivers to always win these bus conflicts, but 1980s' dumping equipment as well as the NES PPU will lose these bus conflicts and produce an unusable readout. Each diode can be mounted either with the anode or the cathode facing the latch output. As a diode will allow current to flow if the anode-side voltage is significantly higher than the cathode-side voltage, the latch bit must be 0 if it faces the anode, and 1 if it faces the cathode. The security diodes were mounted by most Nintendo-manufactured Japanese CNROM games manufactured in 1986 as well as Bandai-manufactured CNROM games from 1986 and 1987. They were never used on North American or PAL CNROM games, even as the NES-CNROM board has an unpopulated spot for them until at least revision -02. Emulators need not emulate the functionality of security diodes to run games correctly. 

## Solder Pad Configuration

  * Horizontal nametable arrangement ("vertical mirroring"): 'H' connected, 'V' disconnected.
  * Vertical nametable arrangement ("horizontal mirroring"): 'H' disconnected, 'V' connected.
  * 16 KB PRG-ROM: 'SL' connected, 'CL' disconnected.
  * 32 KB PRG-ROM: 'SL' disconnected, 'CL' connected.



[_Hayauchi Super Igo_](https://nescartdb.com/profile/view/3844/hayauchi-super-igo) is a CNROM-like board with a 2KB SRAM mapped at $6000, using a [74HC10](7410.xhtml "7410") as the address decoder. 

Theoretically the bank select register could be implemented with a [74HC377](74377.xhtml "74377") octal D latch, allowing up to 2 megabytes of CHR ROM. 

Categories: [CNROM-like mappers](Category_CNROM_like_mappers.xhtml), [Expansion audio](Category_Expansion_audio.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [Nintendo licensed mappers](Category_Nintendo_licensed_mappers.xhtml)
