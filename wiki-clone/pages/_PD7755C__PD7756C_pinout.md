# ΜPD7755C/µPD7756C pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/%CE%9CPD7755C/%C2%B5PD7756C_pinout) | View [other pages](Special_AllPages.xhtml#_PD7755C__PD7756C_pinout)

NEC µPD7755, µPD7756: 18-pin 0.3" DIP (used with mappers [18](INES_Mapper_018.xhtml "INES Mapper 018"), [72](INES_Mapper_072.xhtml "INES Mapper 072"), [86](INES_Mapper_086.xhtml "INES Mapper 086"), and [92](INES_Mapper_092.xhtml "INES Mapper 092")) 
    
    
             .--V--.
      I4  -> |01 18| <- I3
      I5  -> |02 17| <- I2
      I6  -> |03 16| <- I1
      I7  -> |04 15| <- I0
     Iref -> |05 14| <- /START
    SOUND <- |06 13| <- /CS
    /BUSY <- |07 12| <- X1
     /RST -> |08 11| -> X2
      Gnd -- |09 10| -- Vdd
             '-----'
    

The amount of current flowing into Iref specifies the amount of current flowing into the SOUND output when the DAC is set to 16 (1/32nd full scale). This is usually configured with an external resistor. 

X1 and X2 should be hooked up to a 640kHz ceramic resonator. 

Categories: [Pinouts](Category_Pinouts.xhtml)
