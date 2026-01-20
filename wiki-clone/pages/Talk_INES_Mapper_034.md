# Talk:INES Mapper 034

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_034) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_034)

## Emulating both

Isn't it possible to emulate this mapper as a single "mapper" (that is without distinguishing between BNROM and NINA) with the following setup : 

  * 1 PRG bankswitching register at $7ffd and $8000-$ffff
  * 2 CHR bankswitching registers at $7ffe and $7fff, initialized to 0 and 1 respectively on powerup.



That way, the BNROM game starts up with the right CHR banks switched in, bus conflicts aren't emulated, but aren't required for emulation anyways. The NINA game should work well as long as it doesn't write to $8000-$ffff (I fail to see why it would do this). 

The only problem is that this doesn't support BNROM + SRAM extension, but no game was ever made using this anyways. [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 14:20, 17 March 2014 (MDT) 

    I'm not familiar with the two games to know for sure they don't write to conflicting places, but as the mappers are described, yes I believe that is fine as a mapper that supports both. If you'd like to check them both with a debugger to verify that this is the case, it might help resolve this question. If it's safe, we should probably add to that second paragraph in the lead section to mention this. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 19:20, 17 March 2014 (MDT)
    The PowerPak emulates both at once, as do many emulators that don't implement NES 2.0. But the Holy Diver Batman multi-mapper test ROM is designed to test BxROM + [PRG RAM circuit](PRG_RAM_circuit.xhtml "PRG RAM circuit"), and the overlaid NINA logic causes it to fail on PowerPak. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 22:32, 17 March 2014 (MDT)

Darkseed is mapper 164 originally, correct, though there is a mapper hack for mapper 241 that is incorrectly set to 34 in GoodNES. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 11:13, 4 July 2022 (UTC) 
