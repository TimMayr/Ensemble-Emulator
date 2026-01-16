# INES Mapper 077

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_077) | View [other pages](Special_AllPages.xhtml#INES_Mapper_077)
    
    
     ========================
     =  Mapper 077          =
     ========================
     
     Example Game:
     --------------------------
     Napoleon Senki
     
     
     Notes:
     ---------------------------
     This mapper uses an 8 KiB SRAM to provide both 6 KiB of CHR-RAM and
      four-screen mirroring.
     
     Registers: (** BUS CONFLICTS **)
     ---------------------------
     
       $8000-FFFF:  [CCCC PPPP]
         C = CHR Reg (2k @ $0000)
         P = PRG Reg (32k @ $8000)
     
     
     CHR Setup:
     ---------------------------
     CHR-RAM is fixed at $0800-$1FFF.  CHR-ROM is swappable at $0000:
     
        $0000-$07FF $0800-$0FFF $1000-$17FF $1800-$1FFF $2000-$27FF  $2800-$2FFF
       +-----------+-----------+-----------+-----------+-----------+-------------+
       | $8000,ROM |  {1},RAM  |  {2},RAM  |  {3},RAM  |  {0},RAM  |Internal VRAM|
       +-----------+-----------+-----------+-----------+-----------+-------------+
     
     When making an emulator, you do not need to care about the specific order of
      the CHR-RAM banks: just provide 10KiB from $0800-$2FFF.
    

Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with CHR ROM and CHR RAM](Category_Mappers_with_CHR_ROM_and_CHR_RAM.xhtml)
