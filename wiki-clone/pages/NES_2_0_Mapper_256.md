# NES 2.0 Mapper 256

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_256) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_256)

NES 2.0 Mapper 256 is used for games using the bankswitching behavior of the [OneBus](VTxx.xhtml "VT03") Famiclones. The UNIF board names are **UNL-OneBus** , **BMC-OneBus** and **UNL-DANCE**. 

## Contents

  * 1 Registers
  * 2 Single and split PRG/CHR address space
  * 3 Bank register address and CPU opcode scrambling
  * 4 Embedded VT369 ROM



## Registers

To emulate ROM images using mapper 256, one must support bankswitching, nametable mirroring selection and IRQs using both the VTxx native registers (see the [VT02+ PRG-ROM Bankswitching](VT02__PRG_ROM_Bankswitching.xhtml "VT02+ PRG-ROM Bankswitching") and [VT02+ CHR-ROM Bankswitching](VT02__CHR_ROM_Bankswitching.xhtml "VT02+ CHR-ROM Bankswitching") articles for details) as well as the [MMC3 compatible registers](VT02__MMC3_Compatibility_Registers.xhtml "VT02+ MMC3 Compatibility Registers"). 

## Single and split PRG/CHR address space

On physical hardware, bankswitching by the VTxx console itself can only be used if the cartridge uses OneBus addressing, signalled by connecting certain pins on the cartridge connector to Vcc/GND, because only in OneBus mode are the CHR address lines repurposed as higher-order address lines for the single address space. Therefore, regular OneBus ROM images will only contain PRG ROM and no CHR ROM, and PRG ROM ($41xx) and CHR ROM ($201x) bankswitch registers apply to the same address space. 

However, ROM images converted from Waixing's WXN format will have both PRG and CHR ROM, yet use the VTxx-native bankswitch registers and/or register address mangling. These images cannot be run unchanged on physical hardware, and were intended to be run by Waixing's proprietary VTxx emulator found on several 32-bit plug-and-play consoles. For simplicity, mapper 256 is also used for such ROM images; to emulate them, simply apply the PRG ROM ($41xx) and CHR ROM ($201x) bankswitch registers to the respective separate address space. 

## Bank register address and CPU opcode scrambling

Several plug-and-play consoles use a simple protection scheme intended to prevent interchanging the ROM data between different consoles. The protection scheme works by simply swapping the addresses of the bankswitch registers. Some patterns only change the VTxx-native register addresses, others apply to the MMC3 compatibility registers as well. 

Other plug-and-play consoles, in particular those by Jungletac, scramble CPU opcode bytes (and only those) by swapping bits. These consoles power-on with opcode scrambling enabled, but allow it to be disabled --- for example, to run a pirated licensed NES game without modifying it --- by writing to register **411C** (submappers 11, 12 and 14): 
    
    
    D~7654 3210
      ---------
      .E.. ..F.
       |     +-- Select primary CPU opcode encryption (submapper 11 only)
       |          0: disabled
       |          1: enabled (default at power-on)
       +-------- Select CPU opcode encryption (submapper 12+14 only)
                  0: disabled
                  1: enabled (default at power-on)
                 Select secondary CPU opcode encryption (submapper 11 only)
                  0: disabled (default at power-on)
                  1: enabled
    

or **$4169** (submappers 13 and 15): 
    
    
    D~7654 3210
      ---------
      .... ...E
              +- Select CPU opcode encryption (submappers 13+15 only)
                  0: enabled (default at power-on)
                  1: disabled
    

[Submappers](NES_2_0_submappers.xhtml#NES_2.0_Mapper_256 "NES 2.0 submappers") have been allocated to denote the type of protection scheme: 

Submapper # | Name | PPU bank affected by ... | CPU bank affected by ... | CPU opcode bytes   
---|---|---|---|---  
$2012 | $2013 | $2014 | $2015 | $2016 | $2017 | $8000.0 | $8000.1 | $8000.2 | $8000.3 | $8000.4 | $8000.5 | $4107 | $4108 | $8000.6 | $8000.7   
0 | Normal | $1000 | $1400 | $1800 | $1C00 | $0000 | $0800 | $0000 | $0800 | $1000 | $1400 | $1800 | $1C00 | $8000 | $A000 | $8000 | $A000 | none   
1 | Waixing VT03 | $1400 | $1000 | $0800 | $0000 | $1C00 | $1800 | $1C00 | $1800 | $1400 | $1000 | $0800 | $0000 | $8000 | $A000 | $8000 | $A000 | none   
2 | Power Joy Supermax | $1000 | $1400 | $1800 | $1C00 | $0000 | $0800 | $0000 | $0800 | $1000 | $1400 | $1800 | $1C00 | $A000 | $8000 | $A000 | $8000 | none   
3 | Zechess/Hummer Team | $0800 | $0000 | $1C00 | $1800 | $1000 | $1400 | $0000 | $0800 | $1000 | $1400 | $1800 | $1C00 | $8000 | $A000 | $8000 | $A000 | none   
4 | Sports Game 69-in-1 | $1800 | $0800 | $1000 | $0000 | $1C00 | $1400 | $0000 | $0800 | $1000 | $1400 | $1800 | $1C00 | $8000 | $A000 | $8000 | $A000 | none   
5 | Waixing VT02 | $1400 | $1000 | $0800 | $0000 | $0000 | $0800 | $0000 | $0800 | $1000 | $1400 | $1800 | $1C00 | $8000 | $A000 | $8000 | $A000 | none   
11 | Vibes | $1000 | $1400 | $1800 | $1C00 | $0000 | $0800 | $0000 | $0800 | $1000 | $1400 | $1800 | $1C00 | $8000 | $A000 | $8000 | $A000  | D7<->D6, D1<->D2 swapped, switched via $411C.6; D5<->D4 swapped, switched via $411C.1   
12 | Cheertone | $1000 | $1400 | $1800 | $1C00 | $0000 | $0800 | $0000 | $0800 | $1000 | $1400 | $1800 | $1C00 | $8000 | $A000 | $8000 | $A000  | D7<->D6, D1<->D2 swapped, switched via $411C.6   
13 | Cube Tech | $1000 | $1400 | $1800 | $1C00 | $0000 | $0800 | $0000 | $0800 | $1000 | $1400 | $1800 | $1C00 | $8000 | $A000 | $8000 | $A000  | D1 and D4 swapped, switched via $4169   
14 | Karaoto | $1000 | $1400 | $1800 | $1C00 | $0000 | $0800 | $0000 | $0800 | $1000 | $1400 | $1800 | $1C00 | $8000 | $A000 | $8000 | $A000  | D6 and D7 swapped, switched via $411C   
15 | Jungletac | $1000 | $1400 | $1800 | $1C00 | $0000 | $0800 | $0000 | $0800 | $1000 | $1400 | $1800 | $1C00 | $8000 | $A000 | $8000 | $A000  | D5 and D6 swapped, switched via $4169   
  
## Embedded VT369 ROM

The VT369 variant of the OneBus famiclones has 4 KiB of ROM embedded into the NoaC glob itself. These 4 KiB are mapped to $1000-$1FFF of the main CPU's address space and to $4000-$4FFF of the sound CPU's address space, and contain initialization code and a sound CPU program. They are included as NES 2.0 Misc ROM. 

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [NES 2.0 mappers with submappers](Category_NES_2_0_mappers_with_submappers.xhtml)
