# NES 2.0 Mapper 291

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_291) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_291)

NES 2.0 Mapper 291 is used for at least two Kasheng 2-in-1 multicarts both involving the game _Mortal Kombat 6_ , a title screen hack of _餓狼伝説 Special_ (Garou Densetsu Special) and another game. Nestopia Plus! for some inexplicable reason reuses [INES Mapper 047](INES_Mapper_047.xhtml "INES Mapper 047") for them. It's an MMC3 clone with an outer bank register. 

## Outer Bank Register ($6000), write
    
    
    Mask: Probably $E000
    
    7654 3210
    ---------
    ?OM. .PP.
    |||   ++-- Select 32 KiB Inner PRG-ROM bank if Bit 5=1
    ||+------- Select PRG-ROM banking mode
    ||          0: Use MMC3 inner PRG-ROM bank (128 KiB)
    ||          1: Select 32 KiB inner PRG-ROM bank using $6000 Bits 1 and 2
    |+-------- Select 128 KiB outer PRG and 256 KiB outer CHR-ROM bank
    |           0: First
    |           1: Second
    +--------- Unknown function
    

The Outer Bank Register responds even when the MMC3 clone's WRAM bit is clear. 

## Errata

The _2-in-1 (Mortal Kombat 6, Samurai Spirits)_ ROM image in GoodNES 3.23b has the outer 128 KiB and 256 KiB CHR-ROM banks swapped. To run that image unmodified using the above description, the value written to $6000 would have to be XORed with 0x40. 

Categories: [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
