# Input devices

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Input_devices) | View [other pages](Special_AllPages.xhtml#Input_devices)

The NES has two [general-purpose controller ports](Controller_port_pinout.xhtml "Controller port pinout") on the front of the console, as well as a (rarely used) [48-pin expansion port](Expansion_port.xhtml "NES expansion port pinout") underneath. 

The Famicom's standard controllers are hardwired to the front of the unit, and a special [15-pin expansion port](Expansion_port.xhtml "Famicom expansion port pinout") is commonly used for third-party controllers. The AV Famicom, however, features detachable controllers using the same ports as the NES. 

The NES and Famicom have a set of I/O ports used for controllers and other peripherals, consisting of the following: 

  * One output port, 3 bits wide, accessible by writing the bottom 3 bits of $4016. 
    * The values latched by $4016/write appear on the [OUT0-OUT2](CPU_pinout.xhtml "CPU pin out and signal description") output pins of the 2A03/07, where OUT0 is routed to the controller ports and OUT0-OUT2 to the expansion port on the NES.
  * Two input ports, each 5 bits wide, accessible by reading the bottom 5 bits of $4016 and $4017. Reading $4016 and $4017 activates the [/OE1 and /OE2](CPU_pinout.xhtml "CPU pin out and signal description") signals, respectively, which are routed to the controller ports and the expansion port. 
    * On the NES, only D0, D3, and D4 are connected to both controller ports, while all of D0-D4 are connected to the expansion port.
    * On the original Famicom, the two ports differ: $4016 D0 and D2 and $4017 D0 are permanently connected to both controllers, while $4016 D1 and all of $4017's D0-D4 are connected to the expansion port.
    * On the AV Famicom, only D0 is connected to the controller ports. The expansion port is unchanged.



## Contents

  * 1 Programmer's reference
  * 2 Hardware
  * 3 Other I/O devices
  * 4 Usage of port pins by hardware type



## Programmer's reference

  * [Controller reading](Controller_reading.xhtml "Controller reading")



## Hardware

  * [Controller port pinout](Controller_port_pinout.xhtml "Controller port pinout")
  * Controllers 
    * **NES[Standard controller](Standard_controller.xhtml "Standard controller")**
    * [Arkanoid controller](Arkanoid_controller.xhtml "Arkanoid controller")
    * [Bandai Hyper Shot](https://www.nesdev.org/w/index.php?title=Bandai_Hyper_Shot&action=edit&redlink=1 "Bandai Hyper Shot \(page does not exist\)")
    * [Coconuts Pachinko](Coconuts_Japan_Pachinko_Controller.xhtml "Coconuts Pachinko")
    * [Doremikko Keyboard](https://www.nesdev.org/w/index.php?title=Doremikko_Keyboard&action=edit&redlink=1 "Doremikko Keyboard \(page does not exist\)")
    * [Exciting Boxing Punching Bag](Exciting_Boxing_Punching_Bag.xhtml "Exciting Boxing Punching Bag")
    * [Family BASIC Keyboard](Family_BASIC_Keyboard.xhtml "Family BASIC Keyboard")
    * [Four Score, NES Satellite](Four_player_adapters.xhtml "Four Score") 4-player adapters
    * [Hori 4 Players Adapter](Four_player_adapters.xhtml "Hori 4 Players Adapter") for Famicom
    * [Jissen Mahjong controller](Jissen_Mahjong_controller.xhtml "Jissen Mahjong controller")
    * [Konami Hyper Shot](Konami_Hyper_Shot.xhtml "Konami Hyper Shot")
    * [Miracle Piano](Miracle_Piano.xhtml "Miracle Piano")
    * [Mouse](Mouse.xhtml "Mouse") (SNES Mouse, Subor Mouse)
    * [Oeka Kids tablet](Oeka_Kids_tablet.xhtml "Oeka Kids tablet")
    * [Partytap](Partytap.xhtml "Partytap")
    * [Pokkun Moguraa Mat](https://www.nesdev.org/w/index.php?title=Pokkun_Moguraa_Mat&action=edit&redlink=1 "Pokkun Moguraa Mat \(page does not exist\)")
    * [Power Glove](https://www.nesdev.org/w/index.php?title=Power_Glove&action=edit&redlink=1 "Power Glove \(page does not exist\)")
    * [Power Pad](Power_Pad.xhtml "Power Pad")
    * [RacerMate Bicycle](https://www.nesdev.org/w/index.php?title=RacerMate_Bicycle&action=edit&redlink=1 "RacerMate Bicycle \(page does not exist\)")
    * [SNES controller](SNES_controller.xhtml "SNES controller")
    * [Top Rider Bike](https://www.nesdev.org/w/index.php?title=Top_Rider_Bike&action=edit&redlink=1 "Top Rider Bike \(page does not exist\)")
    * [U-Force](https://www.nesdev.org/w/index.php?title=U-Force&action=edit&redlink=1 "U-Force \(page does not exist\)")
    * [Virtual Boy controller](Virtual_Boy_controller.xhtml "Virtual Boy controller")
    * [Zapper](Zapper.xhtml "Zapper") lightgun
  * [Infrared controllers](Infrared_controllers.xhtml "Infrared controllers")



## Other I/O devices

  * [Famicom 3D glasses](Famicom_3D_System.xhtml "Famicom 3D glasses")
  * [Family BASIC Data Recorder](Family_BASIC_Data_Recorder.xhtml "Family BASIC Data Recorder")
  * [R.O.B.](R_O_B_.xhtml "R.O.B.")
  * [Battle Box](Battle_Box.xhtml "Battle Box")
  * [Turbo File](Turbo_File.xhtml "Turbo File")
  * [Barcode Battler](https://www.nesdev.org/w/index.php?title=Barcode_Battler&action=edit&redlink=1 "Barcode Battler \(page does not exist\)")
  * [TV-NET Rank 2 controller](TV_NET_Rank_2_controller.xhtml "TV-NET Rank 2 controller")
  * [FAM-NET Keyboard](FAM_NET_Keyboard.xhtml "FAM-NET Keyboard")
  * [Family Computer Network Adapter](Family_Computer_Network_Adapter.xhtml "Family Computer Network Adapter")
  * [Super Famicom NTT Data Keypad](Super_Famicom_NTT_Data_Keypad.xhtml "Super Famicom NTT Data Keypad")



## Usage of port pins by hardware type

type  | output  | Joypad 1  | Joypad 2  | audio output   
---|---|---|---|---  
signal  | OUT2  | OUT1  | OUT0  | /OE1  | D4  | D3  | D2  | D1  | D0  | /OE2  | D4  | D3  | D2  | D1  | D0  | AUDIO   
access method  | write $4016  | [1] | read $4016  | [2] | read $4017  |   
available on these ports  |  |  |  |  |  |   
Controller port 1 (AV Famicom)  |  |  | OUT0  | /OE1  |  |  |  |  | D0  |  |  |  |  |  |  |   
Controller port 1 (Famicom (internal))  |  |  | OUT0  | /OE1  |  |  |  |  | D0  |  |  |  |  |  |  |   
[Controller port](Controller_port_pinout.xhtml "Controller port pinout") 1 (NES)  |  |  | OUT0  | /OE1  | D4  | D3  |  |  | D0  |  |  |  |  |  |  |   
Controller port 2 (AV Famicom)  |  |  | OUT0  |  |  |  |  |  |  | /OE2  |  |  |  |  | D0  |   
Controller port 2 (Famicom (internal))  |  |  | OUT0  |  |  |  | D2  |  |  | /OE2  |  |  |  |  | D0  |   
[Controller port](Controller_port_pinout.xhtml "Controller port pinout") 2 (NES)  |  |  | OUT0  |  |  |  |  |  |  | /OE2  | D4  | D3  |  |  | D0  |   
[Expansion port](Expansion_port.xhtml#Famicom "Expansion port") (Famicom)  | OUT2  | OUT1  | OUT0  | /OE1  |  |  |  | D1  |  | /OE2  | D4  | D3  | D2  | D1  | D0  | AUDIO   
[Expansion port](Expansion_port.xhtml#NES "Expansion port") (NES)  | OUT2  | OUT1  | OUT0  | /OE1  | D4  | D3  | D2  | D1  | D0  | /OE2  | D4  | D3  | D2  | D1  | D0  | AUDIO   
used by these devices  |  |  |  |  |  |   
[Controller](Standard_controller.xhtml "Standard controller") (port 1)[3] |  |  | OUT0  | /OE1  |  |  |  |  | D0  |  |  |  |  |  |  |   
[Controller](Standard_controller.xhtml "Standard controller") (port 2)[3] |  |  | OUT0  |  |  |  |  |  |  | /OE2  |  |  |  |  | D0  |   
[Controller](Standard_controller.xhtml "Standard controller") (Famicom controller 2)[4] |  |  | OUT0  |  |  |  | D2  |  |  | /OE2  |  |  |  |  | D0  |   
[Controller](Standard_controller.xhtml "Standard controller") (expansion port)  |  |  | OUT0  | /OE1  |  |  |  | D1  |  |  |  |  |  |  |  | AUDIO[5]  
[Arkanoid controller](Arkanoid_controller.xhtml "Arkanoid controller") (port 2)[3] |  |  | OUT0  |  |  |  |  |  |  | /OE2  | D4  | D3  |  |  |  |   
[Arkanoid controller](Arkanoid_controller.xhtml "Arkanoid controller") (expansion port)  |  |  | OUT0  |  |  |  |  | D1  |  | /OE2  |  |  |  | D1  |  |   
[Arkanoid II controller](Arkanoid_controller.xhtml#Arkanoid_II_expansion_port "Arkanoid controller") (2 controllers)  |  |  | OUT0  |  |  |  |  | D1  |  | /OE2  | D4  | D3  |  | D1  |  |   
[Bandai Hyper Shot](https://www.nesdev.org/w/index.php?title=Bandai_Hyper_Shot&action=edit&redlink=1 "Bandai Hyper Shot \(page does not exist\)") | OUT2  | OUT1  | OUT0  | /OE1  |  |  |  | D1  |  |  | D4  | D3  |  |  |  | AUDIO   
[Exciting Boxing Punching Bag](Exciting_Boxing_Punching_Bag.xhtml "Exciting Boxing Punching Bag") |  | OUT1  |  |  |  |  |  |  |  |  | D4  | D3  | D2  | D1  |  |   
[FAM-NET Keyboard](FAM_NET_Keyboard.xhtml "FAM-NET Keyboard") | OUT2  | OUT1  | OUT0  |  |  |  |  |  |  |  | D4  | D3  | D2  | D1  |  |   
[Family Trainer Mat](Power_Pad.xhtml#Family_Trainer_Mat "Power Pad") | OUT2  | OUT1  | OUT0  |  |  |  |  |  |  |  | D4  | D3  | D2  | D1  |  |   
[Family BASIC Keyboard](Family_BASIC_Keyboard.xhtml "Family BASIC Keyboard") | OUT2  | OUT1  | OUT0  |  |  |  |  |  |  |  | D4  | D3  | D2  | D1  |  |   
[Famicom 3D System](Famicom_3D_System.xhtml "Famicom 3D System") |  | OUT1  |  |  |  |  |  |  |  |  |  |  |  |  |  |   
[Famicom Network System controller](Famicom_Network_Controller.xhtml "Famicom Network Controller") |  |  | OUT0  | /OE1  |  |  |  | D1  |  |  |  |  |  |  |  |   
Four player adapter ([Four Score](Four_player_adapters.xhtml "Four Score"))  |  |  | OUT0  | /OE1  |  |  |  |  | D0  | /OE2  |  |  |  |  | D0  |   
Four player adapter ([Hori 4 Players Adapter](Four_player_adapters.xhtml "Four Score"))  |  |  | OUT0  | /OE1  |  |  |  | D1  |  | /OE2  |  |  |  | D1  |  |   
[Hori Track](Hori_Track.xhtml "Hori Track") |  |  | OUT0  |  |  |  |  |  |  | /OE2  |  |  |  | D1  |  |   
[Jissen Mahjong controller](Jissen_Mahjong_controller.xhtml "Jissen Mahjong controller") | OUT2  | OUT1  | OUT0  |  |  |  |  |  |  | /OE2  |  |  |  | D1  |  |   
[Konami Hyper Shot](Konami_Hyper_Shot.xhtml "Konami Hyper Shot") | OUT2  | OUT1  |  |  |  |  |  |  |  |  | D4  | D3  | D2  | D1  |  |   
[Oeka Kids tablet](Oeka_Kids_tablet.xhtml "Oeka Kids tablet") |  | OUT1  | OUT0  |  |  |  |  |  |  |  |  | D3  | D2  |  |  |   
[Pachinko controller](Coconuts_Japan_Pachinko_Controller.xhtml "Coconuts Japan Pachinko Controller") |  |  | OUT0  | /OE1  |  |  |  | D1  |  |  |  |  |  |  |  |   
[Party Tap](Partytap.xhtml "Party Tap") |  |  | OUT0  |  |  |  |  |  |  | /OE2  | D4  | D3  | D2  |  |  |   
[Power Pad](Power_Pad.xhtml "Power Pad") (port 2)[3] |  |  | OUT0  |  |  |  |  |  |  | /OE2  | D4  | D3  |  |  |  |   
[Port test controller](Port_test_controller.xhtml "Port test controller") |  |  | OUT0  | /OE1  | D4  | D3  |  |  | D0  | /OE2  | D4  | D3  |  |  | D0  |   
[TV-NET controller](TV_NET_controller.xhtml "TV-NET controller") |  |  | OUT0  | /OE1  |  |  |  | D1  |  |  |  |  |  |  |  |   
[TV-NET Rank 2 controller](TV_NET_Rank_2_controller.xhtml "TV-NET Rank 2 controller") |  |  | OUT0  | /OE1  |  |  |  | D1  |  |  |  |  |  |  |  |   
[Zapper](Zapper.xhtml "Zapper") (port 2)  |  |  |  |  |  |  |  |  |  |  | D4  | D3  |  |  |  |   
[Zapper](Zapper.xhtml "Zapper") (expansion port)  |  |  |  |  |  |  |  |  |  |  | D4  | D3  |  |  |  | [6]  
[Zapper](Zapper.xhtml "Zapper") (Vs. System) (port 1)[3] |  |  | OUT0  | /OE1  |  |  |  |  | D0  |  |  |  |  |  |  |   
  
  1. ↑ /OE1 is activated by reading $4016.
  2. ↑ /OE2 is activated by reading $4017.
  3. ↑ 3.0 3.1 3.2 3.3 3.4 Controllers using NES ports can be plugged into either port, using that port's /OE and data lines. However, games may expect a controller to only be in a specific port.
  4. ↑ The Famicom controller 2 has a microphone that sends audio input over $4016 D2. This is not affected by OUT0 nor /OE2.
  5. ↑ A Famicom expansion controller may connect the audio output signal to a headphone jack (for example: IQ502 joypad).
  6. ↑ The Casel Zapper plays audio when the trigger is pulled, but this is done entirely by the controller independent of the console's audio out.


