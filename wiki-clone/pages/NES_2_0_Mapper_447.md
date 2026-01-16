# NES 2.0 Mapper 447

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_447) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_447)

**NES 2.0 Mapper 447** denotes the [VRC4](VRC2_and_VRC4.xhtml "VRC4")-based **KL-06** multicart circuit board used by the _1993 New 860-in-1 Over-Valued Golden Version Games_ multicart. The VRC4 clone's A0/A1 inputs are connected to CPU A2/A3, and its PRG-RAM chip-enable signal drives an outer bank register. 

## Outer Bank Register ($6000-$7FFF, write)
    
    
    A~[0110 .... .... .NBB]
                       |++- PRG/CHR A18..A17
                       ||+- 1=Lock outer bank register
                       |+-- 1=NROM-128 mode if N=1, 0=NROM-256 mode if N=1
                       +--- 1=NROM mode
    

NROM mode causes the VRC4's CPU A14 line to be held low, making the VRC4's two PRG registers apply to both $8000/$A000 and $C000/$E000. NROM-256 mode additionally replaces PRG A14 with CPU A14. The 'Lock' functionality is needed because the game that is active when PRG A17=1 (_Crisis Force_) uses 2 KiB of PRG-RAM that overlap the outer bank register. 

Categories: [Multicart mappers](Category_Multicart_mappers.xhtml)
