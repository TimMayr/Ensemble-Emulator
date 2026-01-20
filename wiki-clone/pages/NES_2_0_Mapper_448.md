# NES 2.0 Mapper 448

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_448) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_448)

**NES 2.0 Mapper 448** denotes the [VRC4](VRC2_and_VRC4.xhtml "VRC4")-based **830768C** multicart circuit board used by a _Super 6-in-1_ multicart. The VRC4 clone's A0/A1 inputs are connected to CPU A2/A3, and its PRG-RAM chip-enable signal drives an outer bank register. The VRC4 is otherwise mainly used for its IRQ and mirroring selection functionalities; discrete circuits implement UNROM/UOROM/AOROM-like PRG banking with 8 KiB of unbanked CHR-RAM. 

## Outer Bank Register ($6000-$7FFF, write)
    
    
    A~[0110 .... .... BUPP]
                      |+++- PRG A19..A17
                      |+--- Fixed bank when B=0
                      |      0: 7 (UNROM)
                      |      1: 15 (UOROM)
                      +---- PRG mode
                             0: UNROM/UOROM
                             1: AOROM
    

In UNROM/UOROM mode, the VRC4's PRG register at $8000 selects the 16 KiB PRG bank at $8000-$BFFF, and mirroring depends on the VRC4's normal mirroring register. In AOROM mode, the VRC4's PRG bank and mirroring is ignored, and the entire $8000-$FFFF address range functions like [INES Mapper 007](AxROM.xhtml "INES Mapper 007") with respect to PRG banking and single-screen mirroring, subject only to PRG A19 and A18 set via the outer bank register. 

Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
