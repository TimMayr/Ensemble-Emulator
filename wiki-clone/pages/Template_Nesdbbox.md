# Template:Nesdbbox

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Template%3ANesdbbox) | View [other pages](Special_AllPages.xhtml#Template_Nesdbbox)

**NESCartDB**

[{{{3}}}](https://nescartdb.com/search/advanced?%7B%7B%7B1%7D%7D%7D=%7B%7B%7B2%7D%7D%7D)  
---  
  
A box containing the [In NesCartDB](Category_In_NesCartDB.xhtml "Category:In NesCartDB") category tag and 1 or more links to NesDB searches. 

Single use: 
    
    
    {{nesdbbox|ines|1|iNES 001}}

Multi use: 
    
    
    {{nesdbbox
    |ines|1|iNES 001
    |unif|NES-CNROM|NES-CNROM
    |unif_wild|AOROM|AOROM
    |unif_wild|-A%ROM|AxROM
    }}

The parameters are simply a batched set of [nesdblink](Template_Nesdblink.xhtml "Template:Nesdblink") templates, documentation included below. 

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



Categories: [In NesCartDB](Category_In_NesCartDB.xhtml)
