# NES 2.0 Mapper 263

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_263) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_263)

NES 2.0 Mapper 263 is used for an [MMC3](MMC3.xhtml "MMC3") clone with scrambled mapper address and data bits. Its UNIF board name is **UNL-KOF97**. It is used for two games by Rex Soft: 

  * _Boogerman II: The Final Adventure_
  * _The King of Fighters '97_



## Registers

A12 is used for the MMC3's A0: 

Mask: $F000 
    
    
    Mapper 263    Regular MMC3
    address       address
    ----------    ------------
    8000          8000
    9000          8001
    A000          A000
    B000          A001
    C000          C000
    D000          C001
    E000          E000
    F000          E001
    

The data written to the MMC3 clone has the following bit order compared to a regular MMC3: 
    
    
    Written bit 76543210
                --------
    Actual bit  76243051
    

Which means that to get from the written data to the actual data, the following operation must be performed: 
    
    
    Val =(Val &0xD8) | ((Val &0x20) >>4) | ((Val &4) <<3) | ((Val &2) >>1) | ((Val &1) <<2);
    

## Note

  * Most versions of _The King of Fighters '97_ are hacked for [Mapper 4](MMC3.xhtml "INES Mapper 004"). Several of them also fail to disable the [frame IRQ](APU_Frame_Counter.xhtml "APU Frame Counter"), freezing on emulators that accurately [power-on with the frame IRQ enabled](CPU_power_up_state.xhtml "CPU power up state").



Categories: [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
