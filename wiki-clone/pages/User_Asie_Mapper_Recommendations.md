# User:Asie/Mapper Recommendations

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AAsie/Mapper_Recommendations) | View [other pages](Special_AllPages.xhtml#User_Asie_Mapper_Recommendations)

This page is a work in progress/proposal. 

Not sure which mapper to use for your homebrew project? Here's a (slightly opinionated) recommendation list, written under the following assumptions: 

  * You want to be able to distribute your game digitally (as a ROM) and physically (as a cartridge) going forward. As such, the list is mostly limited to mappers with home-scale manufacturing possibilities, with boards offered by independent makers.
  * You cannot design your own cartridge boards. (If you are an electrical engineer, you can technically use any [mapper](Mapper.xhtml "Mapper") as a base.)



Legend: 

  * "Emulator" refers to emulator support: 
    * "Perfect" means that all emulators in use today should have no problem running emulating the mapper.
    * "High" means that most emulators in use today should have no problem emulating the mapper.
    * "Medium" means that only specific emulators support the mapper perfectly; other emulators do not support it or present notable implementation issues.
    * "Low" means that no emulators support the mapper perfectly.
  * "Flashcart" refers to flashcart support: 
    * "Perfect" implies support by the KrzysioCart, PowerPak, N8 and N8 Pro.
    * "High" implies support by most of the above cartridges (commonly, excluding the Krzysiocart).
    * "Low" implies support by few of the above cartridges (commonly, N8 Pro only).
    * "No" implies no support.
  * "Purchase" refers to purchase availability: 
    * "Ready" refers to boards which can be purchased prepopulated for use with an external flasher. Cheaper DIY variants of such boards may also be available.
    * "DIY" refers to boards which require soldering at home to complete; that is, only DIY variants can be purchased.
    * "Tricky" refers to boards with poor availability; for example, the seller only has stock occasionally, or an uncommon donor chip is required separately.
    * "No" refers to boards which are not currently available on the market.
  * "DIY" refers to DIY viability: 
    * "Easy" refers to boards which can be easily soldered with easily available parts.
    * "Tricky" refers to boards which can be easily soldered, but parts availability may pose a challenge.
    * "Hard" refers to boards which require precision soldering and tools not available to a novice hobbyist.
  * "Donor?" refers to whether or not a donor cartridge is used as part of the process, either on the manufacturer or buyer's end.
  * "Action 53?" refers to whether or not this mapper can be used as the base for an Action 53 multicart; this is a requirement of the annual NESdev Compo.

| PRG | CHR |  | Accessibility   
---|---|---|---|---  
# | Mapper | Layout | Max (KiB) | Layout | Max (KiB) | Mirroring | Features | Comments | Emulator | Flashcart | Purchase | DIY | Donor? | Action 53?   
0  | [NROM](NROM.xhtml "INES Mapper 000") | Fixed | 32  | Fixed | 8  | Fixed H/V  |  |  | Perfect  | Perfect  | Ready  | Easy  | No  | Yes   
2  | [UNROM](UxROM.xhtml "INES Mapper 002") | 16 + 16F | 128/256  | Fixed | 8  | Fixed H/V  |  |  | Perfect  | Perfect  | Ready  | Easy  | No  | Yes   
30  | [UNROM 512](UNROM_512.xhtml "INES Mapper 030") | 16 + 16F | 512  | 8 | 32  | H/1 or V/1  | Flash  |  | High?  | High  |  | Easy  | No  | No   
111  | [GTROM](GTROM.xhtml "INES Mapper 111") | 32 | 512  | 8 | 16  | Fixed 4  | 2 LEDs, Flash, 3.75 KiB extra PPU RAM  |  | High?  | Medium  |  | Easy  | No  | No   
  
Some mappers are not listed in the table. While they are available for projects, they are not recommended for various reasons. 

## Contents

  * 1 Discrete mappers
    * 1.1 NROM (000)
    * 1.2 UNROM 512 (030)
    * 1.3 GTROM (111)



## Discrete mappers

This term refers to mappers which can be built from discrete logic chips, without using donor ASICs, programmable CPLDs or FPGAs. 

Only some discrete mappers are listed here, as they are typically very similar; a more complete list is available [here](User_Lidnariq_Discrete_Logic_Table.xhtml "User:Lidnariq/Discrete Logic Table"). 

### NROM (000)

The simplest mapper, or the lack of one: a PRG and CHR ROM chip connected to the cartridge bus. 

While this limits your game's scope, it also makes it the simplest to manufacture. 

Vendors: 

  * [Broke Studio](https://www.brokestudio.fr/product/nes-nrom-mapper-0-pcb/) (France; NES; pre-assembled, can be pre-flashed)
  * [MouseBiteLabs](https://www.etsy.com/listing/869384793/nintendo-nes-cartridge-circuit-board) (USA; NES; PCB only, many discrete mappers)
  * [Muramasa Entertainment](https://www.muramasaentertainment.com/product/nes-nrom/) (USA; NES; PCB only, also available in [green](https://www.muramasaentertainment.com/product/nes-nrom-green/))
  * [Muramasa Entertainment](https://www.muramasaentertainment.com/product/fc-nrom/) (USA; Famicom; PCB only, also available in [green](https://www.muramasaentertainment.com/product/fc-nrom-green/))
  * [Muramasa Entertainment](https://www.muramasaentertainment.com/product/nes-discrete-basic-black/) (USA; NES; PCB only, many discrete mappers)
  * [Muramasa Entertainment](https://www.muramasaentertainment.com/product/fc-discrete-basic-black/) (USA; Famicom; PCB only, many discrete mappers)
  * [RetroStage](https://retrostage.net/?product=nes-discrete-blaster-lite) (USA; NES; PCB only, many discrete mappers)



### UNROM 512 (030)

Note that support for the flashable variant is limited among emulators and flashcarts. 

Flashcart compatibility: 

  * KrzysioCart supports this mapper only in later revisions.
  * PowerPak supports this mapper with a supplemental add-on file; flash emulation is limited.
  * EverDrive N8 supports this mapper with a supplemental add-on file; flash emulation is missing.
  * EverDrive N8 Pro TODO



### GTROM (111)

Note that support for self-flashing is limited among emulators and flashcarts. 

Flashcart compatibility: 

  * KrzysioCart does not support this mapper.
  * PowerPak supports this mapper with a supplemental add-on file; flash emulation is missing.
  * EverDrive N8 supports this mapper with a supplemental add-on file; flash emulation is missing.
  * EverDrive N8 Pro TODO


