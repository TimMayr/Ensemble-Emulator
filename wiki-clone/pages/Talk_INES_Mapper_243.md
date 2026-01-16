# Talk:INES Mapper 243

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_243) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_243)

## Solder Pad

What specifies the solder pad? (both in hardware and for an emulator) I don't see a pin on the pinout that could relay it...? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 14:43, 7 December 2019 (MST) 

    That only applies to Shogi Gakuen (cartridge code SA-015), mapper 150. Until a PCB image of that particular game is obtained, I cannot answer that question. The solder pad obviously does not exist on the SA-020A PCB, so I have removed its (copy-pasted) description from the mapper 243. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 16:00, 7 December 2019 (MST)
    Take a look at this solder pad on the SA-015 PCB: <https://symphoniae.com/nrs/nesdev/SA-015-SolderPad.jpg>
    It selects whether ASIC pin 14 is connected to CPU D2 or to Vcc. At first, that may seem irrelevant, until you debug Shogi Gakuen's pad-reading code fully (starting at CPU offset $807B). The game first writes $00 to all registers, then writes $FF to register $02, then reads back register $06 and checks if the result is zero. If the solder pad connects ASIC pin 14 to CPU D2, then writing $FF to register $02 will cause register $06 to read back as $00 (actually $40 because D3-D7 are open bus) since it was cleared before. If the solder pad connects Vcc to CPU D2, the writing $FF to register $02 will actually write to register $06 and thus cause it to read back as $FF (or $43, since D3-D7 are always open bus, and D2 is not connected to the ASIC either in that solder pad configuration). I had not noticed this before because I had only looked at the final register $06 read, thinking I had verified it to match what emulators are already doing. Correct would be to state that both the index and all registers are readable, and the solder pad selects whether D2 is connected to the ASIC pin 14 or Vcc is. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 04:37, 8 December 2019 (MST) 

    Interesting. Does that imply that mapper 150 needs submappers? It looks like this modification would seriously hinder the nametable layout register. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 11:25, 8 December 2019 (MST) 

    I implement that solder pad like a DIP switch with a menu that opens when the ROM is loaded, not as a submapper, because Shogi Gakuen uses it to select a title screen variant. It's true that in the ASIC pin14=Vcc position, the game would be limited to the L- and single-screen mirroring types, the former being what the sole game to have that pad configuration uses anyway. Unless R7.0 does something mirroring-related after all, like on the Sachen 8259, which I need to check along with your question about L-shaped mirroring. I did verify before that R7.0 has no influence on CHR banking. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 11:47, 8 December 2019 (MST)

## L-shaped mirroring layout

Old documentation said that L-shaped mirroring was OR2(PPU A10,PPU A11) (i.e. top left is unique). In contrast, the current documentation seems to say it's OR2(NOT(PPU A10),PPU A11) (i.e. top right is unique). Is this a typo, my misinterpretation, or a change in documentation? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 15:53, 7 December 2019 (MST) 

    I need to check again. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 16:00, 7 December 2019 (MST)
    I now had the opportunity to test mirroring again. I first set register $07 to $05, wrote $30 to $2000 and $31 to $2C00, then dumped the $2000-$2FFF region every time after writing $00,$01,...,$07 to register $07. Resulting banks:
    
    
    R7=0: 0,0,0,1
    R7=1: 0,0,0,1
    R7=2: 0,0,1,1
    R7=3: 0,0,1,1
    R7=4: 0,1,0,1
    R7=5: 0,1,0,1
    R7=6: 1,1,1,1
    R7=7: 1,1,1,1
    

    This confirms that R7.0 has no effect on mirroring either, so there is not "simple" bit like on the Sachen 8259. I am not sure what to call R7=0/1, or what the underlying bitwise logic would be. Note that I did not have the ability to measure CIRAM A10 directly, so if R7=5 (the setting I used when filling NTRAM) is actually 1,0,1,0 instead of 0,1,0,1 then my results will be inverted. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 13:38, 11 December 2019 (MST)
    
    
    Mirroring value (R7 SHR 1) =0: CIRAM A10=PA10 AND PA11
    Mirroring value (R7 SHR 1) =1: CIRAM A10=PA11 ("horizontal")
    Mirroring value (R7 SHR 1) =2: CIRAM A10=PA10 ("vertical")
    Mirroring value (R7 SHR 1) =3: CIRAM A10=Vcc ("one screen, page 1")
    

    Correct? [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 13:48, 11 December 2019 (MST) 

    Yup, that's AND2(PPU A10,PPU A11). Wacky that it's so different from previous documentation. Implementation is most likely a single [AND-OR gate](https://en.wikipedia.org/wiki/AND-OR-Invert "wikipedia:AND-OR-Invert"). —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 13:51, 11 December 2019 (MST)

## Register readability

Given that you discovered that both the address ($4100) and data ports ($4101) were readable, are all eight registers fully implemented for read-back? In other words, are there actually 27 bits of state here? Or are only the 11 bits that are explicitly connected externally implemented at all? (Or somewhere in-between?) —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 13:16, 13 December 2019 (MST) 

    Actually, I _assumed_ that the index was readable based on watching Shogi Gakuen, but now I recognize that the reads that I saw are actually dummy reads from the indexed register write, and that the game does not depend on the index being readable. I was certain that the registers must be readable because Shogi Gakuen depends on that.
    In any case, I wrote a CopyNES dumping script to test this, though I am not sure whether I am testing correctly. [Here is the CopyNES dumping script](https://symphoniae.com/nrs/nesdev/243read.7z) that I am using to test. allreadable.nes is the result it produces in my CopyNES emulator if both index and all registers were readable with all three bits each. 243read.nes is the result with an actual cartridge in a CopyNES. I interpret this actual result as the index not being readable after all, returning open bus instead, while all registers being readable with all three bits each, indicating 27 bits of state. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 15:22, 13 December 2019 (MST) 

    I agree with your test implementation and your conclusion. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 18:09, 13 December 2019 (MST) 

    Quite wasteful, implementing 27 state bits when only 11 are actually needed. I wonder what they were thinking. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 02:10, 14 December 2019 (MST) 

    I wonder if it's like the multicarts that sometimes had a 74'670... —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:38, 14 December 2019 (MST)
