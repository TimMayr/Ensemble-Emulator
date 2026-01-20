# PowerPak Menu

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/PowerPak_Menu) | View [other pages](Special_AllPages.xhtml#PowerPak_Menu)

The [PowerPak](PowerPak.xhtml "PowerPak") boot ROM and menu system use its own simple mapper. The PowerPak menu is split into a series of modules that reside at $0400-$07FF in the NES's built-in RAM and are loaded from the CF card. After booting, the module "I.MAP" is loaded and run. 

See also: [PowerPak development resources](PowerPak.xhtml#PowerPak_development_resources "PowerPak")

## Contents

  * 1 Modules
  * 2 Banks
  * 3 Registers
    * 3.1 FPGADATA ($5000)
    * 3.2 FPGAPROGRAM ($5800)
    * 3.3 MAPPERWR ($8000)
    * 3.4 PRGBANK ($4200)
    * 3.5 CHRBANK ($4201)
  * 4 External Links



## Modules

Modules are switched between by writing the filename to fileEntry (usually a single character, calling ClearFindEntry beforehand if it's not already cleared), then calling CardLoadModule. Source for all modules is located [here](http://nespowerpak.com/powerpaksrc/). 

  * **A.MAP** (_A_ uto) - checks a "saves" directory for a save with the same name as the ROM being loaded
  * **B.MAP** (_B_ attery) - save selection screen
  * **D.MAP** (_D_ isk) - [FDS](Family_Computer_Disk_System.xhtml "FDS") save selection screen
  * **E.MAP** (_E_ rror) - displays "Bad File Header"
  * **F.MAP** (_F_ DS) - attempts to load an "FDSBIOS.BIN" from the PowerPak directory into WRAM, then starts an FDS game
  * **G.MAP** (_G_ ame Genie) - Game Genie .txt file selection screen
  * **I.MAP** (_I_ nit) - title screen
  * **L.MAP** (_L_ ast) - saves the game name to LASTNES.TXT in the PowerPak directory, also loads appropriate iNES mapper file based on header
  * **M.MAP** (_M_ usic) - loads an NSF and the NSF player, and starts the NSF
  * **N.MAP** (_N_ avigate) - ROM selection screen
  * **O.MAP** (_O_ ptions) - game options screen, with Game Genie code entering and save loading
  * **Q.MAP** (_Q_ uestion) - "save to CF card?" screen
  * **S.MAP** (_S_ ave) - writes a save to the CF card



## Banks

  * PPU $0000-$1FFF: 8 KB switchable CHR RAM bank.
  * CPU $6000-$7FFF: 8 KB switchable PRG RAM bank, selects from the 512 KB "PRG ROM" space.
  * CPU $8000-$BFFF: 16 KB switchable PRG ROM bank, selects from boot ROM.
  * CPU $C000-$FFFF: 16 KB PRG ROM bank, fixed to the last bank of boot ROM.



## Registers

### FPGADATA ($5000)

The boot ROM writes $FF to this register. It tells the FPGA it is being reprogrammed. 

### FPGAPROGRAM ($5800)
    
    
    7  bit  0
    ---- ----
    DDDD DDDD
    |||| ||||
    ++++-++++- Writes a byte of configuration to the FPGA.
    

### MAPPERWR ($8000)
    
    
    7  bit  0
    ---- ----
    .... ..DD
           ||
           ++- Selects the bank of boot ROM that appears at $8000-$BFFF.
    

Banks 0, 1 and 2 contain the initial FPGA configuration. Bank 3 contains 8KB of font and then 8KB of system routines. As the last 16KB are also accessible via the fixed bank, the $8000-$BFFF area isn't useful for menu software. 

### PRGBANK ($4200)
    
    
    7  bit  0
    ---- ----
    WCDD DDDD
    |||| ||||
    ||++-++++- Selects the bank of PRG RAM that appears in $6000-$7FFF.
    |+-------- Enable CHR RAM writing if 1.
    +--------- Selects from 32KB of WRAM instead of main PRG RAM.
    

### CHRBANK ($4201)
    
    
    7  bit  0
    ---- ----
    ..DD DDDD
      || ||||
      ++-++++- Selects the bank of CHR RAM used.
    

## External Links

  * Collection of information and photos, source code for menu and loader: <http://nespowerpak.com/>



Categories: [Mappers using $4020-$5FFF](Category_Mappers_using__4020__5FFF.xhtml)
