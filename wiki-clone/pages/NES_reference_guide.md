# NES reference guide

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_reference_guide) | View [other pages](Special_AllPages.xhtml#NES_reference_guide)

This page lists all major hardware reference divided by categories in their simplest form. From this list, you can drill down to a more specific section of the selected category. 

## Contents

  * 1 General
  * 2 Hardware reference
    * 2.1 Miscellaneous
  * 3 File format reference
  * 4 Emulation reference
  * 5 Notes
  * 6 External links



### General

  * [Glossary](Glossary.xhtml "Glossary")
  * [Cartridge and mappers' history](Cartridge_and_mappers__history.xhtml "Cartridge and mappers' history")



### Hardware reference

  * [2A03](2A03.xhtml "2A03"): [APU](APU.xhtml "APU"), [CPU](CPU.xhtml "CPU")
    * [Obelisk 6502 Guide](https://www.nesdev.org/obelisk-6502-guide/) \- CPU reference
  * [PPU](PPU.xhtml "PPU")
  * [Input devices](Input_devices.xhtml "Input devices")
  * [Mappers](Mapper.xhtml "Mapper")
  * [Pinout](Hardware_pinout.xhtml "Hardware pinout")
  * [Cycle reference chart](Cycle_reference_chart.xhtml "Cycle reference chart") for various PPU and frame-related timing details; includes CPU cycle counts
  * [Cartridge board](Cartridge_board_reference.xhtml "Cartridge board reference")
  * [RF Famicom wiring diagram](File_Neswires_jpg.xhtml "File:Neswires.jpg") (many parts also applicable to NES)
  * [Errata](Errata.xhtml "Errata"): Hardware bugs and quirks
  * [Famicom cartridge dimensions](Famicom_cartridge_dimensions.xhtml "Famicom cartridge dimensions")
  * Console5 Tech wiki has schematics and information: [Famicom](https://wiki.console5.com/wiki/Famicom), [NES-001](https://wiki.console5.com/wiki/Nintendo_NES-001), [NES-101](https://wiki.console5.com/wiki/Nintendo_NES-101)



#### Miscellaneous

  * [Family Computer](Family_Computer.xhtml "Family Computer") \- the original Japanese console
  * [Family Computer Disk System](Family_Computer_Disk_System.xhtml "Family Computer Disk System")
  * [Vs. System](Vs__System.xhtml "Vs. System")
  * [FamicomBox](FamicomBox.xhtml "FamicomBox")
  * [CIC lockout chip](CIC_lockout_chip.xhtml "CIC lockout chip")
  * [Myths](Myths.xhtml "Myths")
  * [PRG RAM circuit](PRG_RAM_circuit.xhtml "PRG RAM circuit")
  * [Implementing Mappers In Hardware](Implementing_Mappers_In_Hardware.xhtml "Implementing Mappers In Hardware")
  * [Tutorial on reading circuits in Visual 6502/2A03/2C02](Visual_circuit_tutorial.xhtml "Visual circuit tutorial")
  * [RF modulator board](MOD_RF.xhtml "MOD RF")
  * [Nintendo NES-001](http://console5.com/wiki/Nintendo_NES-001) on Console5
  * [Demo Vision](Demo_Vision.xhtml "Demo Vision")
  * [Fonts](Fonts.xhtml "Fonts") \- fonts used on official devices.
  * [List of games with significant regional differences](List_of_games_with_significant_regional_differences.xhtml "List of games with significant regional differences")
  * [Nintendo header](Nintendo_header.xhtml "Nintendo header") \- a metadata header found in ~33% of licensed NES games.
  * [Enri's Homepage: Famicom](http://cmpslv3.stars.ne.jp/Famic/Famic.htm) (Japanese) - transcription of some official Famicom documentation, and other notes.
  * [NES Music Ripping Guide](https://nesdev.org/nesaudio.zip) by Chris Covell
  * [Battery holder](Battery_holder.xhtml "Battery holder") \- guide for adding a battery holder to an NES cartridge
  * [NES Classic Controller for Wii](NES_Classic_Controller_for_Wii.xhtml "NES Classic Controller for Wii") \- guide for adapting an NES controller to Wii



### File format reference

  * [iNES](INES.xhtml "INES")
    * [NES 2.0](NES_2_0.xhtml "NES 2.0")
  * [UNIF](UNIF.xhtml "UNIF")
  * [NSF](NSF.xhtml "NSF")
  * [FDS](Family_Computer_Disk_System.xhtml "FDS")
  * IPS 
    * [description](https://datacrystal.romhacking.net/wiki/Patch)
    * [file format specification](http://zerosoft.zophar.net/ips.htm)
    * [limitations and pitfalls](http://justsolve.archiveteam.org/wiki/IPS_\(binary_patch_format\))
    * [caveats and extensions](https://web.archive.org/web/20120310064326/http://romhack.wikia.com/wiki/IPS)



### Emulation reference

  * List of NES [emulators](Emulators.xhtml "Emulators")
  * [Emulator tests](Emulator_tests.xhtml "Emulator tests")
  * [Game bugs](Game_bugs.xhtml "Game bugs") \- games that display buggy behavior on the actual hardware
  * [Tricky-to-emulate games](Tricky_to_emulate_games.xhtml "Tricky-to-emulate games")
  * [Sprite overflow games](Sprite_overflow_games.xhtml "Sprite overflow games") \- games which use the [sprite overflow bug](PPU_sprite_evaluation.xhtml#Sprite_overflow_bug "PPU sprite evaluation") of [OAM](PPU_OAM.xhtml "OAM")
  * [Colour-emphasis games](Colour_emphasis_games.xhtml "Colour-emphasis games") \- games which make use of the colour emphasis bits of $2001
  * [Colour $0D games](Color__0D_games.xhtml "Colour $0D games") \- games which use the infra-black colour $0D.
  * [Expansion audio games](List_of_games_with_expansion_audio.xhtml "Expansion audio games") \- Famicom games that use extra audio hardware.



### Notes

  * All content refer to the NTSC system unless otherwise specified



### External links

  * [NESdev.org archive](https://www.nesdev.org/archive.html) \- Archive of the old NESdev webpage with many historical documents and links.
  * [NES-001 (frontloader)](http://console5.com/wiki/Nintendo_NES-001) and [NES-101 (toploader)](http://console5.com/wiki/Nintendo_NES-101) Control Deck PCB maps on Console5 TechWiki


