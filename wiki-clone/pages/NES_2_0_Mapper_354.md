# NES 2.0 Mapper 354

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_354) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_354)

NES 2.0 Mapper 354 denotes three different PCBs: 

  * **FAM250** for the _1992 巨作 250-in-1 劃面選關 鑽石巨王_ multicart (submapper 0),
  * **810139C** for the _1992 劃面選關 400-in-1 創新版_ as well as an undumped 650-in-1 multicart (also submapper 0),
  * **810331C/SCHI-24** for the _[1992 劃面選關 1050-in-1](1992______1050_in_1.xhtml "1992 劃面選關 1050-in-1")_ multicart (submapper 1).



It is very similar to [INES Mapper 015](INES_Mapper_015.xhtml "INES Mapper 015") but has an additional PRG mode for the FDS conversion of _Bubble Bobble_ as well as supporting larger PRG ROM sizes. 

# Address and Data Latch ($E000-$FFFF, write)
    
    
    Mask: $F000
    
    A~FEDC BA98 7654 3210  D~7654 3210
      -------------------    ---------
      111B .... ...A CSSS    pMPP PPPP
         |         | ||||    ||++-++++- PRG A19..A14
         |         | ||||    |+-------- Nametable mirroring
         |         | ||||    |           0: Vertical
         |         | ||||    |           1: Horizontal
         |         | ||||    +--------- PRG A13 if SSS=2/5/6, ignored otherwise
         |         | |+++-------------- PRG banking mode
         |         | |                   0/4: NROM-256 (PRG A14=CPU A14)
         |         | |                   1:   UNROM    (PRG A14..16=111 when CPU A14=1)
         |         | |                   2/6: NROM-64  (PRG A13=p)
         |         | |                   3/7: NROM-128
         |         | |                   5:   FDS Conversion:
         |         | |                        $6000-$7FFF selected as in NROM-64,
         |         | |                        $8000-$FFFF selected as in NROM-256, but with
         |         | |                                    PRG A15/A16 forced to 1
         |         | +----------------- 1=Write-protect CHR-RAM
         |         +------------------- PRG A20
         +----------------------------- PRG A21 (submapper 1 only), must be 1 on submapper 0
    

  * Power-on and reset value: All bits clear
  * On **submapper 0** , the register only responds to $F000-$FFFF writes; $8000-$EFFF writes are ignored, which is necessary for some individual games.
  * On **submapper 1** , the register responds to $E000-$FFFF writes; $8000-$DFFF writes are ignored.



# See also

PCB images and analysis of [FAM250](http://forums.nesdev.org/viewtopic.php?f=9&t=17612), [810331C/SCHI-24](https://forums.nesdev.org/viewtopic.php?t=24261)

Categories: [FDS conversion mappers](Category_FDS_conversion_mappers.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
