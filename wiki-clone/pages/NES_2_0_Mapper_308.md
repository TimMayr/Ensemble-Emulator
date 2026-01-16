# NES 2.0 Mapper 308

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_308) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_308)

NES 2.0 Mapper 308 is used for a bootleg version of the Sunsoft game _Batman_. It uses a VRC2 clone (VRC2b, CPU A0=chip A0, CPU A1=chip A1, i.e. similar to Mapper 23 Submapper 3) with custom IRQ functionality. Its UNIF board name is **UNL-TH2131-1**. 

## Contents

  * 1 Registers
    * 1.1 VRC2-compatible registers ($8000-$FFFF)
    * 1.2 IRQ Acknowledge and Reset ($F000)
    * 1.3 IRQ Counter Enable ($F001)
    * 1.4 IRQ High Counter Value ($F003)
  * 2 IRQ Operation
  * 3 See also



# Registers

## VRC2-compatible registers ($8000-$FFFF)

See description at [VRC2](VRC2_and_VRC4.xhtml "VRC2"). 

## IRQ Acknowledge and Reset ($F000)

Mask: $F003 

Acknowledge IRQ and disable IRQ counting; reset low counter to zero. 

## IRQ Counter Enable ($F001)

Mask: $F003 

Enable IRQ counting. 

## IRQ High Counter Value ($F003)
    
    
    Mask: $F003
    
    D~7654 3210
      ---------
      VVVV ....
      ++++----- Value for high counter
    

# IRQ Operation

If IRQ counting is enabled, the 12 bit low counter is increased on every M2 cycle. Upon reaching the value 2048, the high counter is decreased. If the high counter is zero and the low counter is below 2048, an IRQ is asserted. 

# See also

[PCB image and discussion](https://forums.nesdev.org/viewtopic.php?f=28&t=13270)

Categories: [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml)
