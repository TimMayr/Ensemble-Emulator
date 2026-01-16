# Talk:NROM

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ANROM) | View [other pages](Special_AllPages.xhtml#Talk_NROM)

## FamicomHDL

I have made some of FamicomHDL already and I think the following code seems to work for implementing NROM-128 with vertical mirroring: 
    
    
    import Language.FamicomHDL;
    import Language.FamicomHDL.Cartridge;
    import Language.FamicomHDL.Memory;
    nrom :: Mapper ();
    nrom = do {
      prgrom <- makeROM 0;
      chrrom <- makeROM 16384;
      connects $ zip cpuAddress (take 14 $ addressPins prgrom);
      connects $ zip cpuData (dataPins prgrom);
      connects $ zip ppuAddress (take 13 $ addressPins chrrom);
      connects $ zip ppuData (dataPins chrrom);
      connect romSelect (chipEnablePin prgrom);
      connect ppuA13inv ciramEnable;
      connect ciramA10 (ppuAddress !! 10);
      connect (chipEnablePin chrrom) (ppuAddress !! 13);
    };
    

Perhaps commands should be added for some of these things make shortcut. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 22:38, 28 October 2012 (MDT) 

## CIC Diodes?

Some NES-NROM boards have slots for diodes too. —[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 01:40, 5 May 2014 (MDT) 

    I can't think of a context in which diodes could be useful on an NROM board... do you have a photo somewhere (e.g. NesCartDB) ? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 03:20, 5 May 2014 (MDT) 

    I also can't think of where they could be useful (not that they are useful on CNROM either...) but [NES-NROM-xxx-02](http://bootgod.dyndns.org:7777/profile.php?id=21) and NES-NROM-xxx-03 boards have slots for 2 diodes next to the CIC. I don't have any game using one of those boards so I can't check where they connect, but you can see the slot clearly on the NesCartDB.—[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 04:03, 5 May 2014 (MDT) 

    Those diodes were intended to go from the CIC +RST and CIC CLK lines to vcc, providing overvoltage protection for the key CIC. But I have no idea under which situation that would have been useful.—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:35, 5 May 2014 (MDT)

Oh thank you ! I don't know why but I thought about the CNROM diodes when I saw them (since they're close to the CHR ROM as well). My bad, thank you very much for clarifying this.~~128.178.195.59~~ 13:25, 5 May 2014 (MDT) 

## Suggested reforms for this page

There is some thing I would belive would change positively. The page is about simultaneously mapper #0 and NROM, the former being wider than the later. Since I don't know if anyone else agree I won't change anything myself (since I bet 99% that any changes will be reverted immediately and everyone disagreeing), so this is just suggestions: 

\- Mapper #0 is defined as any board that lacks a mapper and bankswitching capability 

\- It is written explicitly that NROM is just one board that implements "no mapper", but other boards (RROM, Family basic, 3rd party boards, etc, etc....) also implement the same functionality 

\- It is written explicitly in the intro that there is no registers, and the "registers" paragraph is deleted. There is no need for that explanation of "poor man's CNROM", because it is non-technical, and anyone reading info about the very basic of PPU's way of working and $2000 register already knowns that. Do not assume the reader is a baby. 

\- PRG RAM capability should be left open for anything 8k or less. There is nothing preventing you to implement NROM with 8k PRG RAM while still following iNES mapper #0 specifications. More than 8k is possible but it would not follow iNES mapper #0 anymore and would be a sparate "mapper" (despite being no mapper, oh the irony). I don't know how to word that correctly. In all cases, I believe that NROM + 8k PRG RAM is a valid configuration, as well as less PRG RAM and not relying on mirroring. [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 07:59, 27 May 2015 (MDT) 
