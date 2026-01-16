# VT02+ PRG-ROM Bankswitching

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/VT02%2B_PRG-ROM_Bankswitching) | View [other pages](Special_AllPages.xhtml#VT02__PRG_ROM_Bankswitching)

In OneBus mode, VT02+ consoles combine the Famicom cartridge connector's CPU and PPU address lines into one 32 MiB address space, with separate CPU and PPU bankswitch registers pointing to appropriately-placed code and picture data. The bankswitching scheme is based on, and indeed backwards-compatible to, the Nintendo [MMC3](MMC3.xhtml "MMC3")'s. The CPU address range is divided into four 8 KiB banks. For each of these four 8 KiB banks, the bank number is made up of three components: 

  * an Inner Bank that resembles the MMC3's bank registers,
  * a Middle Bank that can replace zero to eight bits of the lower bank number,
  * an Outer Bank that extend the address range up to 32 MiB.



The final 8 KiB PRG-ROM bank number therefore is: 
    
    
    BankNumber = (InnerBank &InnerBankMask) | (MiddleBank &~InnerBankMask) | (OuterBank <<8);
    

## Inner PRG Bank number

The lower bits bits of the 8 KiB PRG-ROM bank number, constituting the Inner Bank number, are normally the only ones that are manipulated by individual games. By default, they resemble the MMC3's original bank registers; accordingly, two of the four banks are fixed. By setting bit 6 in register $410B (PQ2EN), the $C000-$DFFF bank may be turned into a selectable bank as well. 
    
    
    CPU $8000-$9FFF: Selected by register $4107 (PQ0), akin to MMC3 register 6.
    CPU $A000-$BFFF: Selected by register $4108 (PQ1), akin to MMC3 register 7.
    CPU $C000-$DFFF: If $410B bit 6 (PQ2EN)=0: Fixed to $FE, or second-to-last bank (within the Middle/Outer Bank), as on the MMC3.
                     If $410B bit 6 (PQ2EN)=1: Selected by register $4109 (PQ2), an enhancement over the MMC3.
    CPU $E000-$FFFF: Fixed to $FF, or last bank (within the Middle/Outer Bank), as on the MMC3.
    

If $4105 bit 6 (COMR6) is 1, then the sources of the $8000-$9FFF/$A000-$BFFF bank numbers are swapped with the $C000-$DFFF/$E000-$FFFF banks', just as on the MMC3, or in other words, CPU A14 is inverted. 

## Middle PRG Bank mask and number

The Middle Bank is normally only used on multicarts. It allows masking off and replacing bits of the Inner Bank number, so that several games may be put into one Outer Bank. Bits 0-2 of register $410B (PS) select the AND mask that is applied to the Inner Bank number. Only the bits that have been masked off that way are then replaced with the respective bits from register $410A (PQ3): 
    
    
    $410B     Inner Bank  Middle Bank Effective
    bits 0-2  AND Mask    AND Mask    Inner Bank Size
    --------  ----------  --------    ---------------
    0         3F          C0          512 KiB
    1         1F          E0          256 KiB
    2         0F          F0          128 KiB
    3         07          F8          64 KiB
    4         03          FC          32 KiB
    5         01          FE          16 KiB
    6         00          FF          8 KiB
    7         FF          00          2048 KiB
    

## Outer PRG Bank number

The Outer Bank number is used mostly by multicarts, but also by very large games for which the maximum Inner Bank size of 2 MiB is insufficient. On the VT02 and VT03, bits 4-7 of register $4100 simply select the 2 MiB Outer Bank number for all four banks. 

VT02, VT03; with $411C bit 5 (EXT2421)=0: 
    
    
    CPU $8000-$FFFF: Selected by register $4100 bits 4-7 (PQ7).
    

If $4105 bit 6 (COMR6) is 1, then the sources of the $8000-$9FFF/$A000-$BFFF bank numbers are swapped with the $C000-$DFFF/$E000-$FFFF banks', as was the case with the Inner Bank number. 
