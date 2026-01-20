# Expansion Port Sound Module

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Expansion_Port_Sound_Module) | View [other pages](Special_AllPages.xhtml#Expansion_Port_Sound_Module)

The NES Expansion Port Sound Module is an aftermarket homebrew addition that adds 6 channels of 4-operator FM, 3 channels of [Sunsoft 5B audio](Sunsoft_5B_audio.xhtml "Sunsoft 5B audio"), and a six-instrument sampled drumkit to the front-loading NES. Your NES does not need to be modified - just plug it in the bottom [expansion port](Expansion_port.xhtml "Expansion port") and connect audio to it instead, where it's mixed with the 2A03 audio. 

The EPSM uses Yamaha's [YMF288](https://www.quarter-dev.info/archives.php) (OPN3) chip, which is a lower-power and smaller variant of the [YM2608](https://en.wikipedia.org/wiki/Yamaha_YM2608 "wikipedia:Yamaha YM2608") without most of the sample playback abilities. 

It can operate in two different access methods: 

## Contents

  * 1 Universal access
    * 1.1 $4016 Write (OUT1 rising)
    * 1.2 $4016 Write (OUT1 falling)
    * 1.3 Caveat and workarounds
      * 1.3.1 Long writes
      * 1.3.2 DMC DMA sync
      * 1.3.3 OAM DMA sync
      * 1.3.4 Controller strobe detection
  * 2 Mapper-specific access
  * 3 Tool support
  * 4 References



## Universal access

Rising and falling edges of the OUT1 pin transmit 10 total bits, specifying two address lines and eight data lines. In this mode, it is compatible with all existing cartridges and can operate without any assistance from the cartridge. 

Note that YMF288 address bits are **in reverse order**. 

### $4016 Write (OUT1 rising)
    
    
    7  bit  0
    DDDD AA1.
    |||| |||
    |||| ||+-- DDDD and AA latched when this bit goes from 0 to 1
    |||| ++--- D3=YMF288 A0, D2=YMF288 A1
    ++++------ YMF288 D7-D4
    

### $4016 Write (OUT1 falling)
    
    
    7  bit  0
    DDDD ..0.
    ||||   |
    ||||   +-- DDDD latched and write triggered when this bit goes from 1 to 0
    ++++------ YMF288 D3-D0
    

### Caveat and workarounds

* * *

A barely-noticed design flaw of the 2A03 imposes strict timing constraints on using the universal method: OUT0 through OUT2 are only updated on every APU clock, while the CPU only drives the intended value to the data bus for one CPU clock. Therefore, the EPSM may see OUT1 toggle on the cycle after the data was on the bus, causing it to instead read other, incorrect data. 

Two approaches exist to work around this issue: 

  * **Long writes** : This method keeps the data on the bus for both cycles so the EPSM sees the correct data even if the OUT1 toggle is delayed. It works in all cases and can be handled entirely by a library, so it is the preferred method, but it has a per-write overhead that reduces throughput.
  * **Synchronized writes** : This method involves synchronizing EPSM writes so OUT1 always toggles on the correct cycle. This is done by placing the CPU and APU into a known alignment and then using timed code to do EPSM writes on the correct cycle parity. The current alignment can be detected with controller reads, or the CPU and APU can be forced into an alignment with OAM or DMC [DMA](DMA.xhtml "DMA"). This works because the DMA unit is aligned with the APU clock and alternates between being able to put (write) on one CPU cycle and get (read) on the next, and the OUT delay only occurs for writes on put cycles. Synchronized writes are much more difficult and limited than long writes, but have the potential for higher throughput.



#### Long writes

This is the preferred method for safely writing to the EPSM. 

The EPSM is guaranteed to see the OUT1 toggle either on the CPU cycle $4016 is written or on the next cycle. Therefore, if the data is present on the CPU bus for both cycles, then the data cannot be missed and synchronization between the CPU and APU is unnecessary. The first cycle after the $4016 write is always a read of the following instruction's opcode. By choosing an instruction whose opcode matches the data written to the EPSM, the write will remain valid for both cycles. This mechanism is also interrupt-safe and DMA-safe because the opcode is fetched even when an interrupt or DMA halt occurs between the two instructions. 

The EPSM only reads the upper 6 bits of the written value. This means there are 64 values that need a corresponding opcode. Because the lower 2 bits aren't used by the EPSM, there are 4 possible instructions for each value. By carefully choosing input registers and targeting specific locations in RAM, a safe instruction for each value can be chosen that avoids corrupting RAM or reading outside zero page. 

A long-write library by [Fiskbit](User_Fiskbit.xhtml "User:Fiskbit") automates this process.[1]

#### DMC DMA sync

If the [DMC](APU_DMC.xhtml "DMC") channel is not in use, DMC DMA can be used to synchronize the CPU and APU because it always ends on a get cycle. Starting a silent, one-byte, non-looping sample will trigger a DMA 3 or 4 cycles later. By delaying the earlier of these with a write cycle, the DMA always occurs on the 4th cycle, synchronizing the following code. Write cycles to $4016 afterward must then be aligned to land on get cycles. 

This approach allows writes to be done with little overhead, but has numerous caveats: 

  * An obscure DMC DMA bug can cause a second DMA to halt the CPU on the third cycle after the first DMA, and this second DMA is aborted after just one cycle, inverting the synchronization. The aborted DMA is prevented altogether if it is delayed by a write cycle. The following code compensates for this and successfully synchronizes:


    
    
    STx $4015  ; Initiate DMC DMA.
    STx zp     ; Force DMA to 4 cycles later.
    STx zp     ; Override the second DMA.
    ; The first cycle of the next instruction is a PUT cycle.
    

    In this code, the stores can be from any registers and the zp writes must use the zero page addressing mode targeting any zero page address, required to force write cycles into specific timings that delay the DMA.

    Note that an interrupt must not occur between the first two writes, as this would prevent the second DMA from being overridden.

  * DMC DMA cannot be used to synchronize too frequently. The DMA fills a sample buffer which is consumed by the DMC output unit periodically, as determined by the DMC rate. DMA occurs whenever the sample buffer is empty. This synchronization method requires that the buffer be empty, and so it cannot be used more frequently than the DMC rate. The fastest rate (432 CPU cycles) should be chosen to empty the buffer most quickly, and each series of writes to the EPSM should be synced relative to each other rather than repeatedly triggering DMC DMA.


  * Any interrupts that can be handled in the synchronized region must take an even number of cycles to avoid breaking sync. As described above, however, interrupts may also prevent the second-DMA override from working, inverting the alignment.



#### OAM DMA sync

If DMC DMA is not in use, [OAM DMA](PPU_registers.xhtml "OAMDMA") can be used to synchronize the CPU and APU because it always ends on a put cycle, causing the next instruction to begin on a get cycle. Placing OAM DMA last in vblank and following it with synchronized EPSM writes allows the EPSM to be safely written without wasting vblank time or spending additional time aligning. OAM DMA can also be done at any other time to synchronize, but takes many CPU cycles and may corrupt OAM. 

This approach also has caveats: 

  * DMC DMA occurring on the last or 3rd-to-last cycles of OAM DMA will halt the CPU for an odd number of cycles, inverting the alignment. This prevents OAM DMA sync from working properly while DMC DMA is in use in most situations. If this can be worked around, the first cycle after any contiguous sequence of write cycles in the synchronized region must be a put, as described below in Controller strobe detection.


  * Any interrupts that can be handled in the synchronized region must take an even number of cycles to avoid breaking sync.



#### Controller strobe detection

If an official [standard controller](Standard_controller.xhtml "Standard controller") is present, any reads after the button report is finished return 1. Incrementing $4016 with the controller in this state will perform a single-cycle strobe. Whether the controller sees this strobe depends on the alignment. Because opposite directions on a controller cannot be pressed, at least two D-pad bits are guaranteed to be 0, so reading the D-pad bits with synchronized code to see if any are 0 will indicate whether the strobe was seen. The code can then correct the alignment, if necessary, and perform synchronized EPSM writes. 

Caveats: 

  * This is only guaranteed to work in the presence of official standard controllers. Third-party standard controllers may give 0's instead of 1's after the button report. Other kinds of controllers have their own behavior that may not be compatible, and the console may not even have controllers plugged in.


  * If DMC DMA occurs on the same cycle as a $4016 read, one extra read may occur, deleting a bit from the report. However, because the D-pad has at least two 0 bits, at least one of them can still be seen. (Note that some console types, such as the RF Famicom, do an extra joypad read on each halted, non-DMA cycle, which could cause all 0's to be missed, but this is not the case on the NES-001, the only console with the EPSM's required expansion port.)


  * DMA for refilling the DMC sample buffer normally halts on a put cycle and takes 4 CPU cycles unless delayed by a write cycle, in which case it may take 3. The synchronized code can be kept synchronized in the presence of DMC DMA by ensuring the first cycle after any contiguous sequence of write cycles is a put. This includes the 3 write cycles that occur when handling an interrupt, so maintaining sync in the presence of both DMC DMA and interrupts may be infeasible.


  * Any interrupts that can be handled in the synchronized region must take an even number of cycles to avoid breaking sync.



## Mapper-specific access

In this addressing mode, five of the ten [EXP pins](EXP_pins.xhtml "EXP pins") on the [cartridge](Cartridge_connector.xhtml "Cartridge connector") gain defined function: 
    
    
    EXP1 = EPSM /CE1
    EXP3 = EPSM /CE2
    EXP4 = EPSM A1
    EXP7 = EPSM A0
    EXP8 = EPSM CE3
    

For example, if connected as follows, the EPSM will have a suitable memory map for software that expects [Sunsoft 5B audio](Sunsoft_5B_audio.xhtml "Sunsoft 5B audio")
    
    
    EXP1 - EPSM /CE1 = CPU R/W
    EXP3 - EPSM /CE2 = /ROMSEL
    EXP4 - EPSM A1   = CPU A1
    EXP7 - EPSM A0   = CPU A13
    EXP8 - EPSM CE3  = CPU A14
    

## Tool support

Mesen2 and Mesen-X both support EPSM. The ROM should use an NES 2.0 header specifying the EPSM console type. This enables both the universal access method and mapper-specific access at these fully-decoded addresses: 
    
    
    $401C = Reg write A1=0
    $401D = Data write A1=0
    $401E = Reg write A1=1
    $401F = Data write A1=1
    

FamiStudio has support for EPSM with NSF and ROM exports using mapper-specific addressing, it does also offer sourcecode for it's audio driver in CA65,ASM6 and NESASM. 

## References

  1. ↑ [GitHub:](https://github.com/Perkka2/EPSM/blob/main/Docs/Fiskbit-FallbackHandler.md) Fiskbit's long write library for safe EPSM universal access.


