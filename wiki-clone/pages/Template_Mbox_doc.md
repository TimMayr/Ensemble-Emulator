# Template:Mbox/doc

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Template%3AMbox/doc) | View [other pages](Special_AllPages.xhtml#Template_Mbox_doc)

![](../wiki-images/Ambox_notice.png) |  **This is a[documentation](https://en.wikipedia.org/wiki/Wikipedia:Template_documentation "wikipedia:Wikipedia:Template documentation") [subpage](https://www.nesdev.org/w/index.php?title=Help:Subpages&action=edit&redlink=1 "Help:Subpages \(page does not exist\)") for [Template:Mbox](Template_Mbox.xhtml "Template:Mbox")** (see that page for the template itself).  
It contains usage information, [categories](https://www.nesdev.org/w/index.php?title=Help:Categories&action=edit&redlink=1 "Help:Categories \(page does not exist\)"), [interlanguage links](https://en.wikipedia.org/wiki/Help:Interlanguage_links "wikipedia:Help:Interlanguage links") and other content that is not part of the original template page.   
---|---  
  
This is the `{{[mbox](Template_Mbox.xhtml "Template:Mbox")}}` or **message box** meta-template. 

It is used to build _message box_ templates such as `{{[security](https://www.nesdev.org/w/index.php?title=Template:Security&action=edit&redlink=1 "Template:Security \(page does not exist\)")}}`. It offers several different colours, uses default images if no image parameter is given and it has some other features. 

This meta-template uses the mbox CSS classes in [MediaWiki:Common.css](MediaWiki_Common_css.xhtml "MediaWiki:Common.css"). The classes can also be used directly in a [wikitable](https://www.nesdev.org/w/index.php?title=Help:Table&action=edit&redlink=1 "Help:Table \(page does not exist\)") if special functionality is needed. See the [how-to guide](https://en.wikipedia.org/wiki/Wikipedia:Ambox_CSS_classes "wikipedia:Wikipedia:Ambox CSS classes") for that. 

## Contents

  * 1 Usage
  * 2 Message box types
  * 3 Other images
  * 4 More examples
  * 5 Parameters
    * 5.1 The small parameters
  * 6 Technical details
  * 7 See also



### Usage

Simple usage example: 
    
    
    {{mbox | text = Some text.}}
    

![](../wiki-images/Ambox_notice.png) |  Some text.   
---|---  
  
Complex example: 
    
    
    {{mbox
    | type       = style
    | small      = left
    | image      = [[File:Emblem-question-yellow.svg|40px|alt=Question mark]]
    | smallimage = [[File:Emblem-question-yellow.svg|20px|alt=Question mark]]
    | textstyle  = color: red; font-weight: bold; font-style: italic;
    | text       = Text for a big box, for the top of articles.
    | smalltext  = Text for the top of article sections.
    }}
    

[![Question mark](../wiki-images/Emblem-question-yellow.svg)](File_Emblem_question_yellow_svg.xhtml) |  Text for the top of article sections.   
---|---  
  
But you are not really supposed to use red bold italic text. 

### Message box types

The following examples use different **type** parameters but use no image parameters thus they use the default images for each type. 

![](../wiki-images/Ambox_speedy_deletion.png) |  type=_critical_ – Urgent warnings, such as `{{[XSS alert](https://www.nesdev.org/w/index.php?title=Template:XSS_alert&action=edit&redlink=1 "Template:XSS alert \(page does not exist\)")}}`.   
---|---  
![](../wiki-images/Ambox_deletion.png) |  type=_important_ – Serious problems, such as `{{[delete](Template_Delete.xhtml "Template:Delete")}}`.   
---|---  
![](../wiki-images/Ambox_content.png) |  type=_warning_ – Other problems, such as `{{[deprecated](https://www.nesdev.org/w/index.php?title=Template:Deprecated&action=edit&redlink=1 "Template:Deprecated \(page does not exist\)")}}`.   
---|---  
![](../wiki-images/Edit-clear.svg) |  type=_caution_ – Points of concern, but not so serious, such as `{{[description missing](https://www.nesdev.org/w/index.php?title=Template:Description_missing&action=edit&redlink=1 "Template:Description missing \(page does not exist\)")}}` and `{{[inuse](https://www.nesdev.org/w/index.php?title=Template:Inuse&action=edit&redlink=1 "Template:Inuse \(page does not exist\)")}}`.   
---|---  
![](../wiki-images/Ambox_notice.png) |  type=_style_ – Points of information, such as `{{[oldupgradenotes](https://www.nesdev.org/w/index.php?title=Template:Oldupgradenotes&action=edit&redlink=1 "Template:Oldupgradenotes \(page does not exist\)")}}` and `{{[copyrightNotice](https://www.nesdev.org/w/index.php?title=Template:CopyrightNotice&action=edit&redlink=1 "Template:CopyrightNotice \(page does not exist\)")}}`.   
---|---  
![](../wiki-images/Ambox_notice.png) |  type=_(blank)_ – Default; general 'box'.   
---|---  
![](../wiki-images/Ambox_move.png) |  type=_move_ – Merge, split and transwiki proposals, such as `{{[split](https://www.nesdev.org/w/index.php?title=Template:Split&action=edit&redlink=1 "Template:Split \(page does not exist\)")}}` and `{{[MoveToCommons](https://www.nesdev.org/w/index.php?title=Template:MoveToCommons&action=edit&redlink=1 "Template:MoveToCommons \(page does not exist\)")}}`.   
---|---  
![](../wiki-images/Ambox_protection.png) |  type=_protection_ – Protection notices..   
---|---  
  
### Other images

The default images shown above are mostly for convenience. In many cases it is more appropriate to use more specific images. These examples use the **image** parameter to specify an image other than the default images. 

![](../wiki-images/Unbalanced_scales.svg) |  type = content   
image = [[File:Unbalanced scales.svg|40px|link=|alt=]]   
This image is often used for `{{[POV](https://www.nesdev.org/w/index.php?title=Template:POV&action=edit&redlink=1 "Template:POV \(page does not exist\)")}}` and similar issues.   
---|---  
![](../wiki-images/Wikitext.svg) |  type = style   
image = [[File:Wikitext.svg|50px|link=|alt=]]   
This image is often used for `{{[wikify](https://www.nesdev.org/w/index.php?title=Template:Wikify&action=edit&redlink=1 "Template:Wikify \(page does not exist\)")}}` etc.   
---|---  
![](../wiki-images/Merge-arrows.svg) |  type = move   
image = [[File:Merge-arrows.svg|50px|link=|alt=]]   
This image is used for `{{[merge](Template_Merge.xhtml "Template:Merge")}}` etc.   
---|---  
  
### More examples

Some other parameter combinations. 

![](../wiki-images/Ambox_notice.png) |  No type and no image given (**default**)   
---|---  
|  No type and **image=none** – No image is used and the **text** uses the whole message box area.   
---|---  
[![Clock over a larger globe](../wiki-images/Gnome_globe_current_event.svg)](File_Gnome_globe_current_event_svg.xhtml) |  image = [[File:Gnome globe current event.svg|42px|alt=Clock over a larger globe]]   
imageright = [[File:Nuvola apps bookcase.svg|40px|alt=Three stacked books]]  |  [![Three stacked books](../wiki-images/Nuvola_apps_bookcase.svg)](File_Nuvola_apps_bookcase_svg.xhtml)  
---|---|---  
[![Clock over a larger globe](../wiki-images/Gnome_globe_current_event.svg)](File_Gnome_globe_current_event_svg.xhtml) |  **This article or section documents a current[spaceflight](https://www.nesdev.org/w/index.php?title=Spaceflight&action=edit&redlink=1 "Spaceflight \(page does not exist\)").**   
Content may change as the mission progresses.  |  ![](../wiki-images/Shuttle.svg)  
---|---|---  
  
### Parameters

List of all parameters: 
    
    
    {{mbox
    | type  = critical / serious / warning / notice / / move / protection
    | image = none / [[File:...|40px|...]]
    | imageright = [[File:...|40px|...]]
    | style = CSS values
    | textstyle = CSS values
    | text  = The message body text.
    | small = {{{small|}}} / left
    | smallimage = none / [[File:...|20px|...]]
    | smallimageright = none / [[File:...|20px|...]]
    | smalltext  = A shorter message body text.
    }}
    

**type**

    If no **type** parameter is given the template has a 'blank' style.

**image**

    **No parameter** = If no **image** parameter is given the template uses a default image. Which default image it uses depends on the **type** parameter.
    **An image** = Should be an image with usual wiki notation. Widths of 40px - 50px are usually about right. (Images over 52 pixels wide will cause padding problems.) 

    Often an icon is [purely decorative](https://en.wikipedia.org/wiki/Alternative_text_for_images#Purely_decorative_images "wikipedia:Alternative text for images") in the W3C sense that it repeats the text. To improve [accessibility](https://en.wikipedia.org/wiki/Accessibility "wikipedia:Accessibility"), it is desirable to not have it be announced by [screen readers](https://www.nesdev.org/w/index.php?title=Screen_reader&action=edit&redlink=1 "Screen reader \(page does not exist\)"), as well as to avoid it linking to an irrelevant page. If (and only if) the image license allows this, it can be marked with "`|link=``|alt=`". For example: 

    `image = [[File:Unbalanced scales.svg|40px|link=|alt=]]`
    Conversely, an icon that does not use "`|link=``|alt=`", and which therefore is announced to visually impaired readers, should use an "`|alt=[alt text](https://en.wikipedia.org/wiki/Alternative_text_for_images "wikipedia:Alternative text for images")`" parameter that describes the icon. With no `|link=` parameter (using the default link), the alt text should describe the icon's visual appearance. For example: 

    `image = [[File:Gnome globe current event.svg|40px|alt=Clock over a larger globe]]`
    With a nonempty "`|link=Page`" the alt text should describe the icon's function. For example: 

    `image = [[File:Purple question mark.svg|40px|link=Special:Random|alt=Random article]]`
    An icon whose license requires attribution may have alt text, but _must keep the default link_. Although public domain images do not require a link, many licenses do require one. Please see _[Purely decorative images](https://en.wikipedia.org/wiki/Alternative_text_for_images#Purely_decorative_images "wikipedia:Alternative text for images")_ for more information about licensing.
    **none** = Means that no image is used.
    ~~**blank**~~ = This parameter is now deprecated. If you see it in use, change it to "image=none".

**imageright**

    **No parameter** = If no **imageright** parameter is given then no image is shown on the right side.
    **An image** = Should be an image with usual wiki notation. 40px - 50px width are usually about right depending on the image height to width ratio. (Images over 52 pixels width will cause padding problems.) For example: 

    `imageright = [[File:Nuvola apps bookcase.png|40px|alt=Three stacked books]]`
    **Anything** = Any other object that you want to show on the right side.

**style**

    Optional [CSS](https://www.nesdev.org/w/index.php?title=Cascading_Style_Sheets&action=edit&redlink=1 "Cascading Style Sheets \(page does not exist\)") values used by the entire message box table. Without quotation marks `" "` but with the ending semicolons `;`. For example: 

    `style = margin-bottom: 0.5em;`

**textstyle**

    Optional [CSS](https://www.nesdev.org/w/index.php?title=Cascading_Style_Sheets&action=edit&redlink=1 "Cascading Style Sheets \(page does not exist\)") values used by the text cell. For example: 

    `textstyle = text-align: center;`

**text**

    The message body text.

#### The small parameters

The small article message boxes are meant for the top of sections. Normally they should only contain one or two lines of text. 

**small**

    **left** = Makes it a smaller left aligned message box. This also makes the default images smaller. Note that any data fed to the **smallimage** , **smallimageright** and **smalltext** parameters is only used if "small=left". To make it so your template also understands the small parameter you can use this code: 

    `small = {{{small|}}}`
![](../wiki-images/Ambox_notice.png) |  small = left   
---|---  
![](../wiki-images/Ambox_notice.png) |  type = style   
small = left   
---|---  
  
**smallimage**

    **No parameter** = If no **smallimage** parameter is given then this template falls back to use the **image** parameter. If the **image** parameter also is empty then a small default image is used.
    **An image** = Should be an image with usual wiki notation. 20px width is usually about right for boxes with one line of text, while 25px width is usually about right for boxes with two lines of text. For example: 

    `smallimage = [[File:Gnome globe current event.svg|20px|alt=Clock over a larger globe]]`
    **none** = Means that no image is used. This overrides any image fed to **image** , when "small=left".
![](../wiki-images/Replacement_filing_cabinet.svg) |  small = left   
image = [[File:Replacement filing cabinet.svg|50px|link=|alt=]]   
smallimage = [[File:Replacement filing cabinet.svg|25px|link=|alt=]]   
---|---  
  
**smallimageright**

    **No parameter** = If no **smallimageright** parameter is given then this template falls back to use the **imageright** parameter. If the **imageright** parameter also is empty then no image is shown on the right side.
    **An image** = Should be an image with usual wiki notation. 20px - 25px width is usually about right. For example: 

    `smallimageright = [[File:Nuvola apps bookcase.png|20px|alt=Three stacked books]]`
    **Anything** = Any other object that you want to show on the right side.
    **none** = Means that no right side image is used. This overrides any image fed to **imageright** , when "small=left".
![](../wiki-images/Ambox_notice.png) |  small = left   
imageright = [[File:Gnome globe current event.svg|50px|alt=Clock over a larger globe]]   
smallimageright = none   
---|---  
  
**smalltext**

    A shorter version of the message body text. If no **smalltext** parameter is given then this template falls back to use the **text** parameter.

### Technical details

If you need to use special characters in the text parameter then you need to escape them like this: 
    
    
    {{mbox
    | text  = <div>
    Equal sign = and a start and end brace { } work fine as they are.
    But here is a pipe &#124; and two end braces <nowiki>}}</nowiki>.
    And now a pipe and end braces <nowiki>|}}</nowiki>.
    </div>
    }}
    

![](../wiki-images/Ambox_notice.png) |  Equal sign = and a start and end brace { } work fine as they are. But here is a pipe | and two end braces }}. And now a pipe and end braces |}}.   
---|---  
  
The `<div>` tags that surround the text in the example above are usually not needed. But if the text contains line breaks then sometimes we get weird line spacing. This especially happens when using vertical dotted lists. Then use the div tags to fix that. 

This template uses CSS classes in [MediaWiki:Common.css](MediaWiki_Common_css.xhtml "MediaWiki:Common.css") for most of its looks, thus it is fully skinnable. 

This template calls `{{[mbox/core](Template_Mbox_core.xhtml "Template:Mbox/core")}}` which holds most of the code for `{{[mbox](Template_Mbox.xhtml "Template:Mbox")}}`, while `{{[mbox](Template_Mbox.xhtml "Template:Mbox")}}` itself does parameter preprocessing. 

Internally this meta-template uses HTML wikimarkup instead of wikimarkup for the table code. That is the usual way we make meta-templates since wikimarkup has several drawbacks. For instance it makes it harder to use [parser functions](https://www.nesdev.org/w/index.php?title=M:Help:ParserFunctions&action=edit&redlink=1 "M:Help:ParserFunctions \(page does not exist\)") and some special characters in parameters. 

The default images for this meta-template are in png format instead of svg format. The main reason is that some older web browsers have trouble with the transparent background that MediaWiki renders for svg images. The png images here have hand optimised transparent background colour so they look good in all browsers. Note that svg icons only look somewhat bad in the old browsers, thus such hand optimisation is only worth the trouble for very widely used icons. 

For more technical details see the [talk page](https://www.nesdev.org/w/index.php?title=Template_talk:Mbox&action=edit&redlink=1 "Template talk:Mbox \(page does not exist\)") and the "See also" links below. 

### See also

Other pages: 

  * [wikipedia:Wikipedia:Ambox CSS classes](https://en.wikipedia.org/wiki/Wikipedia:Ambox_CSS_classes "wikipedia:Wikipedia:Ambox CSS classes") – Describes how to use the ambox CSS classes directly in [wikitables](https://www.nesdev.org/w/index.php?title=Help:Table&action=edit&redlink=1 "Help:Table \(page does not exist\)") and [HTML tables](https://en.wikipedia.org/wiki/HTML_element#Tables "wikipedia:HTML element").
  * [wikipedia:Wikipedia:Article message boxes](https://en.wikipedia.org/wiki/Wikipedia:Article_message_boxes "wikipedia:Wikipedia:Article message boxes") – The style guideline for creating article message boxes.
  * [wikipedia:Wikipedia talk:Article message boxes](https://en.wikipedia.org/wiki/Wikipedia_talk:Article_message_boxes "wikipedia:Wikipedia talk:Article message boxes") – For discussion about these matters.



  


Categories: [Template documentation](Category_Template_documentation.xhtml)
