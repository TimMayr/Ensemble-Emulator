# Vs. System

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Vs._System) | View [other pages](Special_AllPages.xhtml#Vs__System)

The **Vs. UniSystem** and **Vs. DualSystem** are two different cabinet variations that use same "MDS-01-CPU" through "MDS-05-CPU" arcade boards. They are closely related to the NES. 

Like two NESes, both CPUs have 2 KiB of RAM from $0000-$1FFF and a PPU at $2000-$3FFF. The PPUs are [RGB ones](PPU_palettes.xhtml#2C03_and_2C05 "PPU palettes"), so colors and some registers may behave differently. 

The CPUs are always the letterless revision, and the arcade board is incompatible with a 2A03G, so tonal noise is not supported. There are other resulting differences, but they do not seem to be relevant. 

Registers to operate coin counters were added. The upper seven bits read from $4016 and $4017 have changed meaning. 

2 KiB of shared RAM mapped from $6000-$7FFF was added. In the DualSystem it can be swapped between CPUs; in the UniSystem it is either ignored or a special jumper "2A04" was installed to give dedicated access. There's a holder for two AA batteries to provide battery backup for this RAM only. _Vs. Ice Climber_ uses this RAM to count coins inserted into the coin slots on the main and sub systems.[[1]](https://forums.nesdev.org/viewtopic.php?p=233038#p233038)

Unlike the NES, the Vs. System always provides 4 KiB of RAM for nametables, and all games always use 4-screen layout instead of horizontal or vertical mirroring 

## Contents

  * 1 Synchronization
  * 2 Palette
  * 3 Registers
    * 3.1 Controller and CHR ROM bank ($4016 write)
    * 3.2 Right stick data, coins, and DIP switches ($4016 read)
    * 3.3 Left stick data, DIP switches, and watchdog timer ($4017 read)
    * 3.4 Coin counter ($4020-$403F, &c)
    * 3.5 Copy Protection
  * 4 Supported mappers
  * 5 References
  * 6 See also



## Synchronization

The Vs. System manual refers to the two sides as the "main" and "sub" sides, as does _Vs. Ice Climber_.[1] (Other video game concepts that use the "main" and "sub" terminology include Super NES color math, Sega CD CPUs, and Nintendo DS CPUs.) 

The Vs. DualSystem has a main CPU, main PPU, sub CPU, and sub PPU. All four are fed by the same clock, and all four are released from reset by the watchdog at the same time. The RGB PPUs used in the Vs. System never have the missing dot that the 2C02 does, and so it is never possible for the two PPUs to not be anything but perfectly synchronized: on every PPU cycle, the exact same name table and attribute table fetches will happen at the same time. NMI, if enabled, will be asserted at the same time. 

## Palette

There are several different RGB PPUs used in Vs. games. To determine which PPU is used, read the PPU type byte of the [NES 2.0](NES_2_0.xhtml "NES 2.0") header if available; otherwise, use the hash of the PRG and CHR ROM data. 

2C03
    This PPU is used in _Duck Hunt_ and _Tennis_ , as well as the PlayChoice, Famicom Titler, and Famicom TVs. Its colors closely resemble those of the 2C02 in the standard NTSC NES.
2C04
    There are four versions of this PPU with different permutations of the same (but different from the 2C03) [60-color master palette](PPU_palettes.xhtml#2C04 "PPU palettes"). This was used as a form of copy protection, so that games would have wrong colors if someone were to burn a new PRG ROM and CHR ROM and put them on the game PCB. Fading requires lookup tables with these PPUs.
2C05
    This PPU has the same colors as the 2C03, but it swaps the meanings of $2000 and $2001 and returns a constant identifying value in bits 4-0 of $2002. This is also for copy protection, causing games to write 1 to bit 7 of the wrong register, never receive [NMI](NMI.xhtml "NMI"), and lock up. However, there exist circuits to adapt 2C05 for boards that expect 2C03.

## Registers

Registers $4016 and $4017 have additional bits related to coin insertion and difficulty switches, and $4020 is a new register. All stick-controlled Vs. System games are wired to connect the left stick to $4017 and the right stick to $4016, but some games expect the first player to use the right stick, while others expect the first player to use the left stick.[2][3] The 2C05 swaps [PPUCTRL and PPUMASK](PPU_registers.xhtml "PPU registers"). Otherwise, all registers have the same meanings as on the NES or PlayChoice. 

### Controller and CHR ROM bank ($4016 write)
    
    
    7  bit  0
    ---- ----
    xxxx xCRS
          |||
          ||+- 1 then 0: Request a report from the [joysticks](Standard_controller.xhtml "Standard controller") or [Zapper](Zapper.xhtml "Zapper")
          |+-- In the DualSystem, does two things:
          |    #1: Used to request an IRQ on the _other_ CPU (if its interrupts are enabled).
          |        When 0: asserts an IRQ request.
          |        When 1: releases the IRQ request.
          |    #2: On the primary CPU only, controls which CPU can access 2 KiB of shared RAM mapped in the $6000-$7FFF region.
          |        When 1: the primary CPU has access and the secondary CPU sees open bus.
          |        When 0: the secondary CPU has access and the primary CPU sees open bus.
          |        The secondary CPU has no direct control over access.
          +--- Select 8 KiB CHR ROM bank for PPU $0000-$1FFF ([mapper 99](INES_Mapper_099.xhtml "INES Mapper 099") games only)
               Note: In case of games with 40KiB PRG-ROM (as found in VS Gumshoe),
                     the above bit additionally changes 8KiB PRG-ROM at $8000-$9FFF.
    

VS Super Mario Bros. requires dedicated exclusive access to the shared memory. The bundled RP2A04 is nothing but a jumper that ties the primary socket's OUT1 pin to ground. This means that Vs. SMB can't use IRQs, because its CPU's /IRQ pin is always asserted. 

### Right stick data, coins, and DIP switches ($4016 read)
    
    
    7  bit  0
    ---- ----
    PCCD DS0B
    |||| ||||
    |||| |||+- Buttons for right stick (A, B, 1, 3, Up, Down, Left, Right)
    |||| ||+-- always 0 (from floating input on 74LS240)
    |||| |+--- Service button (commonly inserts a credit)
    |||+-+---- DIP switches "2" and "1", respectively
    |++------- Coin inserted (read below)
    +--------- 0: Game is running on the primary CPU (it controls which CPU has access to shared RAM)
               1: Game is running on the secondary CPU (it must prevent watchdog timer timeout)
    

The "coin inserted" signals are the contact switches in the coin acceptors. The default UniSystem harness wiring connects both coin acceptors in parallel, and only to the $20s bit, so that bit is the only one of the two that goes high when a coin is inserted in either acceptor. However, games MUST check both bits because the coin acceptors could have been connected independently. Some games refuse to grant a credit if the "coin inserted" signal is true for too long, to protect against malicious arcade-goers. 

The coin collectors will read as true for approximately 40 to 70ms. A game should check the registers at least every other NMI to be assured of not eating the player's coins. 

The service button is debounced with an RC network: after the button is released, it should take somewhere between 3 to 6 milliseconds until it reads as false. After the capacitor has been fully charged, it should take between 1 and 1.4 milliseconds until it reads as true. The game need not poll the service button with any frequency, since the button can be held by the arcade operator for an arbitrary amount of time. 

Games may, and even probably should, parse the two "coin inserted" bits independently: emulators should NOT set both bits high when a single virtual coin has been inserted. For example, VS Super Mario Bros. gives 2 credits if both coin bits are set. 

Unlike previous advice, it is not _necessary_ to acknowledge a coin insert by driving the coin counter, but the arcade operator would be cross if the game didn't! 

### Left stick data, DIP switches, and [watchdog timer](https://en.wikipedia.org/wiki/watchdog_timer "wikipedia:watchdog timer") ($4017 read)
    
    
    7  bit  0
    ---- ----
    DDDD DD0B
    |||| ||||
    |||| |||+- Buttons for left stick (A, B, 2, 4, Up, Down, Left, Right)
    |||| ||+-- always 0 (from floating input on 74LS240)
    ++++-++--- More DIP switches ("8" down to "3")
    

The secondary CPU must be present in socket 8J and it must be instructed to read from $4017 at least every 1.2 seconds, or else the watchdog timer will reset both CPUs, both PPUs, and both CPUs' bit at $4020. 

### Coin counter ($4020-$403F, &c)

The game is expected to keep track of the total number of coins inserted by toggling this line every time a coin is seen while reading $4016. 
    
    
    15   address 4    0  7  bit  0
    ---- ---- ---- ----  ---- ----
    010x xxxA xx1x xxxx  xxxx xxxC
            |                    |
            |                    +- (write) 1, delay, 0: Increment coin counter
            +---------------------- (read) Same as write, but see below
    

The port is mirrored across the entire range of $4020-$5FFF and may interfere with mappers that put [ports, ROM, or RAM in this range](Category_Mappers_using__4020__5FFF.xhtml "Category:Mappers using $4020-$5FFF"). When the latched value is 1, it drives an electromagnet in the coin counter. Writing 1 then 0 will increment the counter. The electromagnetic counter can vary from machine to machine, but the least common denominator is a 10Hz DC coin counter: driving the signal high for 50ms (3 vblanks) and then low for 50ms is guaranteed to work. Some counters may be able to be driven faster: one requires as little [as 16ms high and 22ms low](http://forums.nesdev.org/viewtopic.php?p=127442#p127442). This register is explicitly cleared on powerup and reset. 

Reading from the register effectively writes the value of [open bus](Open_bus_behavior.xhtml "Open bus"). No games use any of this bit's mirrors, or try to read from it. 

### Copy Protection

The games released for the Vs. System mostly relied on the variety of PPUs to prevent copyright infringement. A few third-party games added an extra IC as well; the three known games with this additional protection were used with Namco's [108](INES_Mapper_206.xhtml "INES Mapper 206"). 

## Supported mappers

The hardware used by games released during the Vs. System's commercial life is equivalent to the following mappers: 

  * [iNES Mapper 099](INES_Mapper_099.xhtml "INES Mapper 099") (games provided without a daughterboard, erroneously [NROM](NROM.xhtml "NROM")/iNES Mapper 000)
  * iNES Mapper 001 ([MMC1](MMC1.xhtml "MMC1"))
  * iNES Mapper 002 ([UxROM](UxROM.xhtml "UxROM"))
  * [iNES Mapper 067](INES_Mapper_067.xhtml "INES Mapper 067") (SUNSOFT-3)
  * iNES Mapper 075 ([VRC1](VRC1.xhtml "VRC1"), erroneously [iNES Mapper 151](INES_Mapper_151.xhtml "INES Mapper 151"))
  * [iNES Mapper 206](INES_Mapper_206.xhtml "INES Mapper 206") (Namco 108, erroneously [MMC3](MMC3.xhtml "MMC3")/[iNES Mapper 004](MMC3.xhtml "INES Mapper 004"))



## References

  1. ↑ [Forum post by zeroone, January 2019](https://forums.nesdev.org/viewtopic.php?p=233038#p233038)
  2. ↑ [NewRisingSun's summary of which games are NES-like (left controller=$4016) and which are reversed](https://forums.nesdev.org/viewtopic.php?p=220654#p220654)
  3. ↑ [Further discussion based on kmg's study of schematics](https://forums.nesdev.org/viewtopic.php?p=282689)



## See also

  * [VS System mainboard schematic](https://nesdev.org/VSSCHEM.pdf)
  * [VS UniSystem cabinet harness hookup diagram](https://nesdev.org/VS_Wiring.pdf)
  * [VS UniSystem conversion kit installation guide](https://nesdev.org/VS_UniSystem.pdf) for converting an older single-game Nintendo arcade machine to a VS UniSystem
  * [Lidnariq's Vs. System's characterization test ROM](https://forums.nesdev.org/viewtopic.php?p=127592#p127592), [Memblers's](https://forums.nesdev.org/viewtopic.php?p=127442#p127442) results, and [lupin3rd's](https://forums.nesdev.org/viewtopic.php?p=179196#p179196) test results.
  * [Lidnariq's Vs. System, N108, and N127 characterization test ROM](https://forums.nesdev.org/viewtopic.php?p=179718#p179718), and [lupin3rd's](https://forums.nesdev.org/viewtopic.php?p=179696#p179696) test results.
  * [Nocash's comments](https://problemkaputt.de/everynes.htm#vssystemprotections) on Vs. System copy protection mechanisms


