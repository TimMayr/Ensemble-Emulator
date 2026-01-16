# Template:Infobox iNES mapper

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Template%3AInfobox_iNES_mapper) | View [other pages](Special_AllPages.xhtml#Template_Infobox_iNES_mapper)

**{{{name}}}**

**Company** | _unknown_  
---|---  
**Boards** | _unknown_  
**PRG ROM capacity** | {{{prgmax}}} 

  * **`{{Infobox iNES mapper}}` error: `prgmax` is not specified**

  
**PRG RAM capacity** | None   
**CHR capacity** | {{{chrmax}}} 

  * **`{{Infobox iNES mapper}}` error: `chrmax` is not specified**

  
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | Fixed H or V, controlled by solder pads   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | _unknown_  
**IRQ** | No   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | [[iNES Mapper {{{mapper}}}|{{{mapper}}}]]   
  
[![](../wiki-images/Template-info.svg)](File_Template_info_svg.xhtml) Template documentation

Copy and paste this at the top of a mapper page: 
    
    
    {{Infobox iNES mapper
    |name=MMC1
    |name2=SxROM
    |company=Nintendo, others
    |mapper=1
    |othermappers=[[iNES Mapper 155|155]]
    |nescartdbgames=69
    |complexity=ASIC
    |boards=SKROM, SLROM,<br/>SNROM, others
    |prgmax=512K
    |prgpage=16K + 16K fixed or 32K
    |wrammax=32K
    |wrampage=8K
    |chrmax=128K
    |chrpage=4K + 4K or 8K
    |mirroring=H, V, or 1, switchable
    |busconflicts=No
    |irq=No
    |audio=No
    }}
    

Categories: 

  * If you provide `nescartdbgames`, it automatically inserts [Category:In NesCartDB](Category_In_NesCartDB.xhtml "Category:In NesCartDB") and a link to a list of games on NesCartDB using the mapper provided in `mapper` (without regional duplicates). The argument need not be a number; it can be a string such as "Few" or "Many".
  * `complexity` needs to be `Discrete logic` (for [Category:Discrete logic mappers](Category_Discrete_logic_mappers.xhtml "Category:Discrete logic mappers")) or `ASIC` (for [Category:ASIC mappers](Category_ASIC_mappers.xhtml "Category:ASIC mappers")).
  * Page sizes are hidden if the corresponding max size is not specified or if `wrampage=Impossible` (which is used for mappers with ports in $6000-$7FFF).



To do: 

  * Do string processing on `busconflicts`, `irq`, and `audio` to trigger adding to categories such as [Category:Mappers with bus conflicts](Category_Mappers_with_bus_conflicts.xhtml "Category:Mappers with bus conflicts").



|  The above [documentation](https://en.wikipedia.org/wiki/Wikipedia:Template_documentation "wikipedia:Wikipedia:Template documentation") is [transcluded](https://en.wikipedia.org/wiki/Wikipedia:Transclusion "wikipedia:Wikipedia:Transclusion") from [Template:Infobox iNES mapper/doc](Template_Infobox_iNES_mapper_doc.xhtml "Template:Infobox iNES mapper/doc"). ([edit](https://www.nesdev.org/w/index.php?title=Template:Infobox_iNES_mapper/doc&action=edit) | [history](https://www.nesdev.org/w/index.php?title=Template:Infobox_iNES_mapper/doc&action=history))   
Editors can experiment in this template's sandbox ([create](https://www.nesdev.org/w/index.php?title=Template:Infobox_iNES_mapper/sandbox&action=edit&preload=Template:Documentation/preload-sandbox)) and [testcases](Template_Infobox_iNES_mapper_testcases.xhtml "Template:Infobox iNES mapper/testcases") ([edit](https://www.nesdev.org/w/index.php?title=Template:Infobox_iNES_mapper/testcases&action=edit)) pages.   
Please add categories and interwikis to the [/doc](Template_Infobox_iNES_mapper_doc.xhtml "Template:Infobox iNES mapper/doc") subpage. ~~Subpages of this template~~.   
---|---
