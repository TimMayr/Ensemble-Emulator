# Catch-up

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Catch-up) | View [other pages](Special_AllPages.xhtml#Catch_up)

The NES [CPU](CPU.xhtml "CPU"), [PPU](PPU.xhtml "PPU"), [APU](APU.xhtml "APU"), and [mapper](Mapper.xhtml "MMC") run in parallel (that is, at the same time), and NES games are designed with this in mind. Many of them modify the PPU registers and CHR bank numbers multiple times per frame to produce scroll splits, curved roads, and other raster effects, though not nearly to the same extent as the Atari 2600 games featured in _Racing the Beam_ by Nick Montfort and Ian Bogost (ISBN 9780262012577). But most emulators are programmed for a [Von Neumann architecture](https://en.wikipedia.org/wiki/Von_Neumann_architecture "wikipedia:Von Neumann architecture") that does only one thing at a time. So in some sense, an emulator must switch among emulating the CPU, PPU, APU, and mapper one at a time. This switching must be fairly fine-grained: if an emulator runs the CPU for a whole frame and then runs the PPU for a whole frame, for example, the raster effects won't be visible. 

The design philosophy of the emulator [Nintendulator](Nintendulator.xhtml "Nintendulator") takes clarity and accuracy over speed; it emulates each component for one CPU cycle before switching to the next. Most other emulators optimized for run-time efficiency do some level of **catch-up** , involving running the emulated CPU for several dozen cycles and then running the PPU and APU until they are synchronized. Keeping one component in the host CPU for a longer time speeds things up because the relevant data stays in the host CPU's fast registers and cache, not (slower) main memory, as long as the end result is [as if](https://docs.oracle.com/cd/E19205-01/819-5265/bjalr/index.html) the emulator ran all components cycle-by-cycle. 

The basic technique looks like this: 

  1. Find the next time that one component could affect another, such as the CPU writing to a PPU register or the PPU asserting an interrupt to the CPU.
  2. Run the least predictable component up to that time. In the NES's case, the CPU is least predictable, so you usually want to run that first.
  3. Run the other components up to that time.



At the end of each frame (for example, the start of scanline 0 or scanline 240), the emulator catches up everything and hands off the completed video surface and audio stream to the operating system. 

## Contents

  * 1 Prediction
  * 2 Timestamping
  * 3 Scanline-based emulation
  * 4 Rewind to checkpoint



## Prediction

One basic technique involves predicting when each component will do something "important", like asserting an interrupt or changing a status register, and then running one component ahead until that time. 

Some things can be predicted: 

  * Vertical blanking [NMI](NMI.xhtml "NMI")
  * Scanlines containing sprite 0 that might trigger sprite 0 hit
  * Scanlines containing at least 8 sprites that might trigger the overflow flag
  * IRQs from [APU Frame Counter](APU_Frame_Counter.xhtml "APU Frame Counter") and [APU DMC](APU_DMC.xhtml "APU DMC") completion
  * APU length counter status
  * Mapper IRQ, in many cases



An emulator might make a rough prediction that slightly underestimates the time until that component sees the change, run that component for that amount of time, and then fall back to I/O catch-up or cycle-by-cycle emulation until the "important" event has happened. 

## Timestamping

Another technique involves remembering at what time (that is, what cycle) the CPU has written to each register, and then having the other component process the write as if it had occurred at that cycle. 

But if a timestamp changes a prediction, you'll want to catch-up the other components instead of timestamping the write: 

  * Writes to [PPU registers](PPU_registers.xhtml "PPU registers") (especially $2004) might change the sprite 0 prediction.
  * Writes to mapper or PPU registers might change the mapper IRQ prediction.
  * Writes to APU registers might change the Frame IRQ prediction and the length counter predictions.



## Scanline-based emulation

A **scanline-based emulator** is an emulator that uses a crude form of prediction and timestamping: something "important" might happen on each scanline, and timestamps are rounded to a scanline boundary. They run the CPU for one scanline's worth of cycles and then run the PPU and mapper for one scanline (341 dots), and after all scanlines are finished, run the APU for one frame. This isn't perfect but can run "well-behaved" games efficiently on emulators designed for old PCs or handheld devices. Most mappers that generate interrupts do so at some predictable point in the scanline, and few games use the APU interrupt or write to the same APU register multiple times in a frame, except possibly to write raw sample values to [$4011](APU_DMC.xhtml "APU DMC"). 

## Rewind to checkpoint

This isn't as important for the NES, but in systems with multiple CPUs that can interrupt each other (like the Super NES with an SA1 coprocessor), prediction is far more difficult. So an emulator can set a checkpoint on CPU A, predict that CPU B will _not_ assert an interrupt, and run CPU A for that time. If it turns out that CPU B asserted an interrupt, the emulator can rewind CPU A to the checkpoint and run it until the time the interrupt occurred. 

Image processing performed in the system software of a PC or modern game console can insert one or possibly more frames of delay. So can the LCD television monitors connected to its video output. To maintain the illusion that a game is responsive to control by the player, some emulators run one or more frames ahead. Then whenever a button changes, the emulator rewinds a few frames with the updated input. 
