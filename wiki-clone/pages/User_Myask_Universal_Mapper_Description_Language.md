# User:Myask/Universal Mapper Description Language

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AMyask/Universal_Mapper_Description_Language) | View [other pages](Special_AllPages.xhtml#User_Myask_Universal_Mapper_Description_Language)

[Discussion thread.](http://forums.nesdev.org/viewtopic.php?f=2&t=14542)

## Contents

  * 1 Needed features
  * 2 Specification
    * 2.1 Comments
    * 2.2 Declarations:
    * 2.3 Statements:
    * 2.4 Operators
    * 2.5 Execution blocks
    * 2.6 Handy Shorthand Defines
  * 3 NROM-256 example



# Needed features

Been thinking about making some better way to talk about mappers, as described [in this post](http://forums.nesdev.org/viewtopic.php?p=74174#p74174) among several other places on our boards. Parts/functions 

  * Define state bits 
    * include easy ROM/RAM chip(/internal) declaration; don't want to exclude MagicFloor nor MMC5/6 from "correct" description
  * Logic
  * Arbitrary address bus size for chips?



Convenience addtions 

  * Conditional operations (optional but helps user-side…but makes it harder program-side) 
    * some header fields as parameters might be desirable (mirroring, chip sizes)
    * on the other hand, they are different boards in some manner. Perhaps only as solder-pad options?
  * [Cartridge connector](Cartridge_connector.xhtml "Cartridge connector") pins as predefined signal names, or a module (to allow picking 60 or 72-pin)
  * Option to autoconnect power, ground, address lines that are not in file 
    * like connecting PPU_A[0:7] to CHR_ROM_A[0:7] if CHR_ROM_A[0:7] do not appear in the description)
    * Also autoconnect CIC



Extra function thoughts 

  * Outputs (e.g. LED)
  * Inputs (e.g. DIPswitch, solder pads)
  * Describing expansion port devices in similar manner
  * Describing controller port devices in similar manner



Hard Part 

  * Expansion audio (analog, can involve extra oscillators as [VRC7 audio](VRC7_audio.xhtml "VRC7 audio") does)



# Specification

### Comments
    
    
    //c-style
    /*and c-style*/
    //Let's also treat all whitespace the same (except newlines terminating //, blocks)
    //and "to" should be ignored in whitespace
    

### Declarations:
    
    
    mapper name begin
    //contents
    end name;
    

The outer part. Technically a block started with begin/end. Name optional. 
    
    
    mem name (address lines, data lines, writable, class);
    

  * writable can be RAM or ROM.
  * Class can be PRG, CHR, VRAM, [audio?] and MISC. Class controls default connections, and helps make obvious to readers what a thing is.



(see NROM example for default connections. Any prg gets CPU_A, CPU_D, /ROMSEL ("ROMSEL_n"), and optionally the R/W signal if it's a RAM. chr/vram gets PPU_A, PPU_D, read/writes, and enable depending on PPU_A[13] (or /A13 if VRAM) 
    
    
    solder name to /*contents*/; //can block if desired
    jumper name to /*contents*/; //can block if desired
    dip name to /*contents*/; //can block if desired: same as solder for emulator
    

  * dipswitch



Just different name, really both just a state-based if statement, and these state bits are not settable except hardware(emulator)-side, unlike… 
    
    
    register name; 
    reg name;//short form
    
    
    
    init name to value; //can refer to iNES header fields like mirroring
    

Not that iNES is something we want much of, but it'll cut down on file redundancy. 

### Statements:
    
    
    connect x to y to z; //any size netlist, whitespace-separated
    = x y z; //shortform
    set name to value; //set a state bit
    <= x y; //shortform
    

### Operators
    
    
    bitwise & AND | OR ^ XOR ~ invert
    logical &&AND ||OR ^^XOR !not
    mathematical + - * binary operations, -negation
    {concatenate, concatenatee} [bus-index:range] {3 duplicate} //as in verilog
    

### Execution blocks
    
    
    on CPU_WRITE /*do stuff*/;
    on PPU_WRITE
    on CPU_READ
    on PPU_READ
    on CLOCK
    

### Handy Shorthand Defines
    
    
    bankreg name (which bus, width, address lines replaced, address mask selecting=equals what, which bus to write [inc A or D to write], mask to write=equals what,, written bits);
    fixedbank (which bus, width, address lines replaced=with what, address mask selecting=equals what);
    

e.g. 
    
    
    bankreg bnrom (CPU, 2, 16:15, 16'h8000=16'h8000, CPU_D, 16'h8000=16'h8000, CPU_D[1:0]);
    
    
    
    bankreg bxrom (CPU, 8, 22:15, 16'h8000=16'h8000, CPU_D, 16'h8000=16'h8000, CPU_D[7:0]);
    
    
    
    bankreg gnrom_cpu (CPU, 2, 16:15, 16'h8000=16'h8000, CPU_D, 16'h8000=16'h8000, CPU_D[5:4]);
    bankreg gnrom_ppu (PPU, 2, 14:13, 16'h2000=16'h0000, CPU_D, 16'h8000=16'h8000, CPU_D[1:0]);
    
    
    
    bankreg magic_series_cpu (CPU, 7, 21:15, 16'h8000=16'h8000, CPU_D, 16'h8000=16'h8000, CPU_D[7:1]);
    bankreg magic_series_ppu (PPU, 8, 20:13, 16'h2000=16'h0000, CPU_D, 16'h8000=16'h8000, CPU_D[7:0]);
    
    bankreg unrom (CPU, 3, 16:14, 16'hC000=16'h8000, CPU_D, 16'h8000=16'h8000, CPU_D[2:0]);
    fixedbank unrom_hi (CPU, 3, 16:14=3'b111, 16hC000=16'hC000);
    

The idea here is that it autogenerates a register of appropriate width, and statements in ON_x_WRITE to write that register when its write mask equality is met, and sets the appropriate address lines when its select mask is met. fixedbank does not need a register, obviously. So, for bnrom, it would expand to 
    
    
    reg bnrom_bank[1:0];
    on CPU_WRITE begin
    	if (CPU_A & 16'h8000 == 16'h8000) bnrom_bank <= CPU_D[1:0];
    	if (CPU_A & 16'h8000 == 16'h8000) prg.a[16:15] = bnrom_bank;
    end CPU_WRITE;
    on CPU_READ if (CPU_A & 16'h8000=16'h8000) prg.a[16:15] = bnrom_bank;
    

# [NROM](NROM.xhtml "NROM")-256 example
    
    
    mapper NROM_256V begin
     //without autofills
     //aside from the mirroring, strikes me as the same as "default connections"?
    //component section
     prgrom prg(32KiB); 
     //could also write 256Kib..but seems like a source of many typo problems
     //perhaps go by address line, data line counts?
     chrrom chr(8KiB);
     //only difference between PRG and CHR def'ns are its default connections
     //and outputs
     CIC cic(NES);//allow other chips I guess?
    
    //dynamic components section
     solder h to connect CIRAM_A10 to PPU_A[10];
     solder v to connect CIRAM_A10 to PPU_A[11]; 
     init h iNES.6[0];
     init v ~iNES.6[0];
      //technically redundant per wiki as only V-using boards had solder pads?
      //make "to" as whitespace, allowing nice codelook but not requiring 
     connect CIRAM_CE_n to PPU_A13_n;
    
    //connections: power
     connect VCC prg.vcc cic.vcc chr.vcc;
     connect GND prg.gnd cic.gnd chr.gnd;
      //allow multiple connections per statement
      //considering a shortform lke "=" for connect
    //connections: CIC 
     //[omitted]
    //connections: PRG
     connect prg.a[14:0] CPU_A[14:0];
     //NROM_128: connect prg.a[13:0] CPU_A[13:0];
     //and connect prg.a[14] VCC; //several ways to do it, really.
     connect prg.d[7:0] CPU_D[7:0];
     connect prg.oe_n prg.ce_n ROMSEL_n; //I suspect I've got these mildly wrong
    //connections: CHR
     connect chr.a[12:0] PPU_A[12:0];
     connect chr.d[7:0] PPU_D[7:0];
     connect chr.oe_n chr.ce_n PPU_A[13];
    
    end NROM_256V;
    
