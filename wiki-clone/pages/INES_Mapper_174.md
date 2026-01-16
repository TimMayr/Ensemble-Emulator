# INES Mapper 174

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_174) | View [other pages](Special_AllPages.xhtml#INES_Mapper_174)
    
    
     Here are Bisqwit's original notes:  
     ========================
     =  Mapper 174          =
     ========================
     
     Example Game:
     --------------------------
     NTDec 5-in-1 (NTSC), Famicom cartridge format
     
     PRG ROM size: 128 kB
     CHR ROM size: 64 kB 
     
     Registers:
     ---------------------------
     
      $8000-FFFF:     A~[.... .... OPPP CCCM]
         M = Mirroring (1=Horizontal, 0=Vertical)
         P = when O=0 selects 16k both @ $8000 and @ $C000
             when O=1, the top two bits select 32k @ $8000 and the bottom bit is ignored
         O = PRG mode: selects 16 kB (when 0) or 32 kB mode (when 1).
         C = Selects 8 kB page for CHR memory $0000
     
     All of the mapper register access seems to be done in $FF00-$FFFF range, however the entire range works.
     
     The cartridge powers on in a state where all registers are 0.
     
     The cartridge expect the RAM contents to be kept when reset is pressed,
     whereupon it boots the game. The only way to return to the main menu after
     selecting a game is to power cycle the console (cold reboot).
     
     This mapper is functionally identical to [mapper 58](INES_Mapper_058.xhtml "INES Mapper 058") except the bits are moved around.
     
     Notes:
     ---------------------------
     This mapper is probably unique to that board, constructed from
     two [GD74LS161As](74161.xhtml "74161") (4-bit binary counter) and one [GD74LS153](74153.xhtml "74153") (dual 4-input multiplexer).
     
     It is a multicart that consists of games of different sizes.
     - Three games with PRG size = 32 kB (The Legend of Kage, The Goonies, Arkanoid)
     - One game with PRG size = 16 kB (Urban Champion)
     - One game with PRG size = 8 kB (Galaxian), which shares a 16 kB page together with the main menu.
     
     The main menu does some writes to $4028 once in a while, and one of
     the games (Urban Champion) writes to $4025. The purpose of these writes
     is unknown, as the cartridge does not trap those writes, and there is
     (probably) no hardware in the NTDec famiclone that responds at those addresses.
     It can be noted though, that $4025 is the [FDS](Family_Computer_Disk_System.xhtml "FDS") motor control register,
     and the value written would safely stop a disk system motor.
     It is possibly a leftover from porting the game or its library routines
     from the FDS to the Famicom.
    

More information: [[1]](http://forums.nesdev.org/viewtopic.php?p=96276#p96276)

Similar mappers: [58](INES_Mapper_058.xhtml "INES Mapper 058"), [60](INES_Mapper_060.xhtml "INES Mapper 060"), [212](INES_Mapper_212.xhtml "INES Mapper 212"), [231](INES_Mapper_231.xhtml "INES Mapper 231")

Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
