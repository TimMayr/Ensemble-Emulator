# PowerPak

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/PowerPak) | View [other pages](Special_AllPages.xhtml#PowerPak)

The **PowerPak** is a [Flash Cartridge](Category_Flash_Cartridge.xhtml "Category:Flash Cartridge") made by [RetroUSB](https://www.nesdev.org/w/index.php?title=RetroUSB&action=edit&redlink=1 "RetroUSB \(page does not exist\)"). It uses an [FPGA](https://www.nesdev.org/w/index.php?title=FPGA&action=edit&redlink=1 "FPGA \(page does not exist\)") to emulate a wide variety of [mappers](Mapper.xhtml "Mappers"), allowing the user to store a large collection of ROMs on a single Compact Flash card and run them on an NES. It is widely used by homebrew NES developers to test their software. It's also compatible with the [CopyNES](CopyNES.xhtml "CopyNES"). 

In addition to NES ROMs, the PowerPak is able to play [FDS](Family_Computer_Disk_System.xhtml "FDS") disk images, as well as [NSF](NSF.xhtml "NSF") music files. 

Famicom expansion audio is supported, and output on the EXP 6 expansion pin on the [cartridge connector](Cartridge_connector.xhtml "Cartridge connector"). A simple modification to the NES allows the expansion audio to be mixed with its output. 

Specifications: 

  * PRG size: 512 KB (252 KB for NSF)
  * CHR size: 512 KB
  * Auxiliary PRG-RAM size: 32 KB



Product page: <http://www.retrousb.com/product_info.php?products_id=34>

## Contents

  * 1 Mapper Compatibility
    * 1.1 PowerMappers
    * 1.2 Loopy's Mappers
    * 1.3 Myask's Mappers (WIP)
    * 1.4 Miscellaneous
    * 1.5 Offical Mappers V1.35b
  * 2 Software development limitations
  * 3 Utilities
  * 4 PowerPak development resources



## Mapper Compatibility

The PowerPak mappers have undergone several revisions, gradually improving compatibility. After official development ceased in 2010, Loopy and TheFox have each created a supplemental set of PowerPak mappers to improve its capabilities. 

The commonly recommended setup is: 

  * Begin with the Official mapper set.
  * Add Loopy's mapper set on top, replacing files.
  * Add the PowerMapper set on top if you want savestate support (see its readme).
  * Add any of the additional single mappers if needed.
  * Replace the N.MAP loader with a [patched version](https://forums.nesdev.org/viewtopic.php?p=283943#p283943) so newer ROMs with [ header byte 15 set](NES_2_0.xhtml#Default_Expansion_Device "NES 2.0") can be loaded.



### PowerMappers

TheFox created a set of revised PowerPak mappers to supplement or augment the existing ones, most notably adding a savestate feature for the mappers it contains, but removing the Game Genie feature. 

Download: <http://fo.aspekt.fi>

  * [NROM](NROM.xhtml "NROM") ([0](NROM.xhtml "INES Mapper 000"))
  * [MMC1](MMC1.xhtml "MMC1") ([1](MMC1.xhtml "INES Mapper 001"))
  * [UxROM](UxROM.xhtml "UxROM") ([2](UxROM.xhtml "INES Mapper 002"))
  * [CxROM](CxROM.xhtml "CxROM") ([3](CNROM.xhtml "INES Mapper 003"))
  * [MMC3](MMC3.xhtml "MMC3") ([4](MMC3.xhtml "INES Mapper 004"))
  * [AxROM](AxROM.xhtml "AxROM") ([7](AxROM.xhtml "INES Mapper 007"))
  * [MMC2](MMC2.xhtml "MMC2") ([9](MMC2.xhtml "INES Mapper 009"))
  * [MMC4](MMC4.xhtml "MMC4") ([10](MMC4.xhtml "INES Mapper 010"))
  * [ColorDreams](Color_Dreams.xhtml "ColorDreams") ([11](Color_Dreams.xhtml "INES Mapper 011"))
  * [BxROM](BNROM.xhtml "BxROM") ([34](INES_Mapper_034.xhtml "INES Mapper 034"))
  * [GxROM](GxROM.xhtml "GxROM") ([66](GxROM.xhtml "INES Mapper 066"))
  * [FME-7](Sunsoft_FME_7.xhtml "FME-7") ([69](Sunsoft_FME_7.xhtml "INES Mapper 069"), no expansion audio)
  * [Codemasters](INES_Mapper_071.xhtml "Codemasters") ([71](INES_Mapper_071.xhtml "INES Mapper 071"))
  * MMC3/[TxSROM](INES_Mapper_118.xhtml "TxSROM") ([118](INES_Mapper_118.xhtml "INES Mapper 118"))
  * MMC3/[TQROM](INES_Mapper_119.xhtml "TQROM") ([119](INES_Mapper_119.xhtml "INES Mapper 119"))



Known problems: 

  * Mapper 4 IRQ is delayed by 2 cycles like [RAMBO-1](RAMBO_1.xhtml "RAMBO-1") to work around noise issues with PPU A12. This does not significantly affect most games.
  * Mapper 69 does not support the [Sunsoft 5B expansion audio](Sunsoft_5B_audio.xhtml "Sunsoft 5B audio") used in Gimmick!



### Loopy's Mappers

Loopy released a set of revised PowerPak mappers in 2011, adding fixes and additional support for several mappers: 

Download: <http://3dscapture.com/NES/powerpak_loopy.zip>

  * [CNROM](CNROM.xhtml "CNROM") ([3](CNROM.xhtml "INES Mapper 003"))
  * [MMC3](MMC3.xhtml "MMC3") ([4](MMC3.xhtml "INES Mapper 004"))
  * [MMC5](MMC5.xhtml "MMC5") ([5](MMC5.xhtml "INES Mapper 005"), no expansion audio)
  * [N163](INES_Mapper_019.xhtml "N163") ([19](INES_Mapper_019.xhtml "INES Mapper 019"))
  * [VRC4](VRC2_and_VRC4.xhtml "VRC4") ([21](VRC2_and_VRC4.xhtml "INES Mapper 021"), [23](VRC2_and_VRC4.xhtml "INES Mapper 023"), [25](VRC2_and_VRC4.xhtml "INES Mapper 025"))
  * [VRC6](VRC6.xhtml "VRC6") ([24](VRC6.xhtml "INES Mapper 024"), [26](VRC6.xhtml "INES Mapper 026"))
  * [BxROM](BNROM.xhtml "BxROM") ([34](INES_Mapper_034.xhtml "INES Mapper 034"))
  * [Sunsoft-5B](Sunsoft_FME_7.xhtml "Sunsoft-5B") ([69](Sunsoft_FME_7.xhtml "INES Mapper 069"))
  * [Codemasters](INES_Mapper_071.xhtml "Codemasters") ([71](INES_Mapper_071.xhtml "INES Mapper 071"))
  * [JY Company](J_Y__Company_ASIC.xhtml "JY Company") ([90](J_Y__Company_ASIC.xhtml "INES Mapper 090"), partial)
  * [FDS](Family_Computer_Disk_System.xhtml "FDS")
  * [NSF](NSF.xhtml "NSF") (no MMC5 audio, no VRC7 audio)



Known problems: 

  * Mapper 4 IRQ has reliability issues due to PPU A12 noise, causing status bars etc. to jitter up and down on some systems. (A fixed version is available: [forum thread](https://forums.nesdev.org/viewtopic.php?t=24229).)



Notes: 

  * Mapper 3 now supports unlicensed oversize variants (e.g. used by Panesian games).
  * Mapper 4 now supports Startropics.
  * Mapper 5 does not support the [MMC5 expansion audio](MMC5_audio.xhtml "MMC5 audio").



### Myask's Mappers (WIP)

[Myask](User_Myask.xhtml "User:Myask") made a few mappers not yet covered by others. 

Download: [here](http://forums.nesdev.org/download/file.php?id=5989). BBS Thread: [here](http://forums.nesdev.org/viewtopic.php?f=9&t=14373). 

  * Irem G-101 ([032](INES_Mapper_032.xhtml "INES Mapper 032"), but _Major League_ hack/submapper not implemented)
  * Irem H3001 ([065](INES_Mapper_065.xhtml "INES Mapper 065"))
  * Taito TC0190 ([033](INES_Mapper_033.xhtml "INES Mapper 033")) 
    * Note that some mapper 48 roms have been mislabeled as mapper 33.
  * Taito TC0690, TC0190+PAL16R4 ([048](INES_Mapper_048.xhtml "INES Mapper 048"), very buggy)
  * Magicseries Corp ([107](INES_Mapper_107.xhtml "INES Mapper 107"))



### Miscellaneous

  * Action 53 mapper ([028](Action_53_mapper.xhtml "INES Mapper 028")): [forum thread](https://forums.nesdev.org/viewtopic.php?p=102718#p102718)
  * [GTROM](GTROM.xhtml "GTROM") ([111](GTROM.xhtml "INES Mapper 111")): [forum thread](https://forums.nesdev.org/viewtopic.php?f=9&t=16631)
  * Magic Kid GooGoo ([190](INES_Mapper_190.xhtml "INES Mapper 190")): [forum thread](https://forums.nesdev.org/viewtopic.php?f=9&t=15682)
  * [UNROM 512](UNROM_512.xhtml "UNROM 512") ([030](UNROM_512.xhtml "INES Mapper 030")): [forum post](https://forums.nesdev.org/viewtopic.php?p=236715#p236715)
  * [UXROM 512](https://www.nesdev.org/w/index.php?title=UXROM_512&action=edit&redlink=1 "UXROM 512 \(page does not exist\)") Support up to 512k PRG: [forum post](https://forums.nesdev.org/viewtopic.php?t=24230)
  * [SXROM](SxROM.xhtml "SXROM") ([001](MMC1.xhtml "INES Mapper 001")): [packaged with PR8](https://neilbaldwin.github.io/nes-audio/), but created by bunnyboy
  * NES 2.0 [Default Expansion Device](NES_2_0.xhtml#Default_Expansion_Device "NES 2.0") header incompatibility fix: [forum thread](https://forums.nesdev.org/viewtopic.php?p=283943#p283943)
  * Nova's Rad PowerPak Menu: [forum thread](https://https//forums.nesdev.org/viewtopic.php?t=12091) (alternative PowerPak menu)



### Offical Mappers V1.35b

The last official release of mappers was in 2010. It supports a wide variety of popular mappers. 

Download: [here](https://www.retrousb.com/product_info.php?products_id=34)

Supported mappers: 

[000](NROM.xhtml "INES Mapper 000") | [001](MMC1.xhtml "INES Mapper 001") | [002](UxROM.xhtml "INES Mapper 002") | [003](CNROM.xhtml "INES Mapper 003") | [004](MMC3.xhtml "INES Mapper 004") | [005](MMC5.xhtml "INES Mapper 005") | [006](INES_Mapper_006.xhtml "INES Mapper 006") | [007](AxROM.xhtml "INES Mapper 007") | [008](INES_Mapper_006.xhtml "INES Mapper 008") | [009](MMC2.xhtml "INES Mapper 009") | [010](MMC4.xhtml "INES Mapper 010") | [011](Color_Dreams.xhtml "INES Mapper 011") | [012](INES_Mapper_012.xhtml "INES Mapper 012") | [013](CPROM.xhtml "INES Mapper 013") | [014](INES_Mapper_014.xhtml "INES Mapper 014") | [015](INES_Mapper_015.xhtml "INES Mapper 015")  
---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---  
[016](INES_Mapper_016.xhtml "INES Mapper 016") | [017](INES_Mapper_017.xhtml "INES Mapper 017") | [018](INES_Mapper_018.xhtml "INES Mapper 018") | [019](INES_Mapper_019.xhtml "INES Mapper 019") | [020](INES_Mapper_020.xhtml "INES Mapper 020") | [021](VRC2_and_VRC4.xhtml "INES Mapper 021") | [022](VRC2_and_VRC4.xhtml "INES Mapper 022") | [023](VRC2_and_VRC4.xhtml "INES Mapper 023") | [024](VRC6.xhtml "INES Mapper 024") | [025](VRC2_and_VRC4.xhtml "INES Mapper 025") | [026](VRC6.xhtml "INES Mapper 026") | [027](INES_Mapper_027.xhtml "INES Mapper 027") | [028](Action_53_mapper.xhtml "INES Mapper 028") | [029](INES_Mapper_029.xhtml "INES Mapper 029") | [030](UNROM_512.xhtml "INES Mapper 030") | [031](INES_Mapper_031.xhtml "INES Mapper 031")  
[032](INES_Mapper_032.xhtml "INES Mapper 032") | [033](INES_Mapper_033.xhtml "INES Mapper 033") | [034](INES_Mapper_034.xhtml "INES Mapper 034") | [035](J_Y__Company_ASIC.xhtml "INES Mapper 035") | [036](INES_Mapper_036.xhtml "INES Mapper 036") | [037](INES_Mapper_037.xhtml "INES Mapper 037") | [038](INES_Mapper_038.xhtml "INES Mapper 038") | [039](INES_Mapper_039.xhtml "INES Mapper 039") | [040](INES_Mapper_040.xhtml "INES Mapper 040") | [041](INES_Mapper_041.xhtml "INES Mapper 041") | [042](INES_Mapper_042.xhtml "INES Mapper 042") | [043](INES_Mapper_043.xhtml "INES Mapper 043") | [044](INES_Mapper_044.xhtml "INES Mapper 044") | [045](INES_Mapper_045.xhtml "INES Mapper 045") | [046](INES_Mapper_046.xhtml "INES Mapper 046") | [047](INES_Mapper_047.xhtml "INES Mapper 047")  
[048](INES_Mapper_048.xhtml "INES Mapper 048") | [049](INES_Mapper_049.xhtml "INES Mapper 049") | [050](INES_Mapper_050.xhtml "INES Mapper 050") | [051](INES_Mapper_051.xhtml "INES Mapper 051") | [052](INES_Mapper_052.xhtml "INES Mapper 052") | [053](INES_Mapper_053.xhtml "INES Mapper 053") | [054](INES_Mapper_054.xhtml "INES Mapper 054") | [055](INES_Mapper_055.xhtml "INES Mapper 055") | [056](INES_Mapper_056.xhtml "INES Mapper 056") | [057](INES_Mapper_057.xhtml "INES Mapper 057") | [058](INES_Mapper_058.xhtml "INES Mapper 058") | [059](INES_Mapper_059.xhtml "INES Mapper 059") | [060](INES_Mapper_060.xhtml "INES Mapper 060") | [061](INES_Mapper_061.xhtml "INES Mapper 061") | [062](INES_Mapper_062.xhtml "INES Mapper 062") | [063](INES_Mapper_063.xhtml "INES Mapper 063")  
[064](RAMBO_1.xhtml "INES Mapper 064") | [065](INES_Mapper_065.xhtml "INES Mapper 065") | [066](GxROM.xhtml "INES Mapper 066") | [067](INES_Mapper_067.xhtml "INES Mapper 067") | [068](INES_Mapper_068.xhtml "INES Mapper 068") | [069](Sunsoft_FME_7.xhtml "INES Mapper 069") | [070](INES_Mapper_070.xhtml "INES Mapper 070") | [071](INES_Mapper_071.xhtml "INES Mapper 071") | [072](INES_Mapper_072.xhtml "INES Mapper 072") | [073](VRC3.xhtml "INES Mapper 073") | [074](INES_Mapper_074.xhtml "INES Mapper 074") | [075](VRC1.xhtml "INES Mapper 075") | [076](INES_Mapper_076.xhtml "INES Mapper 076") | [077](INES_Mapper_077.xhtml "INES Mapper 077") | [078](INES_Mapper_078.xhtml "INES Mapper 078") | [079](NINA_003_006.xhtml "INES Mapper 079")  
[080](INES_Mapper_080.xhtml "INES Mapper 080") | [081](INES_Mapper_081.xhtml "INES Mapper 081") | [082](Taito_X1_017.xhtml "INES Mapper 082") | [083](INES_Mapper_083.xhtml "INES Mapper 083") | [084](INES_Mapper_084.xhtml "INES Mapper 084") | [085](VRC7.xhtml "INES Mapper 085") | [086](INES_Mapper_086.xhtml "INES Mapper 086") | [087](INES_Mapper_087.xhtml "INES Mapper 087") | [088](INES_Mapper_088.xhtml "INES Mapper 088") | [089](INES_Mapper_089.xhtml "INES Mapper 089") | [090](J_Y__Company_ASIC.xhtml "INES Mapper 090") | [091](INES_Mapper_091.xhtml "INES Mapper 091") | [092](INES_Mapper_092.xhtml "INES Mapper 092") | [093](INES_Mapper_093.xhtml "INES Mapper 093") | [094](INES_Mapper_094.xhtml "INES Mapper 094") | [095](INES_Mapper_095.xhtml "INES Mapper 095")  
[096](INES_Mapper_096.xhtml "INES Mapper 096") | [097](INES_Mapper_097.xhtml "INES Mapper 097") | [098](INES_Mapper_098.xhtml "INES Mapper 098") | [099](INES_Mapper_099.xhtml "INES Mapper 099") | [100](INES_Mapper_100.xhtml "INES Mapper 100") | [101](INES_Mapper_101.xhtml "INES Mapper 101") | [102](INES_Mapper_102.xhtml "INES Mapper 102") | [103](INES_Mapper_103.xhtml "INES Mapper 103") | [104](PEGASUS_5_IN_1.xhtml "INES Mapper 104") | [105](NES_EVENT.xhtml "INES Mapper 105") | [106](INES_Mapper_106.xhtml "INES Mapper 106") | [107](INES_Mapper_107.xhtml "INES Mapper 107") | [108](INES_Mapper_108.xhtml "INES Mapper 108") | [109](INES_Mapper_109.xhtml "INES Mapper 109") | [110](INES_Mapper_110.xhtml "INES Mapper 110") | [111](GTROM.xhtml "INES Mapper 111")  
[112](INES_Mapper_112.xhtml "INES Mapper 112") | [113](INES_Mapper_113.xhtml "INES Mapper 113") | [114](INES_Mapper_114.xhtml "INES Mapper 114") | [115](INES_Mapper_115.xhtml "INES Mapper 115") | [116](INES_Mapper_116.xhtml "INES Mapper 116") | [117](INES_Mapper_117.xhtml "INES Mapper 117") | [118](INES_Mapper_118.xhtml "INES Mapper 118") | [119](INES_Mapper_119.xhtml "INES Mapper 119") | [120](INES_Mapper_120.xhtml "INES Mapper 120") | [121](INES_Mapper_121.xhtml "INES Mapper 121") | [122](INES_Mapper_122.xhtml "INES Mapper 122") | [123](INES_Mapper_123.xhtml "INES Mapper 123") | [124](INES_Mapper_124.xhtml "INES Mapper 124") | [125](INES_Mapper_125.xhtml "INES Mapper 125") | [126](NES_2_0_Mapper_534.xhtml "INES Mapper 126") | [127](INES_Mapper_127.xhtml "INES Mapper 127")  
[128](NES_2_0_Mapper_265.xhtml "INES Mapper 128") | [129](INES_Mapper_129.xhtml "INES Mapper 129") | [130](INES_Mapper_130.xhtml "INES Mapper 130") | [131](INES_Mapper_131.xhtml "INES Mapper 131") | [132](INES_Mapper_132.xhtml "INES Mapper 132") | [133](INES_Mapper_133.xhtml "INES Mapper 133") | [134](INES_Mapper_134.xhtml "INES Mapper 134") | [135](Sachen_8259.xhtml "INES Mapper 135") | [136](INES_Mapper_136.xhtml "INES Mapper 136") | [137](INES_Mapper_137.xhtml "INES Mapper 137") | [138](Sachen_8259.xhtml "INES Mapper 138") | [139](Sachen_8259.xhtml "INES Mapper 139") | [140](INES_Mapper_140.xhtml "INES Mapper 140") | [141](Sachen_8259.xhtml "INES Mapper 141") | [142](INES_Mapper_142.xhtml "INES Mapper 142") | [143](INES_Mapper_143.xhtml "INES Mapper 143")  
[144](INES_Mapper_144.xhtml "INES Mapper 144") | [145](INES_Mapper_145.xhtml "INES Mapper 145") | [146](NINA_003_006.xhtml "INES Mapper 146") | [147](INES_Mapper_147.xhtml "INES Mapper 147") | [148](INES_Mapper_148.xhtml "INES Mapper 148") | [149](INES_Mapper_149.xhtml "INES Mapper 149") | [150](INES_Mapper_150.xhtml "INES Mapper 150") | [151](INES_Mapper_151.xhtml "INES Mapper 151") | [152](INES_Mapper_152.xhtml "INES Mapper 152") | [153](INES_Mapper_153.xhtml "INES Mapper 153") | [154](INES_Mapper_154.xhtml "INES Mapper 154") | [155](MMC1.xhtml "INES Mapper 155") | [156](INES_Mapper_156.xhtml "INES Mapper 156") | [157](INES_Mapper_157.xhtml "INES Mapper 157") | [158](INES_Mapper_158.xhtml "INES Mapper 158") | [159](INES_Mapper_159.xhtml "INES Mapper 159")  
[160](INES_Mapper_160.xhtml "INES Mapper 160") | [161](INES_Mapper_161.xhtml "INES Mapper 161") | [162](INES_Mapper_162.xhtml "INES Mapper 162") | [163](INES_Mapper_163.xhtml "INES Mapper 163") | [164](INES_Mapper_164.xhtml "INES Mapper 164") | [165](INES_Mapper_165.xhtml "INES Mapper 165") | [166](INES_Mapper_167.xhtml "INES Mapper 166") | [167](INES_Mapper_167.xhtml "INES Mapper 167") | [168](INES_Mapper_168.xhtml "INES Mapper 168") | [169](INES_Mapper_169.xhtml "INES Mapper 169") | [170](INES_Mapper_170.xhtml "INES Mapper 170") | [171](INES_Mapper_171.xhtml "INES Mapper 171") | [172](INES_Mapper_172.xhtml "INES Mapper 172") | [173](INES_Mapper_173.xhtml "INES Mapper 173") | [174](INES_Mapper_174.xhtml "INES Mapper 174") | [175](INES_Mapper_175.xhtml "INES Mapper 175")  
[176](INES_Mapper_176.xhtml "INES Mapper 176") | [177](INES_Mapper_177.xhtml "INES Mapper 177") | [178](INES_Mapper_178.xhtml "INES Mapper 178") | [179](INES_Mapper_179.xhtml "INES Mapper 179") | [180](INES_Mapper_180.xhtml "INES Mapper 180") | [181](INES_Mapper_181.xhtml "INES Mapper 181") | [182](INES_Mapper_114.xhtml "INES Mapper 182") | [183](INES_Mapper_183.xhtml "INES Mapper 183") | [184](INES_Mapper_184.xhtml "INES Mapper 184") | [185](CNROM.xhtml "INES Mapper 185") | [186](INES_Mapper_186.xhtml "INES Mapper 186") | [187](INES_Mapper_187.xhtml "INES Mapper 187") | [188](INES_Mapper_188.xhtml "INES Mapper 188") | [189](INES_Mapper_189.xhtml "INES Mapper 189") | [190](INES_Mapper_190.xhtml "INES Mapper 190") | [191](INES_Mapper_191.xhtml "INES Mapper 191")  
[192](INES_Mapper_192.xhtml "INES Mapper 192") | [193](INES_Mapper_193.xhtml "INES Mapper 193") | [194](INES_Mapper_194.xhtml "INES Mapper 194") | [195](INES_Mapper_195.xhtml "INES Mapper 195") | [196](INES_Mapper_196.xhtml "INES Mapper 196") | [197](INES_Mapper_197.xhtml "INES Mapper 197") | [198](INES_Mapper_198.xhtml "INES Mapper 198") | [199](INES_Mapper_199.xhtml "INES Mapper 199") | [200](INES_Mapper_200.xhtml "INES Mapper 200") | [201](INES_Mapper_201.xhtml "INES Mapper 201") | [202](INES_Mapper_202.xhtml "INES Mapper 202") | [203](INES_Mapper_203.xhtml "INES Mapper 203") | [204](INES_Mapper_204.xhtml "INES Mapper 204") | [205](INES_Mapper_205.xhtml "INES Mapper 205") | [206](INES_Mapper_206.xhtml "INES Mapper 206") | [207](INES_Mapper_207.xhtml "INES Mapper 207")  
[208](INES_Mapper_208.xhtml "INES Mapper 208") | [209](J_Y__Company_ASIC.xhtml "INES Mapper 209") | [210](INES_Mapper_210.xhtml "INES Mapper 210") | [211](J_Y__Company_ASIC.xhtml "INES Mapper 211") | [212](INES_Mapper_212.xhtml "INES Mapper 212") | [213](INES_Mapper_213.xhtml "INES Mapper 213") | [214](INES_Mapper_214.xhtml "INES Mapper 214") | [215](INES_Mapper_215.xhtml "INES Mapper 215") | [216](INES_Mapper_216.xhtml "INES Mapper 216") | [217](INES_Mapper_217.xhtml "INES Mapper 217") | [218](INES_Mapper_218.xhtml "INES Mapper 218") | [219](INES_Mapper_219.xhtml "INES Mapper 219") | [220](INES_Mapper_220.xhtml "INES Mapper 220") | [221](INES_Mapper_221.xhtml "INES Mapper 221") | [222](INES_Mapper_222.xhtml "INES Mapper 222") | [223](INES_Mapper_223.xhtml "INES Mapper 223")  
[224](INES_Mapper_224.xhtml "INES Mapper 224") | [225](INES_Mapper_225.xhtml "INES Mapper 225") | [226](INES_Mapper_226.xhtml "INES Mapper 226") | [227](INES_Mapper_227.xhtml "INES Mapper 227") | [228](INES_Mapper_228.xhtml "INES Mapper 228") | [229](INES_Mapper_229.xhtml "INES Mapper 229") | [230](INES_Mapper_230.xhtml "INES Mapper 230") | [231](INES_Mapper_231.xhtml "INES Mapper 231") | [232](INES_Mapper_232.xhtml "INES Mapper 232") | [233](INES_Mapper_233.xhtml "INES Mapper 233") | [234](INES_Mapper_234.xhtml "INES Mapper 234") | [235](INES_Mapper_235.xhtml "INES Mapper 235") | [236](INES_Mapper_236.xhtml "INES Mapper 236") | [237](INES_Mapper_237.xhtml "INES Mapper 237") | [238](INES_Mapper_238.xhtml "INES Mapper 238") | [239](https://www.nesdev.org/w/index.php?title=INES_Mapper_239&action=edit&redlink=1 "INES Mapper 239 \(page does not exist\)")  
[240](INES_Mapper_240.xhtml "INES Mapper 240") | [241](INES_Mapper_241.xhtml "INES Mapper 241") | [242](INES_Mapper_242.xhtml "INES Mapper 242") | [243](INES_Mapper_243.xhtml "INES Mapper 243") | [244](INES_Mapper_244.xhtml "INES Mapper 244") | [245](INES_Mapper_245.xhtml "INES Mapper 245") | [246](INES_Mapper_246.xhtml "INES Mapper 246") | [247](https://www.nesdev.org/w/index.php?title=INES_Mapper_247&action=edit&redlink=1 "INES Mapper 247 \(page does not exist\)") | [248](INES_Mapper_248.xhtml "INES Mapper 248") | [249](T9552.xhtml "INES Mapper 249") | [250](INES_Mapper_250.xhtml "INES Mapper 250") | [251](INES_Mapper_251.xhtml "INES Mapper 251") | [252](INES_Mapper_252.xhtml "INES Mapper 252") | [253](INES_Mapper_253.xhtml "INES Mapper 253") | [254](INES_Mapper_254.xhtml "INES Mapper 254") | [255](INES_Mapper_255.xhtml "INES Mapper 255")  
  
Known problems: 

  * Mapper 2 limited to 256k PRG. (An oversized version exists for 512k PRG: [forum thread](https://forums.nesdev.org/viewtopic.php?t=24230).)
  * Mapper 3 limited to CNROM support, excluding unlicensed oversize variants (e.g. used by Panesian games).
  * Mapper 4 does not support Startropics. (See [mapper 4 and MMC6](MMC3.xhtml#iNES_Mapper_004_and_MMC6 "MMC3").)
  * Mapper 5 (MMC5) is incomplete, and fails to run most MMC5 games.
  * Mapper 23 (VRC2/4 variants) is listed as buggy.
  * Mapper 92 (Jaleco-JF variant) is listed as buggy.
  * Mapper 95 (Namcot-3425) is listed as buggy.
  * Mapper 96 (Oeka Kids) is listed as buggy.



## Software development limitations

Aside from mapper incompatibility, there are minor differences between running NES programs on the PowerPak versus a traditional single-game cartridge. 

  * The PowerPak does not accurately simulate power-on state. Because power-on always boots the PowerPak menu, RAM and various registers will be initialized to a consistent state before any NES ROM is chosen to run. (Reset state, however, is not affected by this problem.)
  * [Open bus behavior](Open_bus_behavior.xhtml "Open bus behavior") may be different in several memory regions that are used by the PowerPak, but would not be connected on a regular cartridge. ([forum post](http://forums.nesdev.org/viewtopic.php?f=2&t=12549))



## Utilities

  * [make_sram](Make_sram.xhtml "Make sram") : a program written in Python to create PowerPak save files for all NES ROMs in a folder or on a CF card



## PowerPak development resources

  * [PowerPak Menu](PowerPak_Menu.xhtml "PowerPak Menu") \- information about the organization of the PowerPak's operating system.
  * Collection of information and photos, source code for menu and loader: <http://nespowerpak.com/>
  * Source code for CNROM example (Xilinx ISE Webpack 9.1): [powerpakdev1.zip](http://www.nespowerpak.com/powerpakdev1.zip)
  * Source code for some of Loopy's mappers (Verilog): [powerpak_loopy_src.zip](http://forums.nesdev.org/viewtopic.php?p=173302#p173302) / [powerpak_loopy.zip](http://3dscapture.com/NES/powerpak_loopy.zip)
  * Source code for NSF player: [powerpak_nsf_src.zip](http://3dscapture.com/NES/powerpak_nsf_src.zip)
  * Source code for Mapper 190 (Magic Kid GooGoo): [forum thread](https://forums.nesdev.org/viewtopic.php?p=191417)
  * Source code for Mapper 111 (GTROM): [forum thread](https://forums.nesdev.org/viewtopic.php?p=206883)



Categories: [Flash Cartridge](Category_Flash_Cartridge.xhtml)
