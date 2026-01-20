# INES Mapper 156

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_156) | View [other pages](Special_AllPages.xhtml#INES_Mapper_156)

As of 04-sep-2012, fceux supports this mapper correctly (r2648). The code was taken from fceu-mm which had some bugs related to missing reset logic. It is additionally supported by the Nestopia 1.40u4 hack. Mednafen also supports this mapper. 

It is described as "DIS23C01 [DAOU](http://community.fandom.com/wiki/c:bootleggames:Daou_Infosys "wikia:c:bootleggames:Daou Infosys") ROM CONTROLLER, Korea" and is used by the following games: 

  * Metal Force (K)
  * Buzz and Waldog (K)
  * General's Son (K)
  * Koko Adventure (K)



  
The following description of it is based on FCEUX's implementation. It appears to fully-decode them, but that's probably inaccurate. 

  * PRG-ROM (16KiB switchable + 16KiB fixed-to-last banks)
  * PRG-RAM (8KiB, the usual @ $6000-$7FFF, sometimes battery-backed?)
  * CHR-ROM (8x1KiB banks)
  * Mirroring (defaults to One Screen A; only written on General's Son?; once written, changes to V/H, or difference in hardware?)

Registers  Address | Purpose | Size | Target's address range   
---|---|---|---  
$C000 | CHR bank(low) | 1KiB | $0000-$03FF   
$C001 | CHR bank(low) | 1KiB | $0400-$07FF   
$C002 | CHR bank(low) | 1KiB | $0800-$0BFF   
$C003 | CHR bank(low) | 1KiB | $0C00-$0FFF   
$C004 | CHR bank(hi) | 1KiB | $0000-$03FF   
$C005 | CHR bank(hi) | 1KiB | $0400-$07FF   
$C006 | CHR bank(hi) | 1KiB | $0800-$0BFF   
$C007 | CHR bank(hi) | 1KiB | $0C00-$0FFF   
$C008 | CHR bank(low) | 1KiB | $1000-$13FF   
$C009 | CHR bank(low) | 1KiB | $1400-$17FF   
$C00A | CHR bank(low) | 1KiB | $1800-$1BFF   
$C00B | CHR bank(low) | 1KiB | $1C00-$1FFF   
$C00C | CHR bank(hi) | 1KiB | $1000-$13FF   
$C00D | CHR bank(hi) | 1KiB | $1400-$17FF   
$C00E | CHR bank(hi) | 1KiB | $1800-$1BFF   
$C00F | CHR bank(hi) | 1KiB | $1C00-$1FFF   
$C010 | PRG bank | 16KiB | $8000-$BFFF   
$C014 | Mirroring | d0, 1 for H   
  
Categories: [INES Mappers](Category_INES_Mappers.xhtml)
