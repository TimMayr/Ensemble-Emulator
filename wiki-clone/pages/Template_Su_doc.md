# Template:Su/doc

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Template%3ASu/doc) | View [other pages](Special_AllPages.xhtml#Template_Su_doc)

![](../wiki-images/Ambox_notice.png) |  **This is a[documentation](https://en.wikipedia.org/wiki/Wikipedia:Template_documentation "wikipedia:Wikipedia:Template documentation") [subpage](https://www.nesdev.org/w/index.php?title=Help:Subpages&action=edit&redlink=1 "Help:Subpages \(page does not exist\)") for [Template:Su](Template_Su.xhtml "Template:Su")** (see that page for the template itself).  
It contains usage information, [categories](https://www.nesdev.org/w/index.php?title=Help:Categories&action=edit&redlink=1 "Help:Categories \(page does not exist\)"), [interlanguage links](https://en.wikipedia.org/wiki/Help:Interlanguage_links "wikipedia:Help:Interlanguage links") and other content that is not part of the original template page.   
---|---  
  
## Contents

  * 1 Purpose
  * 2 Arguments
  * 3 Examples
  * 4 Line breaks
  * 5 See also



## Purpose

Template for creating two smaller lines of text on one actual line, this can be used for [scientific notations of uncertainty](https://www.nesdev.org/w/index.php?title=Template:Val&action=edit&redlink=1 "Template:Val \(page does not exist\)") and [physics](https://www.nesdev.org/w/index.php?title=Template:PhysicsParticle&action=edit&redlink=1 "Template:PhysicsParticle \(page does not exist\)") symbols, such as [nuclides](https://www.nesdev.org/w/index.php?title=Category:Nuclide_templates&action=edit&redlink=1 "Category:Nuclide templates \(page does not exist\)") and [particles](https://www.nesdev.org/w/index.php?title=Template:SubatomicParticle&action=edit&redlink=1 "Template:SubatomicParticle \(page does not exist\)"). 

It was named after the **< sup>** and **< sub>** HTML tags, which share the first two letters of their name with this template. This template uses the third letter of the name of these tags, **p** and **b** , as the name of the argument that drives the output of the top and bottom line, respectively. 

**Editors beware** : _these templates use all kinds of hacky tricks to make the output render correctly in a wide variety of browsers. If you plan to make changes to this template, please make sure you verify that this template renders correctly after those changes in all major browsers. This list includes the latest version(s) of Chrome, FireFox, MSIE, Opera and Safari as well as any version of those browsers that is still in common use, such as MSIE 6.0 and 7.0!_

## Arguments

**p** | The contents of the top line.   
---|---  
**b** | The contents of the bottom line.   
**a** | The alignment of both lines (**r** for right, **c** for center aligned, default is left aligned)   
**w** | The size of the characters (**f** for fixed width (monospace), default is whatever the current font is)   
  
## Examples

**Simple example** : `X{{su|p=a}}X{{su|b=b}}X{{su|p=a|b=b}}X`

    Xa  
X  
bXa  
bX

**Left aligned (default)** : `X{{su|p=aaaaa|b=b}}X{{su|p=a|b=bbbbb}}X`

    Xaaaaa  
bXa  
bbbbbX

**Right aligned:** `X{{su|a=r|p=aaaaa|b=b}}X{{su|a=r|p=a|b=bbbbb}}X`

    Xaaaaa  
bXa  
bbbbbX

**Center aligned:** `X{{su|a=c|p=aaaaa|b=b}}X{{su|a=c|p=a|b=bbbbb}}X`

    Xaaaaa  
bXa  
bbbbbX

**Smaller font** : `<small>X{{su|p=a}}X{{su|b=b}}X{{su|p=a|b=b}}X</small>`

    Xa  
X  
bXa  
bX

**Larger font** : `<big>X{{su|p=a}}X{{su|b=b}}X{{su|p=a|b=b}}X</big>`

    Xa  
X  
bXa  
bX

**Fixed width** : `X{{su|p=...|b=www}}X{{su|w=f|p=...|b=www}}X`

    X...  
wwwX...  
wwwX

## Line breaks

Unlike <sup> and <sub> tags, this template does not prevent line breaks between the two small lines and the preceding text. Thus, if this template is used to supply a subscript and superscript over a base symbol, the whole structure has to be enclosed in a `{{[nowrap](https://www.nesdev.org/w/index.php?title=Template:Nowrap&action=edit&redlink=1 "Template:Nowrap \(page does not exist\)")}}`, or this template should be invoked through another template that takes care of the wrapping in an appropriate way. 

**Example** : `{{nowrap|X{{su|p=a|b=b}}}}` → [Template:Nowrap](https://www.nesdev.org/w/index.php?title=Template:Nowrap&action=edit&redlink=1 "Template:Nowrap \(page does not exist\)")

## See also

  * `{{[sup](Template_Sup.xhtml "Template:Sup")}}` – superscript text
  * `{{[sub](Template_Sub.xhtml "Template:Sub")}}` – subscript text
  * `{{[e](https://www.nesdev.org/w/index.php?title=Template:E&action=edit&redlink=1 "Template:E \(page does not exist\)")}}` (1.23[Template:E](https://www.nesdev.org/w/index.php?title=Template:E&action=edit&redlink=1 "Template:E \(page does not exist\)"))
  * `{{[±](Template__.xhtml "Template:±")}}` (1.23+4  
−2)
  * [Subscript and superscript](https://www.nesdev.org/w/index.php?title=Subscript_and_superscript&action=edit&redlink=1 "Subscript and superscript \(page does not exist\)")
  * `{{[overunderset](https://www.nesdev.org/w/index.php?title=Template:Overunderset&action=edit&redlink=1 "Template:Overunderset \(page does not exist\)")}}` (or equivalently `{{[underoverset](https://www.nesdev.org/w/index.php?title=Template:Underoverset&action=edit&redlink=1 "Template:Underoverset \(page does not exist\)")}}`) – combines `{{[overset](https://www.nesdev.org/w/index.php?title=Template:Overset&action=edit&redlink=1 "Template:Overset \(page does not exist\)")}}` and `{{[underset](https://www.nesdev.org/w/index.php?title=Template:Underset&action=edit&redlink=1 "Template:Underset \(page does not exist\)")}}` for placing symbols above or below another symbol, rather than adjacent to it.



  


  


Categories: [Template documentation](Category_Template_documentation.xhtml)
