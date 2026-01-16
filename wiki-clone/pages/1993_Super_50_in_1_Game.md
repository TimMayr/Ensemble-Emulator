# 1993 Super 50 in 1 Game

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/1993_Super_50_in_1_Game) | View [other pages](Special_AllPages.xhtml#1993_Super_50_in_1_Game)

## Contents

  * 1 Description
  * 2 Hardware
  * 3 Software
  * 4 Dumping method
  * 5 Games Menu



## Description

The cartridge is a multigames cartridge dated from 1993. 

Versions known : 

  * 1993 Super 50 in 1 Game : NES version (72-pin)



## Hardware

NES PCB (Front mirrored picture) | NES PCB (Bottom picture)   
---|---  
[![50 in 1 NES PCB \(Front mirrored picture\)](../wiki-images/50in1-MG109-frontside-inverse.jpg)](File_50in1_MG109_frontside_inverse_jpg.xhtml "50 in 1 NES PCB \(Front mirrored picture\)") | [![50 in 1 NES PCB \(Bottom picture\)](../wiki-images/50in1-MG109-bottomside.jpg)](File_50in1_MG109_bottomside_jpg.xhtml "50 in 1 NES PCB \(Bottom picture\)")  
  
  


Datas chips   
---  
Chips | Description   
Blob | PRG-ROM   
Blob | CHR-ROM   
Usual fonctionnal chips   
Chips | Description   
74LS161A - 6.70 | BCD Decade counters / 4-bit binary counters   
GD74LS00 - 9223 - GoldStar | NAND Gate, LS Series, 4-Func, 2-Input, TTL, PDIP14   
  
  


[![50 in 1 Schematic](../wiki-images/CART496_50_in_1_schematic_0.png)](File_CART496_50_in_1_schematic_0_png.xhtml "50 in 1 Schematic")

## Software

The 50 in 1 rom contains 64 KB of PRG and 32 KB of CHR. 

The rom has been dumped and assigned to the [INES Mapper 200](INES_Mapper_200.xhtml "INES Mapper 200"). 

It contains 4 different games that can be beginned at different levels. 

## Dumping method

This script can dump the cartridge with the INLretro-prog dumper (Device firmware version: 2.3.x) : 

dumpMG109.lua   
---  
[File:DumpMG109.lua.txt](File_DumpMG109_lua_txt.xhtml "File:DumpMG109.lua.txt")  
      
    
    -- create the module's table
    local dumpmg109 = {}
    
    -- import required modules
    local dict = require "scripts.app.dict"
    local nes = require "scripts.app.nes"
    local dump = require "scripts.app.dump"
    local flash = require "scripts.app.flash"
    local time = require "scripts.app.time"
    local files = require "scripts.app.files"
    local swim = require "scripts.app.swim"
    local buffers = require "scripts.app.buffers"
    
    -- local functions
    local function create_header( file, prgKB, chrKB )
    	-- Mapper : 200 ; Mirroring : 0 (Horizontal)
    
    	nes.write_header( file, prgKB, chrKB, 200, 0)
    end
    
    --dump the PRG ROM
    local function dump_prgrom( file, rom_size_KB, debug )
    
    	--PRG-ROM dump 16KB at a time
    	local KB_per_read = 16
    	local num_reads = rom_size_KB / KB_per_read
    	local read_count = 0
    	local addr_base = 0x80	-- $8000
    
    print("CPU dump :")
    
    	while ( read_count < num_reads )
    	do
    		if debug then print( "dump PRG part ", read_count, " of ", num_reads) end
    
    		-- pause of 1 second to take care about the chips
    		local time=os.clock()+1
    		while time>os.clock()
    			do
    			end
    
    		local nobusconflicts = dict.nes("NES_CPU_RD", 0x8000 | read_count);
    		dict.nes("NES_CPU_WR", 0x8000 | read_count , nobusconflicts)
    
    		dump.dumptofile( file, KB_per_read, addr_base, "NESCPU_PAGE", false )
    
    		read_count = read_count + 1
    	end
    end
    
    --dump the CHR ROM
    local function dump_chrrom( file, rom_size_KB, debug )
    
    	local KB_per_read = 8	--dump both PT at once
    	local num_reads = rom_size_KB / KB_per_read
    	local read_count = 0
    	local addr_base = 0x00	-- $0000
    
    
    print("PPU dump :")
    
    	while ( read_count < num_reads ) do
    
    		if debug then print( "dump CHR part ", read_count, " of ", num_reads) end
    
    		-- pause of 1 second to take care about the chips
    		local time=os.clock()+1
    		while time>os.clock()
    			do
    			end
    
    		local nobusconflicts = dict.nes("NES_CPU_RD", 0x8000 | read_count);
    		dict.nes("NES_CPU_WR", 0x8000 | read_count , nobusconflicts)
    
    		dump.dumptofile( file, KB_per_read, addr_base, "NESPPU_PAGE", false )
    
    		read_count = read_count + 1
    	end
    end
    
    --Cart should be in reset state upon calling this function
    --this function processes all user requests for this specific board/mapper
    --local function process( test, read, erase, program, verify, dumpfile, flashfile, verifyfile)
    local function process(process_opts, console_opts)
    	local test = process_opts["test"]
    	local read = process_opts["read"]
    	local erase = process_opts["erase"]
    	local program = process_opts["program"]
    	local verify = process_opts["verify"]
    	local dumpfile = process_opts["dump_filename"]
    	local flashfile = process_opts["flash_filename"]
    	local verifyfile = process_opts["verify_filename"]
    
    	local rv = nil
    	local file
    	local prg_size = console_opts["prg_rom_size_kb"]
    	local chr_size = console_opts["chr_rom_size_kb"]
    	local wram_size = console_opts["wram_size_kb"]
    
    --initialize device i/o for NES
    	dict.io("IO_RESET")
    	dict.io("NES_INIT")
    
    print("\nRunning dumpMG109.lua")
    
    --dump the cart to dumpfile
    	if read then
    		print("\nDumping ROM...")
    
    		file = assert(io.open(dumpfile, "wb"))
    
    		--create header: pass open & empty file & rom sizes
    		create_header(file, prg_size, chr_size)
    
    		--TODO find bank table to avoid bus conflicts!
    		--dump cart into file
    		time.start()
    		--dump cart into file
    		dump_prgrom(file, prg_size, false)
    		dump_chrrom(file, chr_size, false)
    		time.report(prg_size)
    
    		--close file
    		assert(file:close())
    		print("DONE Dumping ROM")
    	end
    
    	dict.io("IO_RESET")
    end
    
    -- functions other modules are able to call
    dumpmg109.process = process
    
    -- return the module's table
    return dumpmg109  
  
Under UNIX, the commands to run are : 
    
    
    ./inlretro -s scripts/inlretro3.lua -c NES -m dumpmg109 -x 64 -y 32 -d 50in1-1993.nes
    

  
The NES header of the rom is to set to 
    
    
    

## Games Menu

There are 4 unique games and the same games at different levels : 

  * Duck Hunt
  * Wild Gunman
  * Battle City
  * Mario Bros (Arcade / 1983)

Cartridge Menu   
---  
N° | Game Name | Original Game Name | Level   
1 | DUCK HUNT | Duck Hunt | Game A : 1 duck   
2 | TWO DUCKS HUNT | Duck Hunt | Game B : 2 ducks   
3 | CLAY SHOOTING | Duck Hunt | Game C : Clay Shooting   
4 | WILD GUNMAN | Wild Gunman | Game A : 1 outlaw   
5 | TWO GUNMEN | Wild Gunman | Game B : 2 outlaws   
6 | SALOON GUNMEN | Wild Gunman | Game C : Gang   
7 | BATTLE CITY 1 | Battle City | Stage 1   
8 | BATTLE CITY 2 | Battle City | Stage 2   
9 | BATTLE CITY 3 | Battle City | Stage 3   
10 | BATTLE CITY 4 | Battle City | Stage 4   
11 | BATTLE CITY 5 | Battle City | Stage 5   
12 | BATTLE CITY 6 | Battle City | Stage 6   
13 | BATTLE CITY 7 | Battle City | Stage 7   
14 | BATTLE CITY 8 | Battle City | Stage 8   
15 | BATTLE CITY 9 | Battle City | Stage 9   
16 | BATTLE CITY 10 | Battle City | Stage 10   
17 | BATTLE CITY 11 | Battle City | Stage 11   
18 | BATTLE CITY 12 | Battle City | Stage 12   
19 | BATTLE CITY 13 | Battle City | Stage 13   
20 | BATTLE CITY 14 | Battle City | Stage 14   
21 | BATTLE CITY 15 | Battle City | Stage 15   
22 | BATTLE CITY 16 | Battle City | Stage 16   
23 | BATTLE CITY 17 | Battle City | Stage 17   
24 | BATTLE CITY 18 | Battle City | Stage 18   
25 | BATTLE CITY 19 | Battle City | Stage 19   
26 | BATTLE CITY 20 | Battle City | Stage 20   
27 | BATTLE CITY 21 | Battle City | Stage 21   
28 | BATTLE CITY 22 | Battle City | Stage 22   
29 | BATTLE CITY 23 | Battle City | Stage 23   
30 | BATTLE CITY 24 | Battle City | Stage 24   
31 | BATTLE CITY 25 | Battle City | Stage 25   
32 | BATTLE CITY 26 | Battle City | Stage 26   
33 | BATTLE CITY 27 | Battle City | Stage 27   
34 | BATTLE CITY 28 | Battle City | Stage 28   
35 | BATTLE CITY 29 | Battle City | Stage 29   
36 | BATTLE CITY 30 | Battle City | Stage 30   
37 | BATTLE CITY 31 | Battle City | Stage 31   
38 | BATTLE CITY 32 | Battle City | Stage 32   
39 | BATTLE CITY 33 | Battle City | Stage 33   
40 | BATTLE CITY 34 | Battle City | Stage 34   
41 | BATTLE CITY 35 | Battle City | Stage 35   
42 | MARIO BROS 1 | Mario Bros (Arcade / 1983) | Phase 1   
43 | MARIO BROS 2 | Mario Bros (Arcade / 1983) | Phase 2   
44 | MARIO BROS 3 | Mario Bros (Arcade / 1983) | Phase 3   
45 | MARIO BROS 4 | Mario Bros (Arcade / 1983) | Phase 4   
46 | MARIO BROS 5 | Mario Bros (Arcade / 1983) | Phase 5   
47 | MARIO BROS 6 | Mario Bros (Arcade / 1983) | Phase 6   
48 | MARIO BROS 7 | Mario Bros (Arcade / 1983) | Phase 7   
49 | MARIO BROS 8 | Mario Bros (Arcade / 1983) | Phase 8   
50 | MARIO BROS 9 | Mario Bros (Arcade / 1983) | Phase 9 
