# Template:Mapper

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Template%3AMapper) | View [other pages](Special_AllPages.xhtml#Template_Mapper)

[[INES Mapper {{{1}}}|{{{1}}}]] 

## Template

This template is intended for easy inserting of links to mapper pages, with optional icon _(for latter feature it's required that the icon exists on /Icon subpage of mapper page; this isn't done yet for all mappers)_. 

## Usage

`{{mapper|_mapper number_ |_include icon?_}}`

code  | rendered as:   
---|---  
`{{mapper|XXX}}` | [XXX](https://www.nesdev.org/w/index.php?title=INES_Mapper_XXX&action=edit&redlink=1 "INES Mapper XXX \(page does not exist\)")  
`{{mapper|XXX|i}}` | [![Template Mapper Noicon.png](../wiki-images/Template_Mapper_Noicon.png)](INES_Mapper_XXX_Icon.xhtml "INES Mapper XXX/Icon") [XXX](https://www.nesdev.org/w/index.php?title=INES_Mapper_XXX&action=edit&redlink=1 "INES Mapper XXX \(page does not exist\)")  
`{{mapper|19|}}` | [019](INES_Mapper_019.xhtml "INES Mapper 019")  
`{{mapper|0|i}}` | [![Mfr icon Nintendo.png](../wiki-images/Mfr_icon_Nintendo.png)](File_Mfr_icon_Nintendo_png.xhtml) [000](NROM.xhtml "INES Mapper 000")  
`{{mapper|0|anything}}` | [![Mfr icon Nintendo.png](../wiki-images/Mfr_icon_Nintendo.png)](File_Mfr_icon_Nintendo_png.xhtml) [000](NROM.xhtml "INES Mapper 000")
