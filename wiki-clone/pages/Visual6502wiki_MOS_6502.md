# Visual6502wiki/MOS 6502

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Visual6502wiki/MOS_6502) | View [other pages](Special_AllPages.xhtml#Visual6502wiki_MOS_6502)

## Contents

  * 1 MOS 6502 family
  * 2 Our Analysis
  * 3 6502 additional information
    * 3.1 Primary Sources
    * 3.2 Secondary Sources
    * 3.3 Previous Analysis



## MOS 6502 family

We have [photographs of the metal and lower layers](Visual6502wiki_Photos_of_MOS_6502D.xhtml "Visual6502wiki/Photos of MOS 6502D"), the polygons captured, the circuit extracted and we have published [a javascript simulator](http://visual6502.org/JSSim). There is an [FPGA project](https://github.com/pmonta/FPGA-netlist-tools) to implement the simulation in hardware. We have (as yet) unpublished simulators in python and C. 

## Our Analysis

Here are some relatively raw materials we've collected: 

  * Comparative photos of the [Stack Register](Visual6502wiki_6502_Stack_Register_High_Bits.xhtml "Visual6502wiki/6502 Stack Register High Bits") in 6502 and 6507
  * The [Decode ROM](Visual6502wiki_6507_Decode_ROM.xhtml "Visual6502wiki/6507 Decode ROM") (describing the Atari 6507, not exactly the same as the NMOS 6502 used in the [visual6502 simulator](http://visual6502.org/JSSim)
  * [All 256 6502 opcodes](Visual6502wiki_6502_all_256_Opcodes.xhtml "Visual6502wiki/6502 all 256 Opcodes") named and tabulated



And here is some more interpretive material from our explorations: 

  * Collected [observations](Visual6502wiki_6502Observations.xhtml "Visual6502wiki/6502Observations") of 6502 layout and behaviour.
  * The 6502 [datapath timing](Visual6502wiki_6502_datapath.xhtml "Visual6502wiki/6502 datapath")
  * The [unsupported opcodes](Visual6502wiki_6502_Unsupported_Opcodes.xhtml "Visual6502wiki/6502 Unsupported Opcodes")
    * a detailed explanation of the [XAA opcode](Visual6502wiki_6502_Opcode_8B__XAA__ANE_.xhtml "Visual6502wiki/6502 Opcode 8B \(XAA, ANE\)") behaviour
  * [Implementing a realtime netlist simulation](Visual6502wiki_6502___simulating_in_real_time_on_an_FPGA.xhtml "Visual6502wiki/6502 - simulating in real time on an FPGA") in historical systems using an FPGA



## 6502 additional information

See also the [links](http://visual6502.org/links.html) page on the main site. 

### Primary Sources

  * [Visual6502wiki/Photos of MOS 6502D](Visual6502wiki_Photos_of_MOS_6502D.xhtml "Visual6502wiki/Photos of MOS 6502D") (also see our [website](http://visual6502.org/images/6502/index.html))
  * [Visual6502wiki/Atari's 6507 Schematics](Visual6502wiki_Atari_s_6507_Schematics.xhtml "Visual6502wiki/Atari's 6507 Schematics")
  * [Visual6502wiki/Photos of R6502](Visual6502wiki_Photos_of_R6502.xhtml "Visual6502wiki/Photos of R6502")



### Secondary Sources

  * [Visual6502wiki/Hanson's Block Diagram](Visual6502wiki_Hanson_s_Block_Diagram.xhtml "Visual6502wiki/Hanson's Block Diagram")
  * [Visual6502wiki/Balazs' schematic and documents](Visual6502wiki_Balazs__schematic_and_documents.xhtml "Visual6502wiki/Balazs' schematic and documents")



### Previous Analysis

  * [Beregnyei Balazs: 6502 Reverse Engineering](http://impulzus.sch.bme.hu/6502/letolt.php3) ([translation](http://www.downloads.reactivemicro.com/Public/Electronics/Reverse%20Engineering/))
  * [Mark Ormston: 65xx Processor Data (version 0.2b)](http://anyplatform.net/media/guides/cpus/65xx%20Processor%20Data.txt)
  * [Ivo van Poorten: 6502 Bugs List](http://www.textfiles.com/apple/6502.bugs.txt)
  * [Neil Parker: The 6502/65C02/65C816 Instruction Set Decoded](http://www.llx.com/~nparker/a2/opcodes.html)
  * [Graham: 6502/6510/8500/8502 Opcode matrix](http://www.oxyron.de/html/opcodes02.html)
  * [Freddy Offenga: 6502 Undocumented Opcodes](http://members.chello.nl/taf.offenga/illopc31.txt)
  * [Adam Vardy: Extra Instructions Of The 65XX Series CPU](http://www.zimmers.net/anonftp/pub/cbm/documents/chipdata/6502-NMOS.extra.opcodes)


