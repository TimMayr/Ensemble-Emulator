# INES Mapper 116

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_116) | View [other pages](Special_AllPages.xhtml#INES_Mapper_116)

iNES Mapper 116 is used for the SOMARI-P board bearing the **Huang-1** or **Huang-2** ASICs and a PAL, used by releases from Gouder, also known as 哥德 (Gēdé): 

  * **Huang-1** chip: 
    * _A.V. 美少女戰士 Girl Fighting_ (A.V. Měi Shàonǚ Zhànshì)
    * _餓狼伝説 Special_ (Garō Densetsu Special, original Gouder release only)
    * _Kart Fighter_
    * _Somari the Adventurer_ (original Gouder and Kǎshèng releases)
  * **Huang-2** chip: 
    * _A.V. 究極麻將 II_ (A.V. Jiūjí Májiàng, original Gouder release only)



  
This mapper can simulate three different ASICs: the Nintendo [MMC1](MMC1.xhtml "MMC1"), [MMC3](MMC3.xhtml "MMC3") and Konami [VRC2](VRC2_and_VRC4.xhtml "VRC2") (A0/A1, i.e. VRC2b). All three simulated chips have their separate registers, so that when switching between modes using the supervisor register, the entire mapper state changes including PRG/CHR ROM switchable and fixed banks as well as IRQ state. 

## Supervisor Register ($4100, write)
    
    
    Mask: $E100
    
    D~7654 3210
      ---------
      .... .CMM
            |++- Mapper mode
            |     0: VRC2
            |     1: MMC3
            |     2: MMC1
            |     3: same as 2
            +--- Outer 256 KiB CHR-ROM bank (CHR A18)
    

## Notes

  * The **Huang-2** chip differs from the **Huang-1** chip in that the MMC1 PRG-ROM bank register is shifted by one bit to the left compared to what one would write to a normal MMC1, or to the Huang-1 in MMC1 mode. No submapper has been proposed yet for this behavior. In the meantime, the one game relying on this can be heuristically detected by having PRG-ROM and CHR-ROM sizes each of only 128 KiB.
  * When switching _to_ MMC1 mode, the serial shift register is reset. _Garō Densetsu Special_ relies on this. Supposedly, only the -W configuration of the PAL actually does this, though no game seems to be adversely affected by always emulating this behavior.
  * The VRC2 CHR-ROM registers are initialized on power-up to $FF. The original release of _Somari the Adventurer_ relies on this.
  * _Garō Densetsu Special_ relies on being able to write to [mirrors](Mirroring.xhtml#Memory_Mirroring "Mirroring") of $8xxx, $9xxx, and $Axxx in VRC2 mode.
  * The Kǎshèng rerelease of _Garō Densetsu Special_ uses [INES Mapper 115](INES_Mapper_115.xhtml "INES Mapper 115") instead.
  * The Kǎshèng rerelease of _A.V. 究極麻將 II_ uses [INES Mapper 115](INES_Mapper_115.xhtml "INES Mapper 115") instead.



With a iNES 2.0 Header Sub Mapper 3 sets Mapper 116 as a Reset Based Mapper that will switch games on a Hard Reset this however is current only used for one known 5-in-1 Multi-Cart that has 5 MMC3 based Mario Hacks. (SL-FC5-1) Mario 5-in-1 

## Similar mappers

  * [INES Mapper 014](INES_Mapper_014.xhtml "INES Mapper 014") and [INES Mapper 238](INES_Mapper_238.xhtml "INES Mapper 238") uses the same ASIC but with the PAL programmed differently to respond in other ways to other addresses.



Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers using $4020-$5FFF](Category_Mappers_using__4020__5FFF.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multi-ASIC mappers](Category_Multi_ASIC_mappers.xhtml)
