# Programming MMC1

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Programming_MMC1) | View [other pages](Special_AllPages.xhtml#Programming_MMC1)

[MMC1](MMC1.xhtml "MMC1") was Nintendo's first ASIC mapper for the NES. 

## Contents

  * 1 Quick setup for UNROM style
  * 2 PRG banks
  * 3 Interrupts
  * 4 See also
  * 5 References
  * 6 External links



## Quick setup for UNROM style

If you are using the [SGROM](SxROM.xhtml "SGROM") or [SNROM](SxROM.xhtml "SNROM") board to provide an environment similar to [UNROM](Programming_UNROM.xhtml "Programming UNROM"), with 8 KB of CHR RAM, a fixed PRG ROM bank at $C000, and a 16 KB switchable PRG ROM bank at $8000, do this in your init code after the mapper has been reset: 
    
    
      lda #$0E   ; vertical mirroring, fixed $C000, 8 KB CHR pages
      sta $8000  ; (use $0F instead for horizontal mirroring)
      lsr a
      sta $8000
      lsr a
      sta $8000
      lsr a
      sta $8000
      lsr a
      sta $8000
    

Games that use CHR RAM switch to another PRG bank before they [copy tile data into CHR RAM](CHR_ROM_vs__CHR_RAM.xhtml#Switching_to_CHR_RAM "CHR-ROM vs CHR-RAM"). 

## PRG banks

Some revisions of the MMC1 IC might power up in a mode other than fixed-$C000, requiring that the vectors and the start of the [init code](Init_code.xhtml "Init code") be placed in all banks, much as in [BxROM](BNROM.xhtml "BxROM") or [AxROM](AxROM.xhtml "AxROM") or [GxROM](GxROM.xhtml "GxROM"). Other revisions guarantee that the fixed bank is loaded at power on. To make sure your code works on all MMC1 revisions, put the following code in the last 16 bytes of each 16384 byte bank. (_Barbie_ uses almost identical code.) 
    
    
    reset_stub:
      sei
      ldx #$FF
      txs        ; set the stack pointer
      stx $8000  ; reset the mapper
      jmp reset  ; must be in $C000-$FFED
      .addr nmiHandler, reset_stub, irqHandler
    

Then to switch PRG ROM banks, load the bank number (0-15) into A and call this subroutine: 
    
    
    mmc1_load_prg_bank:
      sta $E000
      lsr a
      sta $E000
      lsr a
      sta $E000
      lsr a
      sta $E000
      lsr a
      sta $E000
      rts
    

## Interrupts

If an NMI or IRQ can interrupt a series of writes, it is not easy to know what state the serial register was in before the interruption. One technique for coping with this problem involves using a flag variable to indicate that a serial write should be retried[1]: 

  1. The normal serial write routine should have an "interrupted" flag: 
     * Clear the flag before beginning the series of writes.
     * When completed, check the flag. If an interrupt is indicated, reset the serial register, clear the interrupted flag and begin the serial writes again.
  2. If the NMI or IRQ needs to switch banks, it should reset the serial register and set the "interrupted" flag to indicate that it has done this.



## See also

  * [MMC1](MMC1.xhtml "MMC1") technical reference



## References

  1. ↑ [forum post](https://forums.nesdev.org/viewtopic.php?p=132577#p132577): thefox describes MMC1 interrupt technique used in STREEMERZ



## External links

  * [SNROM project template](https://github.com/pinobatch/snrom-template) by Damian Yerrick on GitHub


