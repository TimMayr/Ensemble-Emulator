# IRQ

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/IRQ) | View [other pages](Special_AllPages.xhtml#IRQ)

**IRQ** (interrupt request) is a signal on the NES CPU. It is used to trigger a [CPU interrupt](CPU_interrupts.xhtml "CPU interrupts"). 

If the CPU's /IRQ input is 0 at the end of an instruction, then the CPU pushes the program counter and the processor status register, sets the I flag to ignore further IRQs, and the Program Counter takes the value read at $fffe and $ffff. 

This behaviour is masked by the CPU's interrupt-disable [status flag](Status_flags.xhtml "Status flag"). The SEI instruction disables IRQs, and the CLI instruction enables them. 

/IRQ functions as an open collector input: it is normally 1, but any device on the CPU bus can pull it down to 0. An IRQ handler is expected to push any registers it uses, acknowledge the interrupt by writing to a port so that the source no longer forces /IRQ to 0, pull the registers back, and return with RTI. 

Therefore if a program uses more than one source of IRQ, the priority between the conflicting interrupts should be handled in software. 

Sources of IRQ on a Famicom or NES include 

Source | Enable | Disable | Acknowledge   
---|---|---|---  
[APU DMC](APU_DMC.xhtml "APU DMC") finish | $4010 write with bit 7 = 1 | $4010 write otherwise | Disable then reenable, or [APU Status](APU.xhtml "APU Status") ($4015) write   
[APU Frame Counter](APU_Frame_Counter.xhtml "APU Frame Counter") | $4017 write with bits 7-6 = 00 | $4017 write otherwise | [APU Status](APU.xhtml "APU Status") ($4015) read   
[MMC3](MMC3.xhtml "MMC3") | Write to $E001 | Write to $E000 | Disable then reenable   
[MMC5](MMC5.xhtml "MMC5") | Write $80 to $5204 | Write $00 to $5204 | Read $5204   
[VRC4/6/7](VRC_IRQ.xhtml "VRC IRQ") | depends on specific IC   
[FME-7](Sunsoft_FME_7.xhtml "Sunsoft FME-7") | write $81 to register $D | write even number to register $D | write anything to register $D   
[Namco 163](INES_Mapper_019.xhtml "INES Mapper 019") | write to $5000 and $5800 | $5800 write with bit 7 = 0 | write to $5000 or $5800   
[FDS](Family_Computer_Disk_System.xhtml "FDS") | Write $02 to $4022 | Write $00 to $4022 | Read $4030   
  
## See also

  * [CPU interrupts](CPU_interrupts.xhtml "CPU interrupts")
  * [Status flags](Status_flags.xhtml "Status flags") used to enable/disable IRQs.
  * [MMC](Mapper.xhtml "MMC"), common source of cartridge IRQs
  * [NMI](NMI.xhtml "NMI"), the other interrupt signal on the CPU


