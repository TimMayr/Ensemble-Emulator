# INES Mapper 173

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_173) | View [other pages](Special_AllPages.xhtml#INES_Mapper_173)

INES Mapper 173 is used to denote an unmarked board used by the original Idea-Tek releases (cartridge codes ET-xx) of several of their games: 

  * _Puzzle_ (ET.01)
  * _小瑪琍_ (Xiǎo Mǎlí) (Nei-Hu and Idea-Tek releases, with identical cartridge shell and label) (ET.02)
  * _F-15 City War_ (ET.03)
  * _撲克精靈_ (Poker Jīnglíng) (ET.04)
  * _戰國四川省_ (Zhànguó Sìchuān Shěng) (ET.05), developed by Computer & Entertainment



## Contents

  * 1 Banks
  * 2 Registers
  * 3 Notes and Errata
  * 4 Similar Mappers
  * 5 See also



## Banks

  * PPU $0000-$1FFF: 8 KB switchable CHR ROM bank



## Registers

Mapper 173 uses a custom IC (real number [05-00002-010](TXC_05_00002_010_pinout.xhtml "TXC 05-00002-010 pinout"), fake marking "ITC20V8-10LP", falsely suggesting a particular type of programmable logic) serving as a latch, adder and inverter. There are six registers: "P", "R", and Output (three bits each); "S", inCrement, and and inVert (one bit each). 
    
    
    Mask: $E100
     read $4100: [xxxx SRRR]
                  |||| ||||
                  |||| ++++- Copy internal registers 'RRR' and**'S' XOR 'V'** to data bus.
                  ++++------ open bus
    Mask: $E103
     write $4100: If Increment is set, internal register 'RRR' <- 'RRR'+1
                  Otherwise, if Invert is clear, copy internal register 'PPP' to 'RRR'
                             if Invert is set, copy '~PPP' to 'RRR'
                  'S' is not changed at all.
     write $4101: [.... ...V] - Invert Mode. This value is immediately inverted and used as CHR A14.
     write $4102: [.... SPPP] - Copy data bus to internal registers 'S' and 'PPP'.
                                'S' can be read back immediately; 'PPP' must be copied using $4100 first.
     write $4103: [.... ...C] - Increment Mode
    Mask: $8000
     write $8000: copy bottom two bits of internal register 'RRR' to CHR A15 and CHR A13 banking pins, in order.
    

In Mapper 173, bit 0 of the 8 KiB CHR ROM bank number (CHR A13) comes from Output bit 0, while bit 1 of the 8 KiB CHR ROM bank number (CHR A14) is the inverted value of the Invert register: 
    
    
    8 KiB CHR-ROM bank number := (Output &1) | (~Invert <<1);
    

Games therefore set Invert (writing $FF or $01 to $4101) to choose CHR banks 0 and 1, and clear Invert (writing $00 to $4101) to choose CHR banks 2 and 3. 

## Notes and Errata

  * _小瑪琍_ (Xiǎo Mǎlí) uses the Mapper 173 circuit board but replaces the 32 KiB CHR-ROM chip with a SGS M2764A 8 KiB UVEPROM. Its pin 27 function is /PGM instead of the normal A14. Setting Invert on this board just disables CHR-ROM. The game's protection involves reading from $4100 and disabling the PPU if the result does not match the expected value.
  * _麻将方块 (Mahjong Block_) (TXC re-release, headerless CRC32 0ACFC3CD) is commonly set to mapper 173, but since it expects Output bit 1 to select CHR A14, it actually uses [INES Mapper 132](INES_Mapper_132.xhtml "INES Mapper 132").
  * _撲克精靈 (Poker Jīnglíng)_ writes the actual 8 KiB bank number to $8000 to copy Register into Output, making it potentially [CNROM](CNROM.xhtml "CNROM")-compatible. It will not run as Mapper 3 on emulators that emulate bus conflicts (such as FCEUX) though, nor will it run on an actual Nintendo CNROM board, for the same reason.
  * _Rad Racket - Deluxe Tennis II_ (TXC release, MGC-011) uses a TXC 01-22111-100 ([INES Mapper 132](INES_Mapper_132.xhtml "INES Mapper 132")) board but sets/clears Invert for 8 KiB CHR-ROM banks 0 and 1/2 and 3, making it compatible with Mapper 173 as well.
  * 戰國四川省 (Zhànguó Sìchuān Shěng, original version of AVE's Tiles of Fate) is set to Mapper 132 in GoodNES 3.23b. That ROM image is actually a mapper hack with the PRG-ROM code unmodified but the CHR-ROM banks rearranged to work as Mapper 132; the correct mapper is INES Mapper 173.



## Similar Mappers

  * INES Mapper 173 is similar to [INES Mapper 132](INES_Mapper_132.xhtml "INES Mapper 132") with only 32 KiB PRG-ROM except that Mapper 132 connects CHR A14 to Output bit 1 rather than the inverse of the Invert bit.



## See also

[PCB image](http://forums.nesdev.org/viewtopic.php?f=3&t=15961&start=15#p210297)
