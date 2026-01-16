# 6 in 1 (MMC3 + CNROM + NROM)

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/6_in_1_%28MMC3_%2B_CNROM_%2B_NROM%29) | View [other pages](Special_AllPages.xhtml#6_in_1__MMC3___CNROM___NROM_)

[![6-in-1 MMC3+CNROM menu.png](../wiki-images/6-in-1_MMC3%2BCNROM_menu.png)](File_6_in_1_MMC3_CNROM_menu_png.xhtml)

[](File_6_in_1_MMC3_CNROM_menu_png.xhtml "Enlarge")

[![](../wiki-images/6-in-1_MMC3%2BCNROM_components.jpg)](File_6_in_1_MMC3_CNROM_components_jpg.xhtml)

[](File_6_in_1_MMC3_CNROM_components_jpg.xhtml "Enlarge")

Note that PRG mask-rom (marked as: EK-606A) was desoldered

[![6-in-1 MMC3+CNROM top.jpg](../wiki-images/6-in-1_MMC3%2BCNROM_top.jpg)](File_6_in_1_MMC3_CNROM_top_jpg.xhtml)

[](File_6_in_1_MMC3_CNROM_top_jpg.xhtml "Enlarge")

[![6-in-1 MMC3+CNROM bottom.jpg](../wiki-images/6-in-1_MMC3%2BCNROM_bottom.jpg)](File_6_in_1_MMC3_CNROM_bottom_jpg.xhtml)

[](File_6_in_1_MMC3_CNROM_bottom_jpg.xhtml "Enlarge")

[![6-in-1 MMC3+CNROM schematics.png](../wiki-images/6-in-1_MMC3%2BCNROM_schematics.png)](File_6_in_1_MMC3_CNROM_schematics_png.xhtml)

[](File_6_in_1_MMC3_CNROM_schematics_png.xhtml "Enlarge")

  
This multicart cartridge consists of AX5202 (pirate MMC3 chip) and other discrete chips. It allows running MMC3, CNROM or NROM games. CNROM games have to be patched so that they change 8kB CHR-ROM banks not by writing to $8000-$ffff, but by using MMC3 registers. If pattern tables of CNROM game is not modified, current CHR bank configuration have to be set using MMC3 registers to map into 8kB of linear CHR-ROM (to mimic CNROM) before starting CNROM game. 

This particular cartridge consists of the following games: 
    
    
    TITLE                                            | MAPPER |  PRG  |  CHR
    -------------------------------------------------+--------+-------+-------
    1. Teenage Mutant Ninja Turtles 3                | MMC3   | 256 K | 256 K 
    2. Darkwing Duck                                 | MMC3   | 128 K | 128 K 
    3. Transformers - Comvoy no Nazo (J)             | CNROM  |  32 K |  32 K 
    4. ASO - Armored Scrum Object (J)                | CNROM  |  32 K |  32 K 
    5. Tiger-Heli (U)                                | CNROM  |  32 K |  32 K 
    6. Karate Kid, The (U)                           | CNROM  |  32 K |  32 K 
    

Menu is encoded into game 1 and games 2-6 are also slightly modified. 

# Registers

All registers are same like in MMC3. There is no SRAM at $6000-$7fff but instead - one additional register mapped at $6000-$7fff. Writes to it can be protected in the same way as MMC3 protects SRAM (using $a001), because MMC3's pins controlling SRAM (!OE/!WE) are connected directly to this register control pins. 
    
    
    D~[.... BbmM] $6000-$7fff
            ||||
            |||+- MMC3 mode (0: on - PRG-ROM A17 & CHR-ROM A17 are controlled by MMC3, 1:off, PRG-ROM A17 = CHR-ROM A17 = b)
            ||+-- MMC3 mode (0: on, 1: off), see notes below 
            |+--- selects 128K PRG-ROM & 128K CHR-ROM block when M=1 (A17 of PRG-ROM & CHR-ROM)
            +---- selects 256K PRG-ROM & 256K CHR-ROM block (A18 of PRG-ROM & CHR-ROM)
    

### bit `m`

This bit controls what is connected to PRG-ROM A14 and MMC3's A14 

m = 0: PRG-ROM A14 is MMC's PRG A14 and MMC3's A14 is CPU-A14 This results in four 8kB CPU banks ($8000-$9FFF/$A000-$BFFF/$C000-$DFFF/$C000-$DFFF) with behaviour just as normal MMC3 

m = 1: PRG-ROM A14 is CPU-A14 and MMC3's A14 is GND When MMC3's A14 is GND, when CPU reads $8000-$bfff or $c000-$ffff, MMC3 thinks it reads $8000-$bfff in both cases, so it uses: 

  * bank PPPPP selected for $8000-$9fff for both $8000-$9fff and $c000-$dfff,
  * bank QQQQQ selected for $a000-$bfff for both $a000-$bfff and $e000-$ffff.



Additionally, PRG-ROM A14 is wired to CPU-A14, so mapping looks like: 
    
    
                A17 A16 A15 A14 A13
    $8000-$9fff   P   P   P   0   P
    $a000-$bfff   Q   Q   Q   0   Q
    $c000-$dfff   P   P   P   1   P
    $e000-$ffff   Q   Q   Q   1   Q
    

If: PPPPP is set to even number and QQQQQ = PPPPP + 1 and PRG ROM bank mode bit in MMC3 is set to 1 so that $9000-$9fff is swappable, then it results in one linear 32 kB bank at $8000-$ffff. 

Resistor R2 and diode D2 forces bit `m` to be set to 0 for writes at $8000-$bfff so that MMC3 can decode properly register writes, no matter if `m` is 0 or 1. 

Categories: [Mappers](Category_Mappers.xhtml)
