# Bus conflict

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Bus_conflict) | View [other pages](Special_AllPages.xhtml#Bus_conflict)

A **bus conflict** occurs when two logic devices output different values on the same [bus](https://en.wikipedia.org/wiki/Bus_\(computing\) "wikipedia:Bus \(computing\)") line. When two signals are asserted at the same voltage, the one with less impedance generally wins. In the NES, this generally happens when a program writes to a [mapper](Mapper.xhtml "MMC") whose registers overlap ROM but the ROM does not shut off its output, causing a potential conflict on the PRG data bus. Most ASIC based mappers include logic to disable the ROM's output enable during writes, putting the ROM's outputs into a high-impedance state and preventing the bus conflict. But many mappers, especially [discrete logic mappers](Category_Discrete_logic_mappers.xhtml "Category:Discrete logic mappers"), do not. 

## Programming around bus conflicts

If you are using a mapper with bus conflicts, make sure that all devices on the bus are asserting the same value by writing to a ROM location that already contains the value that you are writing. For instance, to switch to bank 5 in [UNROM or UOROM](UxROM.xhtml "UxROM"), write a 5 to a ROM location that already contains a 5. 

One common way to do this is to perform an immediate load and then store over the opcode: 
    
    
    @loadInstruction:
      ldy #5
      sty @loadInstruction+1
    

To switch to a bank based on the value of a variable, put it in an indexed register and then perform an absolute indexed store: 
    
    
      lda curMapBank
      tax
      sta bankBytes,x
    ; ...
    bankBytes:
      .byt $00, $01, $02, $03, $04, $05, $06, $07
    

## Emulating bus conflicts

Many emulators have incorrectly assumed that the CPU "wins" all bus conflicts; that is, that the mapper circuitry sees the signals from the CPU more strongly than the signals from the PRG ROM and acts solely on the CPU. Quite a few early programs in [iNES](INES.xhtml "INES") format were developed without taking bus conflicts into account and do not work correctly when run on real hardware. In general, the authors of these programs did not know at the time that bus conflicts existed. These programs can, however, be made to run by adding ROM-disabling circuitry like that of [ANROM](AxROM.xhtml "ANROM") or the positive chip enable of the PRG ROM chips used with [AOROM](AxROM.xhtml "AOROM"). [NES 2.0 submappers](NES_2_0_submappers.xhtml "NES 2.0 submappers") can be used to specify that these programs must be run without bus conflicts. 

The following classes of iNES files will often contain bugs causing bus conflicts: 

  * Old homebrew ROMs: Old documents did not mention the possibility of bus conflicts.
  * Mapper hacks: Because the [MMC1](MMC1.xhtml "MMC1") was poorly understood, and the code to operate an MMC1 is generally larger than that for a discrete logic mapper without bus conflicts, the early English translations of the Famicom game _Final Fantasy II_ changed it from [MMC1](MMC1.xhtml "MMC1") to a variant of [mapper 2](UxROM.xhtml "INES Mapper 002").
  * Buggy games, homebrew, or hacks: Code or tables to avoid bus conflicts may have been written incorrectly, or a jump may have sent the program counter to somewhere that isn't code.



It has been confirmed through [testing](http://forums.nesdev.org/viewtopic.php?p=109708#p109708) that both the CPU and the mask ROMs used in the NES era drive a 0 more strongly than a 1, as one would expect based on the logic's implementation. This implies that an emulator should use the bitwise AND of the value from the CPU and the value from the ROM. However, programmers must not rely on this undefined behavior. Logging a warning when emulating a bus conflict can help modern developers identify bugs in their games and potentially help debug issues in an emulator's PRG ROM bank switching. 

## See also

  * [Mappers with bus conflicts](Category_Mappers_with_bus_conflicts.xhtml "Category:Mappers with bus conflicts")
  * [Open bus](Open_bus_behavior.xhtml "Open bus") \- The opposite condition where nothing is currently trying to output to the bus.


