# Before the basics

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Before_the_basics) | View [other pages](Special_AllPages.xhtml#Before_the_basics)

## Contents

  * 1 Background
    * 1.1 More advanced topics
  * 2 Introducing the NES
  * 3 Important Considerations
  * 4 External links



## Background

A digital computer, such as a PC or NES, is filled with millions of tiny switches that can be turned on or off. Some switches store information for short time; these are called memory. Other switches calculate things based on the output of other switches; these are called logic. Each switch can be on or off, representing true or false, or the numbers 1 or 0. Multiple switches grouped together can represent larger values. A CPU reads machine code from a memory and treats it as instructions to perform arithmetic and logic operations on other values in memory or to perform input and output. Some of the input and output involve user interaction: the user gives commands to the computer, and the computer displays the result. 

If you're confused, now is the opportunity to become familiar with the following basic concepts from computer engineering and computer science. These articles from Wikipedia are in English, and many have been translated into other languages. 

  * [Computer](https://en.wikipedia.org/wiki/Computer "wikipedia:Computer") (it's not just a PC)
  * [Bit](https://en.wikipedia.org/wiki/Bit "wikipedia:Bit")
    * [Boolean logic](https://en.wikipedia.org/wiki/Boolean_algebra "wikipedia:Boolean algebra")
  * [Binary number](https://en.wikipedia.org/wiki/Binary_number "wikipedia:Binary number")
    * [Byte](https://en.wikipedia.org/wiki/Byte "wikipedia:Byte")
    * [Arithmetic logic unit](https://en.wikipedia.org/wiki/Arithmetic_logic_unit "wikipedia:Arithmetic logic unit")
    * [Binary prefix](https://en.wikipedia.org/wiki/Binary_prefix "wikipedia:Binary prefix") (KiB, MiB)
  * [Random-access memory](https://en.wikipedia.org/wiki/Random-access_memory "wikipedia:Random-access memory") (RAM)
  * [Read-only memory](https://en.wikipedia.org/wiki/Read-only_memory "wikipedia:Read-only memory") (ROM)
  * [Central processing unit](https://en.wikipedia.org/wiki/Central_processing_unit "wikipedia:Central processing unit") (CPU) 
    * [MOS Technology 6502](https://en.wikipedia.org/wiki/MOS_Technology_6502 "wikipedia:MOS Technology 6502")
    * [Clock rate](https://en.wikipedia.org/wiki/Clock_rate "wikipedia:Clock rate")
    * [Processor register](https://en.wikipedia.org/wiki/Processor_register "wikipedia:Processor register")
  * [Computer program](https://en.wikipedia.org/wiki/Computer_program "wikipedia:Computer program")
    * [Programming language](https://en.wikipedia.org/wiki/Programming_language "wikipedia:Programming language")
    * [Machine code](https://en.wikipedia.org/wiki/Machine_code "wikipedia:Machine code")
    * [Assembly language](https://en.wikipedia.org/wiki/Assembly_language "wikipedia:Assembly language") (asm)
    * [Source code](https://en.wikipedia.org/wiki/Source_code "wikipedia:Source code")
    * [Object file](https://en.wikipedia.org/wiki/Object_file "wikipedia:Object file")
    * [Parallel computing](https://en.wikipedia.org/wiki/Parallel_computing "wikipedia:Parallel computing")
    * [Thread](https://en.wikipedia.org/wiki/Thread_\(computing\) "wikipedia:Thread \(computing\)")
  * [Input/output](https://en.wikipedia.org/wiki/Input/output "wikipedia:Input/output") (I/O) 
    * [Memory-mapped I/O](https://en.wikipedia.org/wiki/Memory-mapped_I/O "wikipedia:Memory-mapped I/O")
    * [Serial port](https://en.wikipedia.org/wiki/Serial_port "wikipedia:Serial port")
    * [Interrupt](https://en.wikipedia.org/wiki/Interrupt "wikipedia:Interrupt")
    * [Serial Peripheral Interface Bus](https://en.wikipedia.org/wiki/Serial_Peripheral_Interface_Bus "wikipedia:Serial Peripheral Interface Bus") (SPI)
  * [Raster graphics](https://en.wikipedia.org/wiki/Raster_graphics "wikipedia:Raster graphics")
    * [Pixel](https://en.wikipedia.org/wiki/Pixel "wikipedia:Pixel")
    * [Raster scan](https://en.wikipedia.org/wiki/Raster_scan "wikipedia:Raster scan")
    * [Bitmap](https://en.wikipedia.org/wiki/Bitmap "wikipedia:Bitmap")
    * [Planar bitmap](https://en.wikipedia.org/wiki/Planar_\(computer_graphics\) "wikipedia:Planar \(computer graphics\)")
    * [Progressive scan](https://en.wikipedia.org/wiki/Progressive_scan "wikipedia:Progressive scan")
    * [Vertical blanking interval](https://en.wikipedia.org/wiki/Vertical_blanking_interval "wikipedia:Vertical blanking interval") (vblank)
    * [Text mode](https://en.wikipedia.org/wiki/Text_mode "wikipedia:Text mode")
    * [Text-based user interface](https://en.wikipedia.org/wiki/Text-based_user_interface "wikipedia:Text-based user interface")
    * [ASCII art](https://en.wikipedia.org/wiki/ASCII_art "wikipedia:ASCII art")
    * [Sprite](https://en.wikipedia.org/wiki/Sprite_\(computer_graphics\) "wikipedia:Sprite \(computer graphics\)")
    * [Display list](https://en.wikipedia.org/wiki/Display_list "wikipedia:Display list")
  * [Command-line interface](https://en.wikipedia.org/wiki/Command-line_interface "wikipedia:Command-line interface")
    * [Shell](https://en.wikipedia.org/wiki/Shell_\(computing\) "wikipedia:Shell \(computing\)")
    * [Windows Command Prompt](https://en.wikipedia.org/wiki/cmd.exe "wikipedia:cmd.exe")
    * [Batch file](https://en.wikipedia.org/wiki/Batch_file "wikipedia:Batch file")
    * [Plug-in](https://en.wikipedia.org/wiki/Plug-in_\(computing\) "wikipedia:Plug-in \(computing\)")
    * [Filename extension](https://en.wikipedia.org/wiki/Filename_extension "wikipedia:Filename extension")
    * [Zip file](https://en.wikipedia.org/wiki/Zip_\(file_format\) "wikipedia:Zip \(file format\)")
    * [Environment variable](https://en.wikipedia.org/wiki/Environment_variable "wikipedia:Environment variable")



### More advanced topics

It has been suggested that understanding these topics is not necessary to program low-complexity games in assembly language without sound, such as a workalike of Magnavox Odyssey tennis. They are held here for forthcoming sorting into a separate page. 

  * Games larger than one screen 
    * [Scrolling](https://en.wikipedia.org/wiki/Scrolling "wikipedia:Scrolling")
  * Audio 
    * [Digital signal](https://en.wikipedia.org/wiki/Digital_signal "wikipedia:Digital signal")
    * [Digital audio](https://en.wikipedia.org/wiki/Digital_audio "wikipedia:Digital audio")
    * [Signal generator](https://en.wikipedia.org/wiki/Signal_generator "wikipedia:Signal generator")
    * [Delta modulation](https://en.wikipedia.org/wiki/Delta_modulation "wikipedia:Delta modulation")
    * [Digital frequency divider](https://en.wikipedia.org/wiki/Frequency_divider#Digital_dividers "wikipedia:Frequency divider") (also [on Wikia](http://community.fandom.com/wiki/c:electronicmusic:Pulse_divider "wikia:c:electronicmusic:Pulse divider"))
  * Mapper 
    * [Logic gate](https://en.wikipedia.org/wiki/Logic_gate "wikipedia:Logic gate")
    * [Linear feedback shift register](https://en.wikipedia.org/wiki/Linear_feedback_shift_register "wikipedia:Linear feedback shift register")
    * [Ring counter](https://en.wikipedia.org/wiki/Ring_counter "wikipedia:Ring counter")
    * [Bus](https://en.wikipedia.org/wiki/Bus_\(computing\) "wikipedia:Bus \(computing\)")
    * [Bank switching](https://en.wikipedia.org/wiki/Bank_switching "wikipedia:Bank switching")
    * [Integrated circuit](https://en.wikipedia.org/wiki/Integrated_circuit "wikipedia:Integrated circuit") (IC)
    * [7400 series](https://en.wikipedia.org/wiki/7400_series "wikipedia:7400 series")
  * High-level languages 
    * [Compiler](https://en.wikipedia.org/wiki/Compiler "wikipedia:Compiler")



## Introducing the NES

The Nintendo Entertainment System has the following components: 

  * 2A03 CPU IC made by Ricoh 
    * CPU based on the MOS Technology 6502 8-bit microprocessor
    * serial input for game controllers
    * audio output comprising four tone generators and a delta modulation playback device
  * 2 KiB of RAM for use by the CPU
  * 2C02 PPU (picture processing unit) 
    * tile-based background image
    * 64 sprites (individually moving objects)
    * 25 colors out of 53
    * 256x240 pixel progressive picture generator
    * NTSC color encoder
  * 2 KiB of RAM for use by the PPU



Cartridges have the following components: 

  * 16 KiB or more PRG ROM, for use by the CPU
  * 8 KiB or more CHR ROM or CHR RAM, for use by the PPU (and in some cases by the CPU)
  * (optional) Bank switching hardware for the ROMs
  * (optional) Logic to generate interrupts



## Important Considerations

The NES is not an easy system to develop games for anybody who does not have a good information and knowledge about the low-level workings of a computer. Developing games for NES can be tricky for developers who are used to tools like Unreal Engine, Unity, Godot, and game libraries and frameworks like Raylib, OpenGL, etc. 

A few of such (at first) hard-to-grasp concepts and difficulties that one might face during their journey are: 

  * Memory-mapped I/O, memory mirroring, incomplete address decoding and mapping and mappers can be confusing to understand and deal with, at first.
  * Issues like race conditions can pop up during the journey, such as not being able to write to video memory while the PPU is busy rendering the screen (you have to wait until the Vertical Blanking interval begins, of which the PPU can notify using an interrupt).
  * One cannot have "pre-initialized" global variables unless coded manually (due to program residing entirely in the ROM).



If you do not have a good programming knowledge, then you should firstly consider getting programming experience. Knowledge of the 6502 assembly language is important and knowledge and experience in C programming language is extremely helpful, as it is often used and compiled and linked together with the 6502 assembly. 

Not understanding concepts clearly such as memory-management and not having enough technical knowledge about the hardware can give one a hard time developing tools or games on the NES, such as if you don't understand the memory-range of the hardware and you allocate memory outside the range of what hardware can access, you might get bugs and problems popping up in your code. Hence, having a good grasp of assembly is essential as it allows you to gain experience to the memory mappings of the hardware and management of the same. 

And most importantly, **one should not expect much from the NES hardware** , it has a 1.7 MHz processor compared to a modern processor like Intel i5-13400 processor which runs at 2.5 GHz. The PPU on the NES spends exactly _1 clock cycle per pixel_. Another important thing is that **PPU and CPU do not share any memory** , both have their own buses. 

## External links

  * [Easy 6502](https://skilldrick.github.io/easy6502/)


