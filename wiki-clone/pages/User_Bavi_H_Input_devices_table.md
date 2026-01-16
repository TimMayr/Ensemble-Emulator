# User:Bavi H/Input devices table

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3ABavi_H/Input_devices_table) | View [other pages](Special_AllPages.xhtml#User_Bavi_H_Input_devices_table)

signal  | output  | Joypad 1  | Joypad 2  | audio output   
---|---|---|---|---  
OUT2  | OUT1  | OUT0  | /OE1  | D4  | D3  | D2  | D1  | D0  | /OE2  | D4  | D3  | D2  | D1  | D0  | AUDIO   
access method  | write $4016  | [1] | read $4016  | [2] | read $4017  |   
available at  |  |  |  |  |  |   
NES [controller port](Controller_port_pinout.xhtml "Controller port pinout") 1  |  |  | yes  | yes  | yes  | yes  |  |  | yes  |  |  |  |  |  |  |   
Famicom controller 1 internal connection  |  |  | yes  | yes  |  |  |  |  | yes  |  |  |  |  |  |  |   
AV Famicom controller port 1  |  |  | yes  | yes  |  |  |  |  | yes  |  |  |  |  |  |  |   
NES [controller port](Controller_port_pinout.xhtml "Controller port pinout") 2  |  |  | yes  |  |  |  |  |  |  | yes  | yes  | yes  |  |  | yes  |   
Famicom controller 2 internal connection  |  |  | yes  |  |  |  | yes  |  |  | yes  |  |  |  |  | yes  |   
AV Famicom controller port 2  |  |  | yes  |  |  |  |  |  |  | yes  |  |  |  |  | yes  |   
NES [expansion port](Expansion_port.xhtml#NES "Expansion port") | yes  | yes  | yes  | yes  | yes  | yes  | yes  | yes  | yes  | yes  | yes  | yes  | yes  | yes  | yes  | yes   
Famicom [expansion port](Expansion_port.xhtml#Famicom "Expansion port") | yes  | yes  | yes  | yes  |  |  |  | yes  |  | yes  | yes  | yes  | yes  | yes  | yes  | yes   
used by  |  |  |  |  |  |   
[Controller](Standard_controller.xhtml "Standard controller") 1  |  |  | yes  | yes  |  |  |  |  | yes  |  |  |  |  |  |  |   
[Controller](Standard_controller.xhtml "Standard controller") 2  |  |  | yes  |  |  |  | [3] |  |  | yes  |  |  |  |  | yes  |   
Famicom expansion [controller](Standard_controller.xhtml "Standard controller") 1  |  |  | yes  | yes  |  |  |  | yes  |  |  |  |  |  |  |  | [4]  
Famicom expansion [controller](Standard_controller.xhtml "Standard controller") 2  |  |  | yes  |  |  |  |  |  |  | yes  |  |  |  | yes  |  |   
NES [Four Score](Four_player_adapters.xhtml "Four Score") controller 1 & 3  |  |  | yes  | yes  |  |  |  |  | yes  |  |  |  |  |  |  |   
NES [Four Score](Four_player_adapters.xhtml "Four Score") controller 2 & 4  |  |  | yes  |  |  |  |  |  |  | yes  |  |  |  |  | yes  |   
[Hori 4 Players Adapter](Four_player_adapters.xhtml "Four Score") controller 1 & 3  |  |  | yes  | yes  |  |  |  | yes  |  |  |  |  |  |  |  |   
[Hori 4 Players Adapter](Four_player_adapters.xhtml "Four Score") controller 2 & 4  |  |  | yes  |  |  |  |  |  |  | yes  |  |  |  | yes  |  |   
NES [Zapper](Zapper.xhtml "Zapper") in port 2  |  |  |  |  |  |  |  |  |  |  | yes  | yes  |  |  |  |   
Famicom [Zapper](Zapper.xhtml "Zapper") |  |  |  |  |  |  |  |  |  |  | yes  | yes  |  |  |  | [5]  
NES [Power Pad](Power_Pad.xhtml "Power Pad") in port 2  |  |  | yes  |  |  |  |  |  |  | yes  | yes  | yes  |  |  |  |   
[Family Trainer Mat](Power_Pad.xhtml#Family_Trainer_Mat "Power Pad") | yes  | yes  | yes  |  |  |  |  |  |  |  | yes  | yes  | yes  | yes  |  |   
NES [Arkanoid controller](Arkanoid_controller.xhtml "Arkanoid controller") in port 2  |  |  | yes  |  |  |  |  |  |  | yes  | yes  | yes  |  |  |  |   
Famicom [Arkanoid controller](Arkanoid_controller.xhtml "Arkanoid controller") |  | yes  |  |  |  |  |  | yes  |  | yes  |  |  |  | yes  |  |   
[Family BASIC Keyboard](Family_BASIC_Keyboard.xhtml "Family BASIC Keyboard") | yes  | yes  | yes  |  |  |  |  |  |  |  | yes  | yes  | yes  | yes  |  |   
[Famicom 3D glasses](Famicom_3D_System.xhtml "Famicom 3D glasses") |  | yes  |  |  |  |  |  |  |  |  |  |  |  |  |  |   
  
  1. ↑ /OE1 is activated by reading $4016
  2. ↑ /OE2 is activated by reading $4017
  3. ↑ $4016 D2 is used by Famicom controller 2 for microphone input.
  4. ↑ A Famicom expansion controller may connect the audio output signal to a headphone jack (for example: IQ502 joypad).
  5. ↑ A Famicom zapper may use the audio output signal to emit gunshot sounds (for example: Casel Zapper).


