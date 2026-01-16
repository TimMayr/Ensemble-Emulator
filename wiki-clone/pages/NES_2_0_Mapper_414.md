# NES 2.0 Mapper 414

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_414) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_414)

**NES 2.0 Mapper 414** denotes the PCB of the _9999999合1 - 可能是世界上最強組合_ multicart. 
    
    
    Mask $8000:
    Write: A~[1.N. ...S ssss .PPM]
                |     | ||||  ||+- 1=Horizonal,
                |     | ||||  ||   0=Vertical mirroring
                |     | ||||  ++-- PRG A15..A14
                |     | ++++------ Solder pad select
                |     +----------- 1=Normal operation
                |                  0=Solder pad test
                +----------- 1=NROM-256 (PRG A14=CPU A14),
                             0=NROM-128
    
    Mask $8000:
    Write: D~[.... ..CC]  (bus conflicts)
                     ++- CHR A14..A13
    

In solder pad test mode, CPU $C000-$FFFF becomes open bus if any of the solder pads selected via ssss is connected. 

Categories: [Multicart mappers](Category_Multicart_mappers.xhtml)
