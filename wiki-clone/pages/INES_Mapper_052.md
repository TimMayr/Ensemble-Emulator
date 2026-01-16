# INES Mapper 052

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_052) | View [other pages](Special_AllPages.xhtml#INES_Mapper_052)

**iNES Mapper 052** denotes the **Realtec 8213** and similar [MMC3](MMC3.xhtml "MMC3")-based multicart circuit boards. 

## Outer Bank Register ($6000-$7FFF, write)
    
    
    D~[LTCc SBPp]
       |||| ||++-- PRG A18..A17
       |||| |+---- PRG/CHR A19
       |||| +----- PRG A17 mode: 0=from MMC3, 1=from p
       ||++------- CHR A18..A17
       |+--------- CHR A17 mode: 0=from MMC3, 1=from c
       +---------- 1=Lock Outer Bank register until next reset
    Value on reset: $00
    

The MMC3's WRAM interface must be enabled and writeable in [MMC3 register $A001](MMC3.xhtml#PRG_RAM_protect_\(%24A001-%24BFFF,_odd\) "MMC3"). The Outer Bank Register overlaps any actual PRG RAM that may be present. 

## Mixed CHR ROM/CHR RAM variants

Nestopia Plus! has defined **Submapper 13** to denote a PCB variant with 512 KiB of each PRG ROM and CHR ROM that can switch between regular CHR ROM and 8 KiB unbanked CHR RAM. CHR RAM is selected when PRG A17 and PRG A18 are both =1. 

Nestopia Plus! further uses Submapper 13 to denote the _(AB-128) Well 8-in-1_ multicart with 1 MiB of each PRG ROM and CHR ROM, which is incompatible to the above definition, and which is hereby reassigned to **Submapper 14**. Its actual PCB name is **AB892** , and its Outer Bank Register has the following form: 
    
    
    D~[LTRc SBBp]
       |||| |||+-- PRG A18..A17
       |||| |++--- PRG/CHR A19..A18
       |||| +----- PRG A17 mode: 0=from MMC3, 1=from p
       |||+------- CHR A17
       ||+-------- 0=CHR-ROM, 1=CHR RAM
       |+--------- CHR A17 mode: 0=from MMC3, 1=from c
       +---------- 1=Lock Outer Bank register until next reset
    

At least one cart (AB-134) re-uses this PCB but leaves the CHR RAM spot unpopulated, still using the combined bit for PRG/CHR A18, but rendering the "R" bit 5 meaningless. The correct NES 2.0 header for this would be submapper 14 but with zero CHR RAM. 

## See also

  * ["Mario Party" 7 in 1](http://nesdev.org/Mari7in1.txt) by The Mad Dumper.
  * [Forum thread](http://forums.nesdev.org/viewtopic.php?f=9&t=10703) about mapper 052 by FARID



Categories: [INES Mappers](Category_INES_Mappers.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
