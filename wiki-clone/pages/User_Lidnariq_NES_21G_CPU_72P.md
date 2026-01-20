# User:Lidnariq/NES-21G-CPU-72P

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3ALidnariq/NES-21G-CPU-72P) | View [other pages](Special_AllPages.xhtml#User_Lidnariq_NES_21G_CPU_72P)

**NES-21G-CPU-72P** is an board used for some of Nintendo's test cartriges. As populated, it has: 

  * ① A CIC (for example, 3193A).
  * ⑤ A 28-pin 8KiB SRAM for CHR (for example, NEC 64S20)
  * ⑦ A 28-pin 32KiB UVEPROM for PRG (for example, Fujitsu MBM27256-25)
  * Permanently wired for vertical arrangement of nametables



This is noteworthy all on its own for being [NROM](NROM.xhtml "NROM") but with CHR RAM. 

Additionally, the SRAM uses both A13 and /A13 on its /CE and +CE inputs respectively. 

Photos of existing boards have chips with date stamps of: 9004, 9005, 9002; and 901X, 9041, and 9035. 

* * *

It is also the only Nintendo-made cartridge that ever implied a canonical direction for the EXP pins. 

On the cartridge are silkscreen only for a number of other parts: 

② 74HC04 

M2 and SYSTEM CLK proceed on traces to the 74HC04: 
    
    
    signal ---INV---C---+---|>|---+---+---INV--- output
                        |         |   |
            gnd---|>|---+         R   C
                                  |   |
                                 gnd gnd
    

"R" is given as 510k, the other component sizes are not specified on the silkscreen. 

These are voltage doublers, and are being used to detect if the inputs are oscillating. If the input is oscillating, the output is low. 

The SYSTEM CLK trace is cut. 

One inverter inverts R/W, and the last input is tied to ground. 

④ 74LS138 

Only the /Y7 output is used. CPU A13, A14, /ROMSEL, M2 must be high. A12 and A11 must be low. This decode a region at $6000-$67FF. 

③ 74LS32 

R/W, W/R from the inverter, and /Y7 are combined to produce two clocks. As wired, when the CPU reads from $6000 EXP6 is driven low, and writes to $6000 latch the current data bus in the '374. 

One OR gate combines the two voltage doubler outputs, so if either clock is not oscillating the output ("clocks bad") is high. 

The last OR gate combines Q7 from the '374 with the "clocks bad" signal and drives the output on EXP7. 

⑥ 74LS374 

The '374 latches the data bus D0-D7, and in order drives it on: EXP0, 1, 2, 3, 4, 9, 8, IRQ+EXP7 ("Q7"). 

* * *

Finally, a resistor R3 ("3.3K"), a capacitor C9 ("51P"), and a 3-pin part labelled "TRI" allow Q7 to drive /IRQ. 

Additionally, a blob of solder appears to be shorting the PRG UVEPROM's /CE and /OE pins permanently to ground. It is unclear how this works. 

* * *

See also: <http://kevtris.org/mappers/nes_custom/NES_21G_CPU_72P.html>
