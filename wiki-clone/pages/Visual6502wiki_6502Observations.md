# Visual6502wiki/6502Observations

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Visual6502wiki/6502Observations) | View [other pages](Special_AllPages.xhtml#Visual6502wiki_6502Observations)

We've found some interesting things on the 6502, from the layout level, up through circuit level to the programmer visible level. 

# Programmer Visible

Notes here on bugs and undocumented behaviour. 

  * [BRK, the B bit](Visual6502wiki_6502_BRK_and_B_bit.xhtml "Visual6502wiki/6502 BRK and B bit"), and other interrupts
  * [Timing of Interrupt Handling](Visual6502wiki_6502_Timing_of_Interrupt_Handling.xhtml "Visual6502wiki/6502 Timing of Interrupt Handling") noting that a taken branch delays interrupt handling, also that CLI/PLP allow one further instruction to execute, unlike RTI.
  * [Unsupported or undocumented opcodes](Visual6502wiki_6502_Unsupported_Opcodes.xhtml "Visual6502wiki/6502 Unsupported Opcodes") such as SAX and XAA
  * [The ROR bug](https://www.pagetable.com/?p=406) which is found only in rare early devices



See also [our catalogue of 6502 test programs](Visual6502wiki_6502TestPrograms.xhtml "Visual6502wiki/6502TestPrograms"), useful to verify simulators or emulators. 

# Circuit and Logic

Notes here on timing fixes and non-digital circuit techniques, and departures from NMOS design style orthodoxy. 

  * [Signs of a fix](Visual6502wiki_6502_datapath_control_timing_fix.xhtml "Visual6502wiki/6502 datapath control timing fix") to datapath control timing



# Layout

Notes here on the traces of bug fixes, and remnants of the original 6501 layout. 

  * [Traces in the layout](Visual6502wiki_6502_traces_of_6501.xhtml "Visual6502wiki/6502 traces of 6501") of the original 6501 part which was withdrawn after legal wrangling


