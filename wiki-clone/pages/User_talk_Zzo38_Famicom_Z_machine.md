# User talk:Zzo38/Famicom Z-machine

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User_talk%3AZzo38/Famicom_Z-machine) | View [other pages](Special_AllPages.xhtml#User_talk_Zzo38_Famicom_Z_machine)

## Bug report

  * The implementation of PRINTC seems to be wrong (I think `zprntc1` and `zprntc2` are reversed from what it should be)
  * DIV and READ and SAVE and RESTORE are not implemented yet (I believe I haven't missed any others)
  * Calls to `textba` and `textwa` could be tail call optimized (maybe?)
  * It might be possible to save one cycle by avoiding RTS trick for instruction dispatch?
  * Rearrange code to avoid conditional branches across pages (probably should not be optimized prematurely)


