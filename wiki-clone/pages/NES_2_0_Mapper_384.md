# NES 2.0 Mapper 384

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_384) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_384)

**NES 2.0 Mapper 384** denotes the **L1A16** circuit board, used on a _4-in-1_ multicart containing _Crisis Force_ , among other games. It uses a [VRC4](VRC2_and_VRC4.xhtml "VRC4") clone in the VRC4e configuration (VRC4 A0=CPU A2, VRC4 A1=CPU A3) and has 2 KiB of WRAM, mirrored once, in the CPU $6000-$6FFF area. The outer bank register overlays this area and contains a Lock bit to prevent Crisis Force's WRAM writes from changing the outer bank. 

## Outer Bank Register ($6800-$6FFF, write)
    
    
    Mask: $F800
    
    D~7654 3210
      ---------
      .... LbBB
           ||++- PRG/CHR A17/A18 (128 KiB outer bank)
           |+--- unused, but could be PRG/CHR A19 on a larger multicart
           +---- 1: Lock outer bank
    
    Power-on value: $00
    

Categories: [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml), [Mappers with single-screen mirroring](Category_Mappers_with_single_screen_mirroring.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
