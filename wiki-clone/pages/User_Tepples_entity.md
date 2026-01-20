# User:Tepples/entity

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3ATepples/entity) | View [other pages](Special_AllPages.xhtml#User_Tepples_entity)

Look at the source to see what's going on. It's perfectly valid 6502 code, but because a constant is being bitwise ANDed at compile time with the constant `amp` (which here represents the "amplitude" field of APU [$4000, $4004](APU_Pulse.xhtml "APU Pulse"), and [$400C](APU_Noise.xhtml "APU Noise")) and followed by a comment, the MediaWiki parser is confusing it with an [character entity reference](https://en.wikipedia.org/wiki/character_entity_reference "wikipedia:character entity reference"). 

Space indented: 
    
    
    duty    = $C0
    envtype = $30
    amp     = $0F
    lda #$B7&mask off upper nibble
    

`<pre>` element: 
    
    
    duty    = $C0
    envtype = $30
    amp     = $0F
    lda #$B7&mask off upper nibble
    

`<source>` element: <source lang="6502"> duty = $C0 envtype = $30 amp = $0F lda #$B7&mask off upper nibble </source>
