# Template:Nesdblink/doc

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Template%3ANesdblink/doc) | View [other pages](Special_AllPages.xhtml#Template_Nesdblink_doc)

## [Template:Nesdblink](Template_Nesdblink.xhtml "Template:Nesdblink") Documentation

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


