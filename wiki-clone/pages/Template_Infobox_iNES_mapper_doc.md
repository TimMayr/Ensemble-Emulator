# Template:Infobox iNES mapper/doc

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Template%3AInfobox_iNES_mapper/doc) | View [other pages](Special_AllPages.xhtml#Template_Infobox_iNES_mapper_doc)

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


