# FDS expansion port pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/FDS_expansion_port_pinout) | View [other pages](Special_AllPages.xhtml#FDS_expansion_port_pinout)

The [FDS](Family_Computer_Disk_System.xhtml "FDS") has an expansion port located at the rear of the RAM adapter, covered by a shutter. (Labelled as _Expansion Port B_ on the Twin Famicom) The unreleased [Famicom Network Adapter](Family_Computer_Network_Adapter.xhtml "Family Computer Network Adapter") was designed to use this port. An unlicensed device called the _ILine-PC_ (Iライン-PC) is known to connect this port to a PC. 

## Diagram

Open end view of the external connector. 
    
    
             -------
             |  --  \
      +5V -- | 1||2  | <> EXT6
     EXT5 <> | 3||4  | <> EXT4
     EXT3 <> | 5||6  | <> EXT2
     EXT1 <> | 7||8  | <> EXT0
    SOUND <- | 9||10 | -- GND
             |  --  /
             -------
    

## Signal descriptions

  * **+5V** : 5V Power supply from the main voltage regulator.
  * **GND** : 0V Power supply.
  * **EXT0..6** : Bidirectional - outputs from $4026.D0..D6 and inputs to $4033.D0..D6. (open-collector with 4.7K ohm pull-ups) 
    * EXT1..2 are altered when $4025.D5 = 0 but its purpose is unknown.
  * **SOUND** : Analog audio output, after the [FDS audio](FDS_audio.xhtml "FDS audio") is mixed in.



## References

  * <https://web.archive.org/web/20140920224500/green.ap.teacup.com/junker/119.html> ([PDF rehosted on forum](https://forums.nesdev.org/viewtopic.php?t=25219))
  * [Forum thread](https://forums.nesdev.org/viewtopic.php?t=25882): I2 ILine-PC



Categories: [Pinouts](Category_Pinouts.xhtml)
