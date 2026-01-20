# CPU memory map

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/CPU_memory_map) | View [other pages](Special_AllPages.xhtml#CPU_memory_map)

Address range | Size | Device   
---|---|---  
$0000–$07FF | $0800 | 2 KB internal RAM   
$0800–$0FFF | $0800 | [Mirrors](Mirroring.xhtml#Memory_Mirroring "Mirroring") of $0000–$07FF   
$1000–$17FF | $0800   
$1800–$1FFF | $0800   
$2000–$2007 | $0008 | [NES PPU registers](PPU_registers.xhtml "PPU registers")  
$2008–$3FFF | $1FF8 | Mirrors of $2000–$2007 (repeats every 8 bytes)   
$4000–$4017 | $0018 | [NES APU](APU.xhtml "APU") and [I/O registers](2A03.xhtml "2A03")  
$4018–$401F | $0008 | APU and I/O functionality that is normally disabled. See [CPU Test Mode](CPU_Test_Mode.xhtml "CPU Test Mode").   
$4020–$FFFF  
 _• $6000–$7FFF  
• $8000–$FFFF_ | $BFE0  
 _$2000  
$8000_ | Unmapped. Available for cartridge use.  
_Usually cartridge RAM, when present.  
Usually cartridge ROM and [mapper](Mapper.xhtml "Mapper") registers._  
  
  * Some parts of the 2 KiB of internal RAM at $0000–$07FF have predefined purposes dictated by the 6502 architecture: 
    * $0000-$00FF: The zero page, which can be accessed with fewer bytes and cycles than other addresses
    * $0100–$01FF: The page containing the stack, which can be located anywhere here, but typically starts at $01FF and grows downward



    Games may divide up the rest however the programmer deems useful. See [Sample RAM map](Sample_RAM_map.xhtml "Sample RAM map") for an example allocation strategy for this RAM. Most commonly, $0200-$02FF is used for the OAM buffer to be copied to PPU OAM during vblank.

  * The unmapped space at $4020-$FFFF can be used by cartridges for any purpose, such as ROM, RAM, and registers. Many common mappers place ROM and save/work RAM in these locations: 
    * $6000–$7FFF: Battery-backed save or work RAM (usually referred to as WRAM or PRG-RAM)
    * $8000–$FFFF: ROM and mapper registers (see [MMC1](MMC1.xhtml "MMC1") and [UxROM](UxROM.xhtml "UxROM") for examples)



    The cartridge is able to passively observe reads from and writes to any address in the CPU address space, even outside this unmapped space, except for reads from $4015, the only readable register that is internal to the CPU. The cartridge can map writable registers anywhere, but its readable memory can only be placed where it does not interfere with other readable hardware, which would produce a [bus conflict](Bus_conflict.xhtml "Bus conflict"). While cartridges can map readable memory at $4000-$4014 and $4018-$401F, a quirk in the 2A03's register decoding can cause DMA to misbehave if the CPU is halted while reading from $4000-$401F, so it is recommended that cartridges only map readable memory from $4020-$FFFF.

  * If using [DPCM playback](APU_DMC.xhtml "APU DMC"), samples are limited to the following practical range: 
    * $C000–$FFF1: DPCM sample data



    Sample playback wraps around from $FFFF to $8000. The highest sample starting address is $FFC0 and longest sample is $FF1 bytes, so the full DPCM range is $C000-$FFFF and $8000-$8FB0, but making use of the wraparound is challenging because of banking and the presence of the CPU vectors.

  * The CPU expects interrupt vectors in a fixed place at the end of the unmapped space: 
    * $FFFA–$FFFB: NMI vector, which points at an [NMI](NMI.xhtml "NMI") handler
    * $FFFC–$FFFD: Reset vector, which points at [code to initialize the NES chipset](Init_code.xhtml "Init code")
    * $FFFE–$FFFF: IRQ/BRK vector, which may point at a mapper's [interrupt](IRQ.xhtml "IRQ") handler (or, less often, a handler for APU interrupts)



    These vectors are supplied by the cartridge. Unless a mapper fixes $FFFA–$FFFF to some known bank (normally by fixing an entire bank-sized region at the top of the address space, such as $C000-$FFFF, to a specific bank) or uses some sort of reset detection, the vectors (and a suitable reset code stub) must be present in all banks.

  * Reading from memory that is not mapped to anything normally returns [open bus](Open_bus_behavior.xhtml "Open bus behavior"). The cartridge hardware may affect open bus behavior across the entire CPU address space, such as by pulling bits high or low.


