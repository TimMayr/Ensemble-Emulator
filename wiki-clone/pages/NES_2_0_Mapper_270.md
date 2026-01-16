# NES 2.0 Mapper 270

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_270) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_270)

**NES 2.0 Mapper 270** denotes [OneBus](NES_2_0_Mapper_256.xhtml "NES 2.0 Mapper 256") console multicarts that use the consoles' universal input/output (UIO) register $412C to bankswitch higher-order PRG address lines or several PRG chips, and select CHR-RAM via $4242. 

## Contents

  * 1 Submapper 0: Combination
  * 2 Submapper 1: Game Prince RS-16
    * 2.1 Outer PRG Bank ($412C, write)
  * 3 Submapper 2: Family Pocket 638-in-1
    * 3.1 Outer PRG Bank ($412C, write)
    * 3.2 Jumper Cartridge Detection ($412C, read)
    * 3.3 CHR-RAM enable ($4242, write)
  * 4 Submapper 3: Bittboy 300-in-1
    * 4.1 Outer PRG Bank ($412C, write)
  * 5 Notes



# Submapper 0: Combination

Responds to all bank bits of submappers 1-3 at the same time. This is permissible, since the only bit that is used by several submappers has the same function in each of them. 

# Submapper 1: Game Prince RS-16

## Outer PRG Bank ($412C, write)
    
    
    D~7654 3210
      ---------
      .... ..A.
             +-- PRG/CHR A24
    

# Submapper 2: Family Pocket 638-in-1

## Outer PRG Bank ($412C, write)
    
    
    D~7654 3210
      ---------
      .... ..AB
             |+- PRG/CHR A25
             +-- PRG/CHR A24
    

## Jumper Cartridge Detection ($412C, read)
    
    
    D~7654 3210
      ---------
      .... B...
           +---- 0: Jumper cartridge not inserted
                 1: Jumper cartridge inserted
    

## CHR-RAM enable ($4242, write)
    
    
    D~7654 3210
      ---------
      .... ...C
              +- 0: Use normal [OneBus CHR banking](VT02__CHR_ROM_Bankswitching.xhtml "VT02+ CHR-ROM Bankswitching")
                 1: Use 8 KiB of unbanked CHR-RAM
    

# Submapper 3: Bittboy 300-in-1

## Outer PRG Bank ($412C, write)
    
    
    D~7654 3210
      ---------
      .... .A..
            +--- PRG/CHR A24
    

# Notes

  * Any $412C bit can only be read from if the corresponding bit in register $412B (UIO input/output selector) is set to "0" (input).
  * Any $412C bit can only be written to if the corresponding bit in register $412B (UIO input/output selector) is set to "1" (output).



Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml), [NES 2.0 mappers with submappers](Category_NES_2_0_mappers_with_submappers.xhtml)
