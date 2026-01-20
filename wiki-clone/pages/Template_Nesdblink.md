# Template:Nesdblink

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Template%3ANesdblink) | View [other pages](Special_AllPages.xhtml#Template_Nesdblink)

[{{{3}}}](https://nescartdb.com/search/advanced?%7B%7B%7B1%7D%7D%7D=%7B%7B%7B2%7D%7D%7D)

  
See also: [Template:Nesdbbox](Template_Nesdbbox.xhtml "Template:Nesdbbox")

  


[![](../wiki-images/Template-info.svg)](File_Template_info_svg.xhtml) Template documentation

## Template:Nesdblink Documentation

A simple template for creating links to Bootgod's NES Cart Database. 

Usage: 
    
    
    {{nesdblink
     | search type (ines/unif/unif_wild)
     | search parameter (number/board)
     | link text
    }}

Examples: 
    
    
    1. {{nesdblink|ines|1|iNES 001}}
    2. {{nesdblink|unif|NES-CNROM|NES-CNROM}}
    3. {{nesdblink|unif_wild|CNROM|CNROM}}
    4. {{nesdblink|unif_wild|-A%ROM|AxROM}} - this currently doesn't work since the migration to nescartdb
    

  1. Search by iNES mapper.
  2. Search by specific PCB name.
  3. Search for PCB name containing term.
  4. Use % as a wildcard to include multiple variations. (Here the - prefix is to match NES**-A** O**ROM** and HVC**-A** O**ROM** but exclude T**A** ITO-CN**ROM**.)



|  The above [documentation](https://en.wikipedia.org/wiki/Wikipedia:Template_documentation "wikipedia:Wikipedia:Template documentation") is [transcluded](https://en.wikipedia.org/wiki/Wikipedia:Transclusion "wikipedia:Wikipedia:Transclusion") from [Template:Nesdblink/doc](Template_Nesdblink_doc.xhtml "Template:Nesdblink/doc"). ([edit](https://www.nesdev.org/w/index.php?title=Template:Nesdblink/doc&action=edit) | [history](https://www.nesdev.org/w/index.php?title=Template:Nesdblink/doc&action=history))   
Editors can experiment in this template's sandbox ([create](https://www.nesdev.org/w/index.php?title=Template:Nesdblink/sandbox&action=edit&preload=Template:Documentation/preload-sandbox)) and testcases ([create](https://www.nesdev.org/w/index.php?title=Template:Nesdblink/testcases&action=edit&preload=Template:Documentation/preload-testcases)) pages.   
Please add categories and interwikis to the [/doc](Template_Nesdblink_doc.xhtml "Template:Nesdblink/doc") subpage. ~~Subpages of this template~~.   
---|---
