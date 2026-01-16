# PRG RAM circuit

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/PRG_RAM_circuit) | View [other pages](Special_AllPages.xhtml#PRG_RAM_circuit)

The [iNES](INES.xhtml "INES") format implies 8 KiB of PRG RAM at $6000–$7FFF, which may or may not be battery backed, even for [discrete boards](Category_Discrete_logic_mappers.xhtml "Category:Discrete logic mappers") such as [NROM](NROM.xhtml "NROM") and [UxROM](UxROM.xhtml "UxROM") that never actually had RAM there. This inspired some people on the nesdev.org BBS to come up with **circuits to add PRG RAM** to the original boards, so that games relying on it can run on an NES. The primary problem is in producing the enable signals for a [62256](6264_static_RAM.xhtml#6264_Pinout_\(62256_in_parentheses\) "6264 static RAM") or [6264 static RAM](6264_static_RAM.xhtml "6264 static RAM") or compatible PSRAM. 

On the forum, kyuusaku and Bregalad discussed PRG RAM decoder circuits built from [7400 series parts](https://en.wikipedia.org/wiki/List_of_7400_series_integrated_circuits "wikipedia:List of 7400 series integrated circuits") to approximate this behavior in an NES cartridge board. The first tries took two chips[[1]](https://forums.nesdev.org/viewtopic.php?p=32389#p32389) or had possible timing problems.[[2]](https://forums.nesdev.org/viewtopic.php?p=32520#p32520)[[3]](https://forums.nesdev.org/viewtopic.php?p=32531#p32531) They settled on the following circuits: 

## Contents

  * 1 Decoding
    * 1.1 Using 7410
    * 1.2 Using 7420
    * 1.3 Using 74139
  * 2 /ROMSEL delay
    * 2.1 /ROMSEL delay issues
  * 3 Battery backup
  * 4 References



## Decoding

### Using 7410

kyuusaku suggested a circuit based on a [74HC10](7410.xhtml "7410") (triple three-input NAND) stick a pulldown on CE2 to take advantage of Phi2 going high-impedance during reset in order to "offer some write protection".[[4]](https://forums.nesdev.org/viewtopic.php?p=35231#p35231)
    
    
               ,-------------- ROM /CE
              |   ____
    /ROMSEL --+--|    `-.
                 |       \
    A14 ---------|        )o-- RAM /CE
                 |       /
    A13 ---------|____,-'
    
                  ____
    +5V ------+--|    `-.
              |  |       \
              `--|        )o-- ROM /OE
                 |       /
    R/W ------+--|____,-'
              |
              `--------------- RAM /WE
    
    Phi2 ---------+----------- RAM CE2
                  |
                  <
                  < "big R"
                  <
                  |
    GND ----------+----------- RAM /OE
    

### Using 7420

He also suggested a circuit based on a 74HC20 (double 4-input NAND), which appears to be the same one in _Family BASIC_ : 

    Or you could just use a NAND4 to decode any active low memory, also using the /WE priority method. If this is done with a two gate 7420, the second gate could be used to invert r/w to prevent bus conflicts as in the circuit above. This is probably the *final* best way unless you happen to need the extra AND3 from the 7410 and have a positive CE.

The [pinout](https://forums.nesdev.org/viewtopic.php?p=72181#p72181): 

  * A = Phi2
  * B = /ROMSEL
  * C = A14
  * D = A13
  * Y = PRG RAM /CE
  * PRG RAM /OE = GND
  * PRG RAM /WE = Vcc or R//W, depending on the Family BASIC cart's write-protect switch



Kevin Horton suggested [the same circuit](https://forums.nesdev.org/viewtopic.php?p=76149#p76149). 

You could also use the other gate to invert R//W for /OE on the ROM to prevent bus conflicts. 

### Using 74139

If you don't need bus conflict prevention, you can use a [74HC139](74139.xhtml "74139") (double 2-to-4 decoder), which may be cheaper or have better timing than a 74HC20. This circuit resembles the decoder in Jaleco's discrete mappers ([87](INES_Mapper_087.xhtml "INES Mapper 087") and [140](INES_Mapper_140.xhtml "INES Mapper 140")), which uses a 74139 to decode a single mapper register to $6000–$7FFF. 

  * 1/E = GND
  * 1A0 = M2
  * 1A1 = A14
  * 2/E = 1/Y3
  * 2A0 = A13
  * 2A1 = /ROMSEL
  * PRG RAM /CE = 2/Y3



Proof: 

1A0 | 1A1 | 1/Y3 | 2A0 | 2A1 | 2/Y3   
---|---|---|---|---|---  
0 | x | 1 | x | x | 1   
1 | 0 | 1 | x | x | 1   
1 | 1 | 0 | 1 | x | 1   
1 | 1 | 0 | 0 | 1 | 1   
1 | 1 | 0 | 1 | 1 | 0   
  
See [further suggestions from kyuusaku](https://forums.nesdev.org/viewtopic.php?p=75739#p75739). 

The PlayChoice version of _Mike Tyson's Punch-Out!!_ uses an extra IC to add battery-backed RAM. The digits in existing photos are hard to read, but it is believed to be [a 74HC139](https://forums.nesdev.org/viewtopic.php?p=101088#p101088). Its wiring has not been traced. 

## /ROMSEL delay

One thing that can complicate adding PRG RAM to a board is the fact that /ROMSEL and M2, used together to decode $6000–$7FFF, do not change at the same time. /ROMSEL is the logical NAND of M2 and PRG A15. This is accomplished by sending M2 and PRG A15 into a [74LS139](74139.xhtml "74139") two-to-four line decoder on the NES main board. This introduces a small delay of up to 33 ns between the time M2 rises and the time /ROMSEL falls. 

If this delay is too long it can cause unintentional writes to PRG RAM when writing to mapper registers $E000-$FFFF. 

This is not a problem for the original cartridge hardware because the RAM chips used to require a /WE (Write Enable) pulse of at least 50ns to 70ns depending on the chip. This means that the spurious /WE signal generated by this delay (max. 33ns) will not be sufficient to trigger a write on the RAM chip. The circuits above give even more headroom as they tie PRG RAM /OE to ground and decode to /CE. The /CE to end of write timing is typically longer than the minimum /WE pulse width. 

If your RAMs are faster than these timing specifications, your decoding logic must delay M2 by about 33 ns to match the /ROMSEL delay, as in the 74139-based circuit shown above. In [this post](https://forums.nesdev.org/viewtopic.php?p=176078#p176078), lidnariq suggested adding a resistor and capacitor: 
    
    
    card edge M2 --- 1k --- + --- 7420
                            |
                           33pF
                            |
                           GND
    

### /ROMSEL delay issues

This delay is obtained by decreasing the voltage rise according to R·C. However, digital chips have minimum permissible rising speed, so the rise speed cannot be too slow because it might produce oscillation inside the digital input. Another option might be not to delay M2 but instead filter quick pulses on RAM /CE down by using capacitor connected to ground or a power supply: 
    
    
                          ______________________________________________________
    card edge M2 --------| combinatory circuit that outputs                     |----+-----|/CE
    card edge /ROMSEL ---| 0 when M2 = '1', /ROMSEL = '0', A14 = '1', A13 = '1' |    |     |
    card edge A14     ---| 1 otherwise                                          |    1nF   |   RAM 
    card edge A13 -------|______________________________________________________|    |     |_________
                                                                                 GND or +5V
    

If you latch data/address on the _falling_ edge of M2 of writes to $4020–$7FFF, you don't need to worry about this delay because /ROMSEL still has the correct logic level at this point. 

## Battery backup

Adding battery backup to RAM may be desired for maintaining data while the console is off. Most RAMs have a special low-power data retention mode, which decreases current consumption to few microamps, but the following need to be fulfilled: 

  * voltage supply in data retention mode "Vret" should be ≥ 2V (the "data retention voltage" in the datasheet)
  * with this schematic, the voltage supply must be ≤ ordinary power supply voltage Vcc
  * memory must become deselected before supply voltage drops from Vcc to Vret,
  * memory must remain deselected for the whole time Vret is supplied.


    
    
                        D1           ___________
        5V -------------|>|--+      |       RAM
                             |---+--| VCC
        3.3V battery----|>|--+   |  |
                        D2       R1 |
                                 |  |
       RAM /CE decoding logic----+--| /CE
                                    |___________
    

D1 ensures that the 3.3 V battery only powers the RAM, not the whole cartridge when in standby mode. D2 keeps the 5 V from charging the battery (permissible if the battery is rechargeable). R1 (100k should be enough) ties /CE at high level when power is cut off. 

Note that not all diodes are created equal. It is tempting to use Schottky diodes for their lower forward voltage drop. However, Schottky diodes generally have a high reverse leakage current. If D1 is a Schottky, the battery will drive some current backwards through this diode, attempting to power the ROMs and other chips on the cart. Though it isn't much current and it is hard to measure, it is still much more current than normal and the battery's life will be shortened. If D2 is a Schottky, the reverse leakage will attempt to charge the battery from the console's power. For a CR2032, the maximum charging current before damage is 1uA and the leakage may or may not exceed this. So, in general, you shouldn't use Schottky diodes. There are likely to be more exotic solutions or different diodes that are technically superior. Sticking to the ordinary 1N4148/1N914 may be the best advice though, especially if you are interested in maintaining the original spirit of simpler times in your design. 

If your RAM /CE decoding logic does not become high impedance in power off mode, the voltage at RAM /CE might drop after power down (for example, the AX5202P DIL40 pirate MMC3 chip seems to have its RAM /CE output at very low resistance with respect to ground when not powered, and the above circuit after power loss makes RAM /CE voltage drop to 1 V which leads to data corruption). To protect against that situation, RAM /CE decoding logic path must become open circuit after powering off. The following transistor is essentially a switch that does the job: 
    
    
                                     D1           ___________
        5V --------------------------|>|--+      |       RAM
                                          |---+--| VCC
        3.3V battery-----------------|>|--+   |  |
                                     D2       R1 |
                                              |  |
        RAM /CE decoding logic--- E   C ------+--| /CE
                                  \___/          |___________
                                    | B  NPN
        5V -----------------1k------+
    

When no 5V is present or 5V is present but RAM /CE decoding circuit outputs high level, transistor is open. When 5V is present and RAM /CE decoding circuit outputs low level, RAM /CE becomes low. 

See also: [Battery circuit schematic](https://forums.nesdev.org/viewtopic.php?p=87107#87107)

## References

  * Loopy pointed out the /ROMSEL delay [here](https://forums.nesdev.org/viewtopic.php?p=70539#p70539).
  * Further investigation performed in [this thread](https://forums.nesdev.org/viewtopic.php?t=7618).
  * [6264P-12 8Kx8 SRAM Data Sheet](https://www.jameco.com/Jameco/Products/ProdDS/42930.pdf)
  * PRG RAM decoding circuitry - problems [[5]](https://forums.nesdev.org/viewtopic.php?f=9&t=15868)


