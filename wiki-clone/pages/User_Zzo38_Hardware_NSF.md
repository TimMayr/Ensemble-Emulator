# User:Zzo38/Hardware NSF

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AZzo38/Hardware_NSF) | View [other pages](Special_AllPages.xhtml#User_Zzo38_Hardware_NSF)

**!!! WARNING !!! THIS HAS VARIOUS MISTAKES IN IT !!!**

These are some of my ideas for hardware [NSF](NSF.xhtml "NSF"). 

## Contents

  * 1 Hardware
  * 2 PRG ROM Layout
  * 3 CHR ROM Layout
  * 4 Main Routine
  * 5 Interrupt Routine
  * 6 File Loading
  * 7 Miscellaneous Information



## Hardware

Memory from $4018-$403F would be filled by a 40-byte ROM used for an interrupt routine. (Famicom Disk System uses some of these addresses but NSF only uses the audio registers, not these ones.) 

When the main routine is enabled, $8000-$BFFF is a single byte of RAM (used to store the current song number; mirrored) and $C000-$FFFF is ROM (also mirrored; not much ROM is needed). 

Writing to even bytes of $4018-$403F: If bit2 is set, the memory from $8000-$DFFF becomes read/write (except main routine ROM which is always read-only), otherwise it is read-only. If bit0 is set, VRC6 audio will play, otherwise it will be muted. If bit1 is set, VRC7 audio will play and otherwise is muted. If bit5 is set, Sunsoft 5B audio is played and is otherwise muted. Other bits (bit3, bit4, bit6, and bit7) are ignored. 

Writing to odd bytes of $4018-$403F enables main routine ROM if the low bit of the data is set, and disables if the low bit is clear. Other bits are ignored. 

The hardware generate IRQ at the rate specified in the NSF header. When IRQ is called, the IRQ vector should be read as $4018 whenever it is checked. 

Another thing the hardware will do is convert the audio to digital and whenever the name tables are being accessed, make the pattern tables read as a waveform view of the music being played. 

## PRG ROM Layout

(note: somewhat obsolete with new specification) 
    
    
    +-----+-----------------------+
    | 1FF | Interrupt vectors     |
    | 1FA |                       |
    +-----+-----------------------+
    | 1F9 | Main routine          |
    | 100 |                       |
    +-----+-----------------------+
    | 0FF | .NSF header           |
    | 080 |                       |
    +-----+-----------------------+
    | 07F | Main routine helper   |
    | 040 |                       |
    +-----+-----------------------+
    | 03F | Interrupt routine     |
    | 018 |                       |
    +-----+-----------------------+
    | 017 | Stop routine          |
    | 000 |                       |
    +-----+-----------------------+
    

## CHR ROM Layout

CHR ROM is ASCII, but tile $00 is blank, and tiles $10-$1F contain the hex digits "0" to "F". 

Printable ASCII characters use color 3 for the foreground, and hex digits use color 1 for the foreground. All tiles use color 0 for the background. This way, the palette can be changed to make it change color of the song number digits to indicate play/stop. 

While rendering rows below the fourth row, the CHR ROM will not be accessible and instead they will be connected to a analog-to-digital converter which puts the same data regardless of address (but which is depending on the position on screen). 

## Main Routine

(note: somewhat obsolete with new specification) 

  * Disable IRQ.
  * Wait for PPU.
  * Write the starting song number (offset $07 of header) to $8000, and then decrement it.
  * Copy expansion chip byte (offset $7B of header) to $4018.
  * Set up palette.
  * Fill up the nametable with the title and so on from the NSF header, and with the current and maximum song number, and indicate the music is stopped.
  * Enable background.
  * Loop: 
    * Update the display of the current song number.
    * If LEFT is pushed: 
      * Decrement current song number. If it is zero it wraps around.
      * Wait for button is not pushed.
    * If RIGHT is pushed: 
      * Increment current song number. If it is too high it wraps to zero.
      * Wait for button is not pushed.
    * If START is pushed: 
      * Initialize the sound registers by writing $00 to $4000-$4013, $0F to $4015.
      * Initialize the frame counter to 4-step mode ($40 to $4017).
      * Write initial bank values to $5FF6-$5FFF. (If the initial bank values are all zero, instead compute them from the load address.)
      * Update display to indicate song is playing.
      * Load value at memory $8000 into the accumulator.
      * Load zero into the X register.
      * Jump to the start routine.
  * Stop routine: 
    * Disable interrupts.
    * Turn off all audio.
    * Reset the stack pointer.
    * Update display to indicate song is stopped.
    * Go to main loop checking which button is pushed.



## Interrupt Routine

  * Check if the A button is pushed.
  * If the A button is pushed (it should branch if button pushed rather than skip over if unpushed because this way uses less clock cycles): 
    * Enable main routine ROM.
    * Jump to the stop routine.
  * Reset the stack pointer.
  * Call the play subroutine.
  * Enable interrupts.
  * Run an idle loop doing nothing.
  * Start routine: 
    * Disable main routine ROM.
    * Call the init subroutine.
    * Enable interrupts.
    * Run an idle loop doing nothing.



Code (40 bytes, 30 clock): 
    
    
            .ORG $4018
    intr    LDX #1       ; 2/2 (X=1)
            STX $4016    ; 3/4 (strobe)
            DEX          ; 1/2 (X=0)
            STX $4016    ; 3/4 (allow buttons to be read)
            LDA $4016    ; 3/4 (read "A" button)
            BNE stopa    ; 2/2 (branch if button pushed)
            DEX          ; 1/2 (X=255)
            TXS          ; 1/2 (reset stack)
            JSR play     ; 3/6
            CLI          ; 1/2
    wait    JMP wait     ; 3/- (wait)
    stopa   STA $4019    ; 3/- (write 1 to $4019)
            JMP stop     ; 3/-
    start   STX $4019    ; 3/- (write 0 to $4019)
            JSR init     ; 3/-
            CLI          ; 1/-
            JMP wait     ; 3/-
    

## File Loading

Depending on how the data is loaded onto the cartridge, some things may need to differ a bit, for example it may need to include codes to read the file. This may mean having 128 bytes of RAM in $8000-$BFFF instead of only 1 byte, and use this for the .NSF header (header $07 could be reused for the current song number), and then have another code to load the banks by first setting bit2 of $4018, and then after it is loaded, copy header $7B to $4018 which may or may not clear bit2. 

The header bytes at $78 and $79 could be used for telling the cartridge what IRQ period to use. It could do something like this when loading: 

  * Disable interrupts.
  * Read the first 128 bytes of the file into the RAM reserved for the header ($8000-$807F).
  * Check the first six bytes of the header. If they are wrong: 
    * Change background to red.
    * Play error tone.
    * Halt.
  * Detect TV system. 
    * If NTSC: Copy header $6E to $78 and $6F to $79.
    * If bit1 of header $7A is not set, and bit0 of the header $7A is not equal to detected TV system: 
      * Change background to yellow.
      * Play error tone.
      * Halt.
    * Write the detected TV system to header $7A.
  * Read header $70 to $77. If all of them are zero: 
    * Load header $09 into a register.
    * Right shift four spaces.
    * Copy it to the X register.
    * Load zero into the A register.
    * While X is not equal to ten: 
      * STA $8068,X
      * Increment X register.
      * Add one to A register.
  * Bitwise AND header $09 with literal $0F.
  * Bitwise OR header $09 with literal $60.
  * Write $40 to $4018.
  * Write zero to Y register and $5FF6.
  * Until end of file: 
    * Read one byte from file.
    * Use header $08 and $09 as address to write the byte which has been read from the file.
    * Add one to header $08 and carry over to header $09.
    * If bit4 of header $09 is set: 
      * Increment Y register.
      * Write contents of Y register to $5FF6.
      * Clear bit4 at header $09.
  * Start loading the nametable and so on, and wait for buttons pushed.



Here is a description of a possible file-loading interface (you could use it in other hardware designs too; not only this one): 

  * A port with 12 pins on the cartridge: Data0 to Data7, Clock, EOF, Power, and Ground.
  * A device which connects to that port will use the simple file-loading protocol: When Clock is received, it reads a byte from the file and updates the Data0 to Data7 pins. If a byte cannot be read because it is the end of the file, EOF pin is set instead.
  * Cartridge has some way of connecting this port to the memory accessible by CPU.



This hardware NSF could have it implemented as follows: 

  * When powered on, the range $4040-$40FF accesses the file-loading port. A clock will be set and the data pins will be readable in this memory.
  * When EOF is set, IRQ will be triggered and then memory range $4040-$40FF will be used as FDS audio from then on.



## Miscellaneous Information

  * All audio expansion are always implemented, although the Sunsoft 5B, VRC6, and VRC7 can be muted (individually).
  * Memory range $6000-$7FFF is always RAM.
  * Memory $8000-$FFFF is either the NSF RAM or the "main routine" memory, which is 128 bytes of RAM and 512 bytes of ROM (only 1 byte of RAM if not using the "file loading" above).
  * RAM banks of 4K each can be written to when writing $6000-$DFFF is enabled, and can be switched using the NSF bankswitching registers.
  * Memory $E000-$FFFF is read-only, although it is a RAM bank, you can never write the RAM using these addresses; you must map it to different addresses to write it.
  * Memory $6000-$DFFF is banked RAM, and is writable when enabled, and otherwise read-only.
  * Some mirrored registers can be implemented, although done such that the addresses specified in the NSF specification always work without conflicts.


