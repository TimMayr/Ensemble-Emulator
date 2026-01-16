# NES 2.0 Mapper 383

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_383) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_383)

NES 2.0 Mapper 383 denotes J.Y. Company's **YY840708C** circuit board. It consists of an AX5202P MMC3 clone together with a 16V8 PAL. It is used for only one known multicart: 

  * _1995 Soccer 6-in-1 足球小将專輯_ (JY-014)



The PAL maintains four signals for bankswitching and responds both to reads and writes: 

  * **PAL PRG A15** : Set by writing to CPU $8000-$FFFF.
  * **PAL PRG A16** : Set by reading from CPU $8000-$BFFF while **PAL PRG A17/A18** are 0,
  * **PAL PRG/CHR A17** : Set by writing to CPU $8000-$FFFF.
  * **PAL PRG/CHR A18** : Set by writing to CPU $8000-$FFFF.



## Contents

  * 1 16V8 Write ($8100-$FFFF)
  * 2 16V8 Read ($8000-$BFFF)
  * 3 Effective PRG-ROM banks
  * 4 See also



## 16V8 Write ($8100-$FFFF)
    
    
    Mask: $8100
    
    A~FEDC BA98 7654 3210
      -------------------
      1.u. ...1 ..vw ....
        |         |+------ PAL PRG/CHR A17
        |         +------- PAL PRG/CHR A18
        +----------------- PAL PRG A15 in some configurations
    

This register overlays the MMC3's registers in the same address range. 

## 16V8 Read ($8000-$BFFF)

Mask: $C000 

When reading from the CPU $8000-$BFFF area while **PAL PRG/CHR A17/A18** are zero, **PAL PRG A16** is updated with the MMC3 clone's MMC3 PRG A16 output. 

## Effective PRG-ROM banks

The four PAL signals not only provide additional address bits, but also modify the masking of other address bits and which 8 KiB CPU bank the MMC3 clone sees. The following table shows how the 8 KiB PRG-ROM bank coming from the MMC3 clone is first ANDed and then ORed depending on the PAL PRG signals and the CPU bank (all $xx numbers represent 8 KiB bank numbers): 
    
    
      PAL PRG      CPU      MMC3
    ------------           ------
    A17/A18  A16   Bank*    AND   OR
    ---------------------- ------ ---------------
    $00      $00   **       $03   A15|A16|A17|A18
    $00      $08   **       $07       A16|A17|A18
    $10/$20  -     **       $0F           A17|A18
    $30      -     $6->$E   $0B           A17|A18
    $30      -     $8->$C   $0F           A17|A18
    $30      -     $A->$E   $0F           A17|A18
    $30      -     $C->$8   $0F           A17|A18
    $30      -     $E->$A   $0F           A17|A18
    
    *  CPU banks to which this applies, and which CPU bank the MMC3 clone sees.
    ** Applies to all CPU banks, which are input to the MMC3 clone unchanged.
    

Therefore, **PAL PRG/CHR A17/A18** not only select a 128 KiB outer bank, but also select one of three bankswitching modes: 

  * Setting 0 provides a round-about means of dividing the first 128 KiB bank into two 32 KiB and one 64 KiB bank. **PAL PRG A16** is updated with the content of the corresponding MMC3 PRG bank bit by _reading_ from the respective address range, which in turn will then be applied across the _entire_ ROM address range. The content of this bit furthermore selects two bankswitching sub-modes: 
    * When clear, the MMC3 only selects _within_ one of the two 32 KiB banks of the first 64 KiB half of 128 KiB bank #0, which one is decided by **PAL PRG A15**. This is used for the CNROM/NROM-256 games _Power Soccer_ and Nintendo's _Soccer_.
    * When set, the MMC3 selects _within_ the second 64 KiB bank of 128 KiB bank #0. This is used for the game _Tecmo World Cup Soccer_ (renamed "Goal 5").
  * Settings 1 and 2 provide normal 128 KiB MMC3 banking, used for the games _Kunio-kun no Nekketsu Soccer League_ (renamed "Goal 3") and _Nekketsu Koukou Dodgeball Bu - Soccer Hen_ (renamed "Heroes Soccer").
  * Setting 3 provides 128 KiB MMC3 banking with the CPU A14 line fed to the MMC3 clone reversed. This is used for the game _Tecmo Cup: Soccer Game_ (renamed "Tecmo Cup Soccer"), originally an MMC1 game with the fixed bank at $8000-$BFFF and the switchable bank at $C000-$FFFF, a configuration that could not be reproduced with an MMC3 alone. It is also used for the menu, which in part executes from PRG-ROM mapped to the CPU $6000-$7FFF address range on the MMC3 clone's fixed banks alone, as no MMC3 PRG bank register is written to before JMPing to this address range.



## See also

[PCB image and analysis by Krzysiobal](https://forums.nesdev.org/viewtopic.php?f=9&t=19137)

Categories: [Mappers triggering on reads](Category_Mappers_triggering_on_reads.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
