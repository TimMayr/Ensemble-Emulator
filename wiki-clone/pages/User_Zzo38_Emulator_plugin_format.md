# User:Zzo38/Emulator plugin format

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AZzo38/Emulator_plugin_format) | View [other pages](Special_AllPages.xhtml#User_Zzo38_Emulator_plugin_format)

## Mapper
    
    
    #define OPEN_BUS 0xFFFF
    #define CIRAM_BANK_0 0x8000
    #define CIRAM_BANK_1 0x8400
    #define BUS_CONFLICT 0x1000
    
    typedef struct NSF_Interface {
      // Provided by host
      U16(*read_audio)(Mapper*);
      void(*volume)(Mapper*,U8,U16);
      void(*write_vrc6)(Mapper*,U8,U8);
      void(*write_vrc7)(Mapper*,U8,U8);
      void(*write_fds)(Mapper*,U8,U8);
      void(*write_mmc5)(Mapper*,U8,U8);
      void(*write_n163)(Mapper*,U8,U8);
      void(*write_ym2149f)(Mapper*,U8,U8);
      U8(*fds_volume_gain)(Mapper*);
      U8(*fds_sweep_gain)(Mapper*);
      U8(*mmc5_status)(Mapper*);
      U8(*acknowledge)(Mapper*);
      void(*set_timer)(Mapper*,void(*)(Mapper*,void*),void*,U32);
      // There may be other fields here too, which are specific to the
      // emulator, such as functions to access the FDS disk.
    } NSF_Interface;
    
    typedef struct Mapper {
      // Provided by host
      NSF_Interface*nsf;
      void(*irq_write)(Mapper*,int);
      int(*irq_read)(Mapper*);
      const U8*prg_rom;
      U8*prg_ram;
      U8*prg_battery_ram;
      const U8*chr_rom;
      U8*chr_ram;
      U8*chr_battery_ram;
      // Provided by plugin
      void(*destroy)(Mapper*);
      void(*reset)(Mapper*);
      U16(*prg_read)(Mapper*,U16);
      U16(*prg_write)(Mapper*,U16,U8);
      U16(*chr_access)(Mapper*,U16);
      U16(*chr_read)(Mapper*,U16);
      U16(*chr_write)(Mapper*,U16,U8);
      void(*quick_save)(Mapper*,U8*);
      void(*quick_restore)(Mapper*,const U8*);
      void(*irq_trigger)(Mapper*,int);
    } Mapper;
    
    typedef struct Mapper_Info {
      U8 version; // R
      U8 mirroring; // R
      U16 id; // R
      U8 sub; // R
      U32 prg_rom_size; // R
      U32 prg_ram_size; // R/W
      U32 prg_battery_ram_size; // R/W
      U32 chr_rom_size; // R
      U32 chr_ram_size; // R/W
      U32 chr_battery_ram_size; // R/W
      FILE*ines; // R
      int save_size; // W
      U8 audio_ext; // W
    } Mapper_Info;
    
    int define_mapper(U16 id,const char*name,(Mapper*)(*init)(Mapper_Info*));
    

  * OPEN_BUS: Returned by some functions to indicate that there is no data in the cartridge, or that you are not writing to CIRAM.
  * CIRAM_BANK_0: Returned from some functions to indicate read/write the CIRAM when CIRAM A10 is low.
  * CIRAM_BANK_1: Returned from some functions to indicate read/write the CIRAM when CIRAM A10 is high.
  * BUS_CONFLICT: Should be bitwise OR by the other value to indicate that there is a bus conflict; the emulator can ignore it or keep track of a counter of bus conflicts.
  * Mapper: The structure to store the mapper data in the memory. It is possible for the plugin to add additional fields at the end. 
    * nsf: The NSF interface.
    * prg_rom: PRG ROM.
    * prg_ram: Non-battery PRG RAM.
    * prg_battery_ram: Battery PRG RAM.
    * chr_rom: CHR ROM.
    * chr_ram: Non-battery CHR RAM.
    * chr_battery_ram: Battery CHR RAM.
    * destroy: It should free the mapper data since it isn't used anymore. The emulator automatically frees the data that it initializes by itself. It might also be called before reset is called if the emulator is unable to use this mapper for some reason.
    * reset: Indicate a hard or soft reset (the first one is hard reset and the rest is soft reset).
    * prg_read: Read data from PRG address space. Normally, the high eight bits of the return should be cleared if there is a data to read on the cartridge.
    * prg_write: Write data to PRG address space.
    * chr_access: The address line of CHR is updated, even though it isn't read/write.
    * chr_read: Read data from CHR address space.
    * chr_write: Write data to CHR address space.
    * quick_save: Should write any data specific to the mapper (other than the RAM) into the buffer which is provided.
    * quick_restore: Should read any data specific to the mapper (other than the RAM) from the buffer which is provided.
  * NSF_Interface: Provide functions for expansion audio, audio input, timers, and CPU cycle counters. 
    * volume: The second parameter is the expansion to set the volume of; enter 0 to set the volume for the 2A03 audio and microphone audio, otherwise it should have one bit set which is the same as used in NSF. The third parameter is the volume level, where 32767 is normal and 0 is muted.
  * Mapper_Info: The structure of the information provided by the iNES header and emulator. 
    * version: Zero for "DiskDude" files, one for old iNES format, two for NES 2.0 format, 255 if this isn't applicable.
    * mirroring: Nametable mirroring mode.
    * id: The iNES mapper number.
    * sub: The submapper number. If it isn't NES 2.0, then this will be zero.
    * prg_rom_size: PRG ROM size in bytes.
    * prg_ram_size: PRG RAM size in bytes.
    * prg_battery_ram_size: PRG battery RAM size in bytes.
    * chr_rom_size: CHR ROM size in bytes.
    * chr_ram_size: CHR RAM size in bytes.
    * chr_battery_ram_size: CHR battery RAM size in bytes.
    * ines: File handle of iNES ROM image file. This might sometimes be not available for some reason.
    * save_size: The size of quick save data needed by this mapper.
    * audio_ext: Specify what audio expansion chip is used, in the same format as NSF. Note that, some emulator might ignore this specification.
  * define_mapper: Define a mapper. If there is more than one having the same ID number, it will try them in the reverse order of being defined until the init function returns nonzero. 
    * id: The NES 2.0 mapper number.
    * name: The name of the mapper in ASCII format.
    * init: The initialization function, which is called when hard reset (but not when soft reset). It should allocate the Mapper structure, write the necessary changes to Mapper_Info if applicable, and set up the "provided by plugin" fields and its own fields (the emulator will automatically set up the others). If it will return zero, then it shouldn't write anything to the Mapper_Info structure.



It is possible for the emulator to set the Mapper_Info structure other than how it says above, but only if some checksum database is used, and this is not recommended (unless the user specifically adds and enables it, perhaps). 

The following functions are allowed to be null: reset, prg_write, chr_access, irq_trigger. It is also allowed for quick_save and quick_restore to be null if the save_size is zero. Note that they may be called even if not writing a quick save file; they might also be called by a debugger included in the emulator. 

## Input device
    
    
    typedef struct InputDevice {
      // Provided by host
      U16 sound; // class 0 only
      U8 light;
      void(*irq_write)(InputDevice*,int); // class 0 only
      int(*irq_read)(InputDevice*); // class 0 only
      void(*display)(InputDevice*,UserDisplay*);
      // Provided by plugin
      void(*destroy)(InputDevice*);
      void(*connect)(InputDevice*,U8);
      void(*disconnect)(InputDevice*);
      void(*irq_trigger)(InputDevice*,int); // class 0 only
      U8(*read)(InputDevice*);
      U8(*read_other)(InputDevice*); // class 0 only
      void(*write)(InputDevice*,U8);
      void(*quick_save)(InputDevice*,U8*);
      void(*quick_restore)(InputDevice*,const U8*);
      // Provided by plugin -- MIDI events
      void(*note_off)(InputDevice*,U8,U8,U8);
      void(*note_on)(InputDevice*,U8,U8,U8);
      void(*key_pressure)(InputDevice*,U8,U8,U8);
      void(*parameter)(InputDevice*,U8,U8,U8);
      void(*program)(InputDevice*,U8,U8);
      void(*channel_pressure)(InputDevice*,U8,U8);
      void(*pitch_wheel)(InputDevice*,U8,U16);
      void(*sysex)(InputDevice*,const U8*,U32);
      void(*tune)(InputDevice*);
      void(*reset)(InputDevice*);
    } InputDevice;
    
    int define_input_device(const char*name,int class,int save_size,(InputDevice*)(*init)(void));

  * InputDevice: The structure to store the input device data in the memory. It is possible for the plugin to add additional fields at the end. 
    * sound: 2A03 audio mixed with microphone (no expansion audio).
    * light: Tell the light sense of the light gun.
    * irq_write: Set the IRQ pin of Famicom expansion port high or low, which will trigger IRQ in the CPU and the cartridge can also read it.
    * irq_read: Read the IRQ pin of Famicom expansion port, which is whatever the cartridge set it to. If it doesn't set, it is high.
    * display: To display data to the user.
    * destroy: Used to free the input device memory.
    * connect: The device is connected. The argument is the initial OUT0, OUT1, OUT2; if it isn't on the expansion port then only OUT0 is used.
    * disconnect: The device is disconnected.
    * irq_trigger: Used if the cartridge asserts the IRQ. The argument is the new value 0 or 1.
    * read: Called when the CPU request data from this port. Any unused pins should be high.
    * read_other: Called when the CPU request data of the bit1 of the $4016 register for devices connected to the expansion port. All other bits must be high.
    * write: Called when CPU set the OUT0, OUT1, OUT2 pins.
    * quick_save: Should write any data specific to this input device into the buffer which is provided.
    * quick_restore: Should read any data specific to this input device from the buffer which is provided.
    * note_off: MIDI message for note off. Parameter is channel, key, velocity. (Note for this and other MIDI messages: Please use only the low bit of the channel number; the other three bits are used for the emulator's purpose.)
    * note_on: MIDI message for note on. Parameter is channel, key, velocity.
    * key_pressure: MIDI message for key pressure. Parameter is channel, key, pressure.
    * parameter: MIDI message for parameter. Parameter is channel, control number, control data.
    * program: MIDI message for program change.
    * channel_pressure: MIDI message for channel pressure.
    * pitch_wheel: MIDI message for pitch wheel.
    * sysex: MIDI system exclusive message. The data and length is specified as the parameter.
    * tune: MIDI tune request message.
    * reset: MIDI reset message.
  * UserDisplay: A structure to display information to the user.
  * define_input_device: Add a new input device into the emulator. 
    * name: The name of a input device in ASCII format.
    * class: Specify the device class. 0=Famicom expansion port, 1=NES controller port, 2=NES controller port with only bit0, 3=NES controller port with only bit0 and only eight data in the latch like the standard controller protocol. If a class 1, 2, 3, is connected to expansion port then it is player 2 device.
    * save_size: Size of data for quick save.
    * init: Should allocate a InputDevice structure and set up the "provided by plugin" fields and its own fields (the emulator will automatically set up the others).



The emulator will have its own way to somehow convert the user input into MIDI format; for example, by configuration dialog box, configuration files, some combination of this, or others. 

(The reason it is MIDI is that different emulators may have different physical input, whether it is keyboard, mouse, joysticks, or different devices, or even the same thing may differ by library (SDL, Allegro, DirectX, etc), and even if it is the keyboard different emulators may have different configuration of the keys (probably set up by the user), or to select different devices when scroll lock is enabled, etc; also, making it MIDI allows a unified format for input recordings (if the deltas are clock cycles, and text events can be used to select which input devices are being used and so on), and allows the data to be processed by other programs, converted using MIDI yoke (so that one program and output and the emulator can input it), attached to a MIDI input and/or output port on your computer, etc.) 

The following functions are allowed to be null: connect, disconnect, irq_trigger, read_other. MIDI events can also be null, and so can quick_save and quick_restore if the save_size is zero. 
