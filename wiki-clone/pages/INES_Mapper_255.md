# INES Mapper 255

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_255) | View [other pages](Special_AllPages.xhtml#INES_Mapper_255)

iNES Mapper 255 is apparently assigned to the _110-in-1_ multicart. 
    
    
     A~[1BMZ PPPP  PpCC CCCC]
         ||| ||||  |||| ||||
         +-----------++-++++ - Select 8 KiB CHR at PPU $0000
         ||| ||||  ||
         +---++++--++------- - Select 16 KiB PRG at CPU $8000 and $C000 if Z=1
         +---++++--+-------- - Select 32 KiB PRG at CPU $8000 if Z=0
          |+---------------- - Select PRG bank size: 0-32K 1-16K
          +----------------- - Nametable mirroring: 0-PPUA10 ("vertical") 1-PPUA11 ("horizontal")
    

Hardware on the board includes 

  * four 512 KiB ROMs for PRG
  * two 512 KiB ROMs for CHR
  * two 74'273 8-bit latches
  * [74139](74139.xhtml "74139")
  * PAL16L8 emulating a [74153](74153.xhtml "74153")
  * an unpopulated footprint for a 74'670



The two latches' reset inputs are tied to 5V, so the banking register boots with random contents. 

  
This looks like a duplicate of [iNES Mapper 225](INES_Mapper_225.xhtml "INES Mapper 225"). It's not clear why Nestopia implemented it twice. 

See also: <https://forums.nesdev.org/viewtopic.php?p=209732#p209732>

Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
