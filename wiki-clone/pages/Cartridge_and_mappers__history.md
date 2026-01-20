# Cartridge and mappers' history

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Cartridge_and_mappers%27_history) | View [other pages](Special_AllPages.xhtml#Cartridge_and_mappers__history)

1983 July 15th
    The Nintendo Family Computer ([Famicom](Family_Computer.xhtml "Famicom")) is released in Japan. The three launch games are _Donkey Kong_ , _Donkey Kong Jr._ and _Popeye_. During the first year, only Nintendo released games for the system. All of them had 16 KiB PRG ROM and 8 KiB CHR ROM.
1984 June 21th
    _Family BASIC_ is released. It is the first special cart Nintendo made for the system: instead of being a game it allows the user to program the system using the BASIC programming language. It's also the first cart that uses 32 KiB of PRG. Additionally it provides 2 KiB of battery-backed PRG RAM. It is the first cart (for a long while) to use either PRG RAM or battery backup.
1984 July 28th/31st
    Hudson's _Nuts & Milk_ and _Lode Runner_ appear to be the first third-party games released on the Famicom. The cartridges were manufactured by Nintendo though.
1984 November 2nd/8th
    Namco's _Pac Man_ and _Xevious_ appear to be the first releases for the Famicom that were entirely manufactured by a third party. _Xevious_ appears to be the first 32 KiB PRG game for the system (as _Family BASIC_ isn't a game). Because of the lack of any lockout chip, Namco and many other companies (Bandai, Irem, Jaleco, Konami, Sunsoft, Taito) will follow this trend and make their own cartridges instead of having Nintendo make them for them.
1985 September 13th
    Nintendo's _Super Mario Bros._ is released in Japan. It is one of the first (if not the first) Famicom games that is not merely a port of another arcade or computer game. This will be a major factor in the console's success.
1985 September 27th
    Jaleco released _City Connection_ , using [some circuitry](INES_Mapper_087.xhtml "INES Mapper 087") to allow 16 KiB of CHR ROM, switching between two 8 KiB banks. This is the first Famicom game to use hardware other than ROMs (not counting _Family BASIC_). Mappers were born.
1985 October 18th
    The Nintendo Entertainment System (NES) is released in the United States. The launch games are _10-Yard Fight_ , _Baseball_ , _Clu Clu Land_ , _Donkey Kong Jr. Math_ , _Duck Hunt_ , _Excitebike_ , _Golf_ , _Gyromite_ , _Hogan's Alley_ , _Ice Climber_ , _Kung Fu_ , _Mach Rider_ , _Pinball_ , _Stack-Up_ , _Super Mario Bros._ , _Tennis_ , _Wild Gunman_ , and _Wrecking Crew_. Some of these contained 60-pin Famicom cartridge PCBs connected to an internal Famicom-to-NES adapter, while others used 72-pin cartridge PCBs designed specifically for the NES. This list already includes some third-party games but this time the [lockout chip](CIC_lockout_chip.xhtml "CIC lockout chip") inside the NES prevented them from building their own cartridges.
1985 November 16th
    Konami's _Hyper Olympic Gentaiban!_ , a limited-release sequel to a game released in June, is the second game to use the CHR ROM switch circuit first seen in _City Connection_.
1986 February 21st
    Nintendo released the [Famicom Disk System](Family_Computer_Disk_System.xhtml "Family Computer Disk System") (FDS) accessory, which vastly improved the capabilities of the console (as almost no games were yet using mappers). One disk can hold 128 KB of data (64 KB per side), and was cheaper to produce. Additionally it provided extra sound and the disks could hold save data.
1986 April
    Bandai, Konami and Nintendo appear to start producing [CNROM](CNROM.xhtml "CNROM") games almost simultaneously. It's a cost-saving improvement over Jaleco's circuit that uses only one chip instead of two, at the cost of having [bus conflicts](Bus_conflict.xhtml "Bus conflict").
1986 April 17th
    Sunsoft's _Atlantis no Nazo_ appears to be the first cartridge that uses a [dedicated mapper chip](INES_Mapper_184.xhtml "INES Mapper 184") (as opposed to 74 series logic chips). Other companies follow this trend promptly.
1986 June 6th
    Nintendo's _Gumshoe_ (released in USA) is the first [GNROM](GxROM.xhtml "GNROM") game. It's the first game to bank PRG ROM, have 128 KiB of PRG ROM, and to support banking both PRG and CHR.
1986 June 13th
    Capcom's _Maikamura_ (a.k.a. _Ghosts 'n' Goblins_) appears to be the first [UNROM](UxROM.xhtml "UNROM") game. It is the first cartridge game to use CHR RAM.
1986 June 20th
    Namco's _Super Chinese_ appears to be the first game that uses a complex [ASIC](Category_ASIC_mappers.xhtml "Category:ASIC mappers") mapper. [Namco's 108](INES_Mapper_206.xhtml "INES Mapper 206") is the MMC3's predecessor.
1986 July 30th
    Konami's [VRC1](VRC1.xhtml "VRC1") (the second ASIC mapper) is first used in _Ganbare Goemon! Karakuri Douchuu_. After the FDS, the VRC1 is the first mapper that supports changing the nametable layout ("[mirroring](Mirroring.xhtml "Mirroring")") at runtime.
1986 September 1st
    The NES is released in PAL-B regions.
1986 December 15th
    Irem's _Mashou_ is the first game published to use a [BNROM](INES_Mapper_034.xhtml "BNROM") board. It (along with the Western localized version _Deadly Towers_) would turn out to be the only licensed game on that board.
1987 April 14th
    Seta's _Morita Shougi_ appears to be the first game to use the Nintendo [MMC1](MMC1.xhtml "MMC1") mapper. It's also the first game cartridge with 8 KiB of PRG RAM and battery backup.
1987 May 15th
    The NES is released in PAL-A regions.
1987 June 30th
    Taito's [X1-005](INES_Mapper_080.xhtml "INES Mapper 080") is first used in _Mirai Shinwa Jarvas_. The X1-005 is the first mapper to provide a small amount of extra RAM on-die, which was almost always used for saving.
1987 July 7th
    Konami's [VRC2](VRC2_and_VRC4.xhtml "VRC2") is first used in _Getsufuu Maden_. The VRC2 is the first mapper to support eight independent CHR banks.
1987 September 25th
    Konami's [VRC3](VRC3.xhtml "VRC3") is first and only used in _Salamander_. After the FDS, the VRC3 is the first mapper that can generate [IRQs](IRQ.xhtml "IRQ").
1987 September
    Nintendo's [MMC2](MMC2.xhtml "MMC2") is used in _Mike Tyson's Punch-out!!_. It's the first mapper to support hardware trickery to bypass the 256-tile background limitation.
1987 December
    Rare's _Wizards & Warriors_ is the first game published to use an [ANROM](AxROM.xhtml "AxROM") board.
1987 December 4th
    Namco's _Star Wars_ is the first game published to use their [129](INES_Mapper_019.xhtml "Namco 163") mapper IC, which supported ROM nametables and expansion audio. (However, this game used neither.)
1988 March 18th
    Irem released _Napoleon Senki_ , the first game to have [four-screen mirroring](Mirroring.xhtml#4-Screen "Mirroring"), and to contain both CHR RAM and CHR ROM.
1988 August 12th
    Namco's _Final Lap_ is the first cartridge game (i.e. not FDS) to use [expansion audio](Category_Expansion_audio.xhtml "Category:Expansion audio"), via its [Namco 163](Namco_163_audio.xhtml "Namco 163 audio") mapper.
1988 September 27th
    Seta's _8 Eyes_ appears to be the first game to use the Nintendo [MMC3](MMC3.xhtml "MMC3") mapper.
1990 February 3rd
    Koei's _Nobunaga no Yabou: Sengoku Gunyuuden_ (Japanese version of _Nobunaga's Ambition II_) was the first game to use the Nintendo [MMC5](MMC5.xhtml "MMC5") mapper. However, the writing "(c) 1989 Nintendo" and the date code on MMC5 chips suggest the chip already being available in late 1989.
1990 April
    Rare's _[Pin•Bot](INES_Mapper_119.xhtml "INES Mapper 119")_ is the first game that can bankswitch CHR-RAM and CHR-ROM independently.
1990 August 11th
    Bandai's _[SD Gundam Gaiden: Knight Gundam Monogatari](INES_Mapper_159.xhtml "INES Mapper 159")_ is the first game to save games in EEPROM.
1991 August 30th
    HAL Laboratory released _Metal Slader Glory_ , the largest licensed game with 512KB of PRG ROM and CHR ROM.
1994 June 24th
    Hudson Soft's _Takahasi Meijin no Boukenjima (Adventure Islands) IV_ is the last licensed game released for the Famicom.
1994 December
    Nintendo's _Wario's Woods_ is the last licensed game for the NES in USA, and the only NES game to receive an [ESRB](https://en.wikipedia.org/wiki/Entertainment_Software_Ratings_Board "wikipedia:Entertainment Software Ratings Board") rating.
1995 May 25
    _The Lion King_ (PAL only) is the last licensed game for the NES.[1]
2003 September 25th
    Nintendo discontinued all manufacturing and support for the Famicom.

**Note:** this list is not a citeable source for release dates; all dates either come from [NEScartDB](https://nescartdb.com) or Wikipedia. 

## References

  1. ↑ [The Lion King (video game)](https://en.wikipedia.org/wiki/The_Lion_King_\(video_game\) "wikipedia:The Lion King \(video game\)") – Wikipedia


