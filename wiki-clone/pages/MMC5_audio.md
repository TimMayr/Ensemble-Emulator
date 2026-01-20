# MMC5 audio

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/MMC5_audio) | View [other pages](Special_AllPages.xhtml#MMC5_audio)

Nintendo's [MMC5](MMC5.xhtml "MMC5") mapper provides extra sound output, consisting of two pulse wave channels and a PCM channel. The pulse wave channels behave almost identically to the native pulse channels in the [NES APU](APU.xhtml "APU"). 

The sound output of the square channels are equivalent in volume to the corresponding APU channels, but the polarity of all MMC5 channels is reversed compared to the APU. 

The PCM channel is similarly equivalent in volume to the APU with equivalent input, and inverted, but using the extra high bit of input it can become twice as loud. 

## Contents

  * 1 Pulse 1 ($5000-$5003)
  * 2 Pulse 2 ($5004-$5007)
  * 3 PCM Mode/IRQ ($5010)
    * 3.1 Write
    * 3.2 Read
  * 4 Raw PCM ($5011)
    * 4.1 Write
    * 4.2 Pin 2 DAC Characteristic
  * 5 PCM description
    * 5.1 IRQ operation
  * 6 Status ($5015, read/write)
  * 7 Hardware
  * 8 References



## Pulse 1 ($5000-$5003)

These registers manipulate the MMC5's first [pulse wave](APU_Pulse.xhtml "APU Pulse") channel, which functions the same as to those found in the [NES APU](APU.xhtml "APU") except for the following differences: 

  * $5001 has no effect. The MMC5 pulse channels will not sweep, as they have no [sweep unit](APU_Sweep.xhtml "APU Sweep").
  * Frequency values less than 8 do not silence the MMC5 pulse channels; they can output ultrasonic frequencies.
  * Length counter operates twice as fast as the APU length counter (might be clocked at the envelope rate).
  * MMC5 does not have an equivalent [frame sequencer](APU_Frame_Counter.xhtml "APU Frame Counter") (APU $4017); envelope and length counter are fixed to a 240hz update rate.



Other features such as the envelope and phase reset are the same as their APU counterparts. 

## Pulse 2 ($5004-$5007)

These registers manipulate the MMC5's second pulse channel. 

## PCM Mode/IRQ ($5010)

### Write
    
    
    7  bit  0
    ---- ----
    Ixxx xxxM
    |       |
    |       +- Mode select (0 = write mode. 1 = read mode.)
    +--------- PCM IRQ enable (1 = enabled.)
    

### Read
    
    
    7  bit  0
    ---- ----
    Ixxx xxxM  MMC5A default power-on read value = $01
    |       |
    |       +- In theory but not verified: Read back of mode select (0 = write mode. 1 = read mode.)
    +--------- IRQ (0 = No IRQ triggered. 1 = IRQ was triggered.) Reading $5010 acknowledges the IRQ and clears this flag.
    

## Raw PCM ($5011)

This functions similarly to the [NES APU](APU.xhtml "APU")'s register $4011, except that all 8 bits are used. 

_Shin 4 Nin Uchi Mahjong_ is the only game to uses the extra PCM channel ($5011). 

### Write

Writes are ignored in PCM read-mode. 
    
    
    7  bit  0
    ---- ----
    WWWW WWWW
    |||| ||||
    ++++-++++- 8-bit PCM data
    

Writing $00 to this register will have no effect on the output sound, and does not change the PCM counter. 

### Pin 2 DAC Characteristic

Pin 2 no-load voltage very closely follows the equation: 

Voltage = [(DAC value / 255) * (0.4 * AVcc)] + (0.1 * AVcc) 

[![MMC5 DAC Characteristic.png](../wiki-images/MMC5_DAC_Characteristic.png)](File_MMC5_DAC_Characteristic_png.xhtml)

The DAC output is very low impedance -- it holds its voltage true under load. To protect itself, the output also has a current limit. The current limit level depends on AVcc. MMC5A measured current limit in microAmps = (42.178 * Avcc) - 36.808. It is unknown, but quite doubtful, that prolonged exposure to current limit mode could cause damage to the DAC or other parts of the MMC5. Since current limit mode would cause audible distortion, it should be considered a self-protection feature that should be avoided. In practice, with Avcc = 5V, the DAC output pin should drive 15kohms or higher to avoid current limit mode. 

The same MMC5A chip, tested with repeated power cycles on different days of the week and different moon phases, etc, has been observed to power on with a DAC voltage equivalent to DAC value $EF or $FF. No other power-on values have been observed. This voltage is not affected by a reset detected when M2 stops toggling. 

## PCM description

MMC5's DAC is changed either by writing a value to $5011 (in write mode) or reading a value from $8000-BFFF (in read mode). If you try to assign a value of $00, the DAC is not changed; an IRQ is generated instead. This could be used to read stream 8-bit PCM from ROM and terminate at $00. 

### IRQ operation

(pseudocode) 
    
    
    (On DAC write)
        if(value=0)
            irqTrip=1
        else
            irqTrip=0
    
    (On $5010 write)
        irqEnable=value.bit7
    
    (On $5010 read)
        value.bit7=(irqTrip AND irqEnable)
        irqTrip=0
    
    Cart IRQ line=(irqTrip AND irqEnable)
    

## Status ($5015, read/write)

This register is analogous to the [APU Status](APU.xhtml "APU Status") register found within the NES at $4015, except that only the bottom 2 bits are used; being for the MMC5's two pulse channels. The MMC5A default power-on value read from this register is $00. 

## Hardware

Expansion audio hardware configuration from HVC-ETROM-02: 
    
    
                R3 15k   C4 100n
    From 2A03 ---/\/\/-----||------+           R4 10k
    (Cart.45)                      |   +--------/\/\/---------+
                                   |   |           _          |
                R2 15k   C3 100n   |   | (MMC5.1) | \         +--- To RF
    MMC5 DAC ----/\/\/-----||------+---+----------|+  \       |    (Cart.46)
    (MMC5.2)                       |              |    >------+
                                   |       GND ---|-  / (MMC5.100)
                R1 15k   C2 100n   |              |_/
    MMC5 Pulse --/\/\/-----||------+
    (MMC5.3)
    

## References

  * [Famicom expansion hardware recordings](https://forums.nesdev.org/viewtopic.php?p=90245#p90245) \- forum thread with MMC5 test ROMs and recordings confirming various MMC5 audio features.
  * [MMC5 volume test](https://forums.nesdev.org/viewtopic.php?p=124288#p124288) \- hotswap test ROM for investigating MMC5 volume.



Categories: [Expansion audio](Category_Expansion_audio.xhtml)
