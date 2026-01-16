# Family Computer

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Family_Computer) | View [other pages](Special_AllPages.xhtml#Family_Computer)

[![](../wiki-images/Nintendo-Family-Computer.png)](File_Nintendo_Family_Computer_png.xhtml)

[](File_Nintendo_Family_Computer_png.xhtml "Enlarge")

The Family Computer, also known as the Famicom

The **Family Computer** (HVC-001: **Famicom** , **FC** for short) is a video game console made by Nintendo and sold in Japan starting in 1983. The console would later be sold in Taiwan and Hong Kong. 

The Nintendo Entertainment System (NES), which Nintendo sold outside Japan a couple years later, is nearly identical in behavior but with several changes in the cords, controllers, and system look. While the Famicom was made to look friendly and well matched for a Japanese household with bright colors, the NES was designed as an "entertainment system" in order to get it into American retail stores who had been burned by the [video game crash](https://en.wikipedia.org/wiki/Video_game_crash_of_1983 "wikipedia:Video game crash of 1983") (known in Japan as the Atari shock). 

The **New Famicom** (HVC-101: also known as the **AV Famicom**) is a Famicom model released in 1993 which outputs AV composite video. It is similar to the New-Style NES (NES-101). The original Famicom is sometimes called the RF Famicom to retroactively distinguish it from the AV Famicom, which is what this article will use. 

## Contents

  * 1 Differences from the NES
    * 1.1 Input
    * 1.2 Output
    * 1.3 Other
  * 2 References



## Differences from the NES

### Input

  * On the RF Famicom, two controllers are hard-wired to the console. The AV Famicom uses standard NES controller ports.
  * [Controller](Standard_controller.xhtml "Standard controller") 2 on the RF Famicom has a microphone in place of the Select and Start buttons; the missing buttons always return "not pressed".
  * The controllers on early RF Famicom revisions had square rubber buttons, which were replaced with round plastic buttons on later revisions.
  * There is a [15-pin expansion port](Expansion_port.xhtml "Famicom expansion port pinout") where the [Zapper](Zapper.xhtml "Zapper") and other peripherals connect to. 1- and 2-player games tend to [merge inputs](Controller_reading_code.xhtml#Standard_Read_for_2_Controllers_and_Famicom "Controller reading code") from from controllers 1 and 2 with controllers 3 and 4 connected this way.
  * Only one peripheral may be connected at once. Despite using NES controller ports, the AV Famicom isn't wired to accept peripherals made for the NES. If a peripheral was made for both the NES and the Famicom, the protocol may differ between the two, such as the [Power Pad](Power_Pad.xhtml "Power Pad"), [Arkanoid controller](Arkanoid_controller.xhtml "Arkanoid controller"), and [Four Score](Four_player_adapters.xhtml "Four Score").



### Output

  * The RF Famicom had only RF output, not the AV output seen on the front-loading NES. Modifications to produce an AV output are common on second-hand units.
  * The Famicom is always NTSC. PAL Famiclones were designed for compatibility with the Famicom; the [clock rate](Cycle_reference_chart.xhtml#Clock_rates "Cycle reference chart") preserved the ratio of PPU to CPU cycles, and extra scanlines were added to the post-render period instead of vertical blanking so that cycle-counting mapper IRQs and cycle-timed NMI handlers continue to work.
  * The Sharp Famicom Titler (AN-510) and Sharp C1 (Famicom TV) have different [palettes](PPU_palettes.xhtml#2C03_and_2C05 "PPU palettes") and [emphasis](Colour_emphasis.xhtml "Colour emphasis") behavior compared to other Famicom models due to them using RGB PPUs. Some commercial Famicom games are labelled as incompatible with these models.
  * The APU on the earliest Famicom revisions doesn't support 93-step [noise](APU_Noise.xhtml "APU Noise") (instead playing it as normal noise), and has a different noise period for rate $F.
  * The Famicom audio path loops through the cartridge connector. This allows cartridges to generate their own audio and mix it with the console's audio. A number of cartridges have their own [audio synthesizers](List_of_games_with_expansion_audio.xhtml "List of games with expansion audio"). _Famicom Karaoke Studio_ is an example of a cartridge that provides its own microphone. The Sharp C1 is incompatible with these games as it uses the audio path to detect inserted cartridges.



### Other

  * Reset acts like a top-loading NES, not a front-loading NES: the Reset button resets only the CPU, not the PPU.
  * The "cassette" connector on the Famicom is smaller than the "Game Pak" connector on the NES. Famicom cassettes have 60 pins instead of 72. However, the pin pitch is slightly wider: 2.54 mm (0.1 in) on the Famicom vs. a non-standard 2.50 mm on the NES.
  * No expansion port on the bottom, and no ten pass-through pins on the cassette connector.
  * No [CIC lockout chip](CIC_lockout_chip.xhtml "CIC lockout chip").



## References

  * [Family Computer - Nintendo](https://www.nintendo.com/jp/famicom/hardware/index.html) (Japanese)
  * [Family Computer - FamiWiki](https://famiwiki.net/wiki/Family_Computer)
  * [Family Computer - MarioWiki](https://www.mariowiki.com/Family_Computer)
  * [Family Computer - Wikipedia](https://ja.wikipedia.org/wiki/%E3%83%95%E3%82%A1%E3%83%9F%E3%83%AA%E3%83%BC%E3%82%B3%E3%83%B3%E3%83%94%E3%83%A5%E3%83%BC%E3%82%BF) (Japanese)
  * [System « Famicom World](https://famicomworld.com/system/)


