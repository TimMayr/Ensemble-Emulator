# CPU power up state

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/CPU_power_up_state) | View [other pages](Special_AllPages.xhtml#CPU_power_up_state)

Initial tests on the power-up/reset state of the CPU/APU and RAM contents were done using an NTSC front-loading NES from 1988 with a RP2A03G CPU on the NES-CPU-07 board revision. 

Countless bugs in [commercial](Game_bugs.xhtml "Game bugs") and [homebrew](Program_compatibility.xhtml "Program compatibility") games exist because of a reliance on the initial system state. An NES programmer should not rely on the state of CPU/APU registers and RAM contents not guaranteed at power-up/reset. 

## Contents

  * 1 CPU
  * 2 APU
    * 2.1 Revision-dependent Register Values
  * 3 RAM contents
  * 4 Best practices
  * 5 See also
  * 6 References



## CPU

Initial [ CPU Register](CPU_registers.xhtml "CPU registers") Values  Register  | At Power  | After Reset   
---|---|---  
A, X, Y | 0 | unchanged   
PC | ($FFFC) | ($FFFC)   
[S](Stack.xhtml "Stack")[1] | $00 - 3 = $FD | S -= 3   
[C](Status_flags.xhtml#C:_Carry "Status flags") | 0 | unchanged   
[Z](Status_flags.xhtml#Z:_Zero "Status flags") | 0 | unchanged   
[I](Status_flags.xhtml#I:_Interrupt_Disable "Status flags") | 1 | 1   
[D](Status_flags.xhtml#D:_Decimal "Status flags") | 0 | unchanged   
[V](Status_flags.xhtml#V:_Overflow "Status flags") | 0 | unchanged   
[N](Status_flags.xhtml#N:_Negative "Status flags") | 0 | unchanged   
  
## APU

Initial [APU](APU.xhtml "APU") Register Values  Register  | At Power  | After Reset   
---|---|---  
[Pulses](APU_Pulse.xhtml "APU Pulse") ($4000-$4007) | 0 | unchanged?   
[Triangle](APU_Triangle.xhtml "APU Triangle") ($4008-$400B) | 0 | unchanged?   
[Triangle](APU_Triangle.xhtml "APU Triangle") phase | ? | 0 (output = 15)   
[Noise](APU_Noise.xhtml "APU Noise") ($400C-$400F) | 0 | unchanged?   
[Noise](APU_Noise.xhtml "APU Noise") 15-bit LFSR | $0000 (all 0s, first clock shifts in a 1)[2] | unchanged?   
[DMC](APU_DMC.xhtml "APU DMC") flags and rate ($4010)[3] | 0 | unchanged   
[DMC](APU_DMC.xhtml "APU DMC") direct load ($4011)[3] | 0 | [$4011] &= 1   
[DMC](APU_DMC.xhtml "APU DMC") sample address ($4012)[3] | 0 | unchanged   
[DMC](APU_DMC.xhtml "APU DMC") sample length ($4013)[3] | 0 | unchanged   
[DMC](APU_DMC.xhtml "APU DMC") LFSR | 0? (revision-dependent?) | ? (revision-dependent?)   
[ Status](APU.xhtml#Status_\(%244015\) "APU") ($4015) | 0 (all channels disabled) | 0 (all channels disabled)   
[Frame Counter](APU_Frame_Counter.xhtml "APU Frame Counter") ($4017) | 0 (**frame IRQ enabled**) | unchanged   
[Frame Counter](APU_Frame_Counter.xhtml "APU Frame Counter") LFSR[4] | $7FFF (all 1s) | revision-dependent   
  
### Revision-dependent Register Values

2A03 letterless  Register  | At Power  | After Reset   
---|---|---  
[DMC](APU_DMC.xhtml "APU DMC") LFSR | 0? | ?   
[Frame Counter](APU_Frame_Counter.xhtml "APU Frame Counter") LFSR[4] | $7FFF (all 1s) | unchanged   
2A03E, 2A03G, 2A07, various clones  Register  | At Power  | After Reset   
---|---|---  
[DMC](APU_DMC.xhtml "APU DMC") LFSR | 0? | ?   
[Frame Counter](APU_Frame_Counter.xhtml "APU Frame Counter") LFSR[4] | $7FFF (all 1s) | $7FFF (all 1s)   
  
## RAM contents

Internal RAM ($0000-$07FF) and cartridge RAM (usually $6000–$7FFF, depends on mapper) have an unreliable state on power-up and is unchanged after a reset. Some machines may have consistent RAM contents at power-up, but others may not. Emulators often implement a consistent RAM startup state (e.g. all $00 or $FF, or a particular pattern), and [ flashcarts](Category_Flash_Cartridge.xhtml "Flashcart") may partially or fully initialize RAM before starting a program. 

Battery-backed save RAM and other types of SRAM/NVRAM have an unreliable state on the first power-up and is generally unchanged after subsequent resets and power-ups. However, there is an added chance of data corruption due to loss of power or other external factors (bugs, cheats, etc). Emulators and flashcarts may initialize save files with a consistent state (much like other sections of RAM) and persist this data without corruption after closing or reloading a game. 

Because of these factors, an NES programmer must be careful not to blindly trust the initial contents of RAM. 

## Best practices

  * Configure the emulator so it provides a random system state and random RAM contents on power-up. 
    * [Mesen](https://www.mesen.ca/) provides a set of such emulation options recommended for developers, along with a debugger setting to break execution on all reads from uninitialized RAM.
  * Refer to the [init code](Init_code.xhtml "Init code") article when setting up the reset handler. The sample implementation is a good point to start from. 
    * If you are using an [audio driver](Audio_drivers.xhtml "Audio drivers"), make sure to call its initialization routine in the reset handler before playing any sound.
  * If some RAM state is intended to persist across resets, ensure that the checks used to do so are robust against random initial RAM contents. (e.g. unique multi-byte signatures, checksum calculations, etc)
  * Validate any data read from potentially unreliable sources before using it. For example, the stats of an RPG character could be checked against valid ranges when loading them from a save.



## See also

  * [PPU power up state](PPU_power_up_state.xhtml "PPU power up state")



## References

  1. ↑ RESET uses the logic shared with NMI, IRQ, and BRK that would push PC and P. However, like [some but not all 6502s](Visual6502wiki_6502_BRK_and_B_bit.xhtml#masking_of_the_stack_writes_during_RESET "Visual6502wiki/6502 BRK and B bit"), the 2A03 prohibits writes during reset. [This test](https://forums.nesdev.org/viewtopic.php?p=184247#p184247) relies on open bus being precharged by these reads. See [27c3: Reverse Engineering the MOS 6502 CPU (en)](https://www.youtube.com/watch?v=fWqBmmPQP40&t=41m45s) from 41:45 onward for details
  2. ↑ [Noise channel init log](https://forums.nesdev.org/viewtopic.php?p=172797#p172797)
  3. ↑ 3.0 3.1 3.2 3.3 [DMC power-up state manifests as buzzing in Eliminator Boat Duel](https://forums.nesdev.org/viewtopic.php?p=231773#p231773)
  4. ↑ 4.0 4.1 4.2 [2A03letterless is missing transistor to set frame counter LFSR on reset](https://forums.nesdev.org/viewtopic.php?p=214939#p214939)


