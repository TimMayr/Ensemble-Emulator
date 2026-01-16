# Infrared controllers

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Infrared_controllers) | View [other pages](Special_AllPages.xhtml#Infrared_controllers)

A few famiclones are equipped with infrared receiver and are paired with additional battery powered wireless joypad(s) with infrared transmitters. Most of them can be configured to work as P1/P2 or even execute special functions (console restart, power down). They use special proprietary chips to handle the communication, whose protocols are not compatible with each other. 

  


## Contents

  * 1 UA6580 + UA6581
  * 2 FLY046 + FLY047
  * 3 FLY-826A + FLY-827A
  * 4 RTS703 + RTS702 + RTS705
  * 5 ?? + 74126



## UA6580 + UA6581
    
    
              ,---V---.                                                          
         VCC--|01   18|->!LED                                                    
              |02   17|<-!RIGHT                ,---V---.                         
              |03   16|<-!LEFT        !RESET?  |01   14|->VCC                    
     !P1!/P2->|04   15|<-!DOWN         delay?  |02   13|->P2 inactivity pulse?   
       XTAL1->|05   14|<-!UP            XTAL1->|03   12|->P1 inactivity pulse?   
       XTAL2->|06   13|<-!START         XTAL2->|04   11|->P2_D0                  
              |07   12|<-!SELECT       P1_CLK->|05   10|<-P2_CLK                 
          IR<-|08   11|<-!A             P1_D0<-|06   09|<-OUT0
         GND--|09   10|<-!B               GND -|07   08|<-IR
              `-------`                        `-------`                         
            UA6580 (transmitter)            UA6581 (receiver)                    
    

They can be found in _Bentech Computer Game Machine_ console and _DR Super_ joypad. 

  * More info: [https://forums.nesdev.org/viewtopic.php?f=9&t=16872](https://forums.nesdev.org/viewtopic.php?f=9&t=16872)



## FLY046 + FLY047
    
    
                ,---V---.                                                        
           !UP->|01   20|<-!DOWN
        !START->|02   19|<-!LEFt                                                 
       !SELECT->|03   18|<-!RIGHT                ,---V---.                       
            !B->|04   17|<-!2P/1P        P1_CLK->|01   14|
            !A->|05   16|--VCC           P2_CLK->|02   13|<-OUT0
     !TURBO_ON->|06   15|<-!OFF/ON               |03   12|<-IR
         XTAL1->|07   14|--GND                   |04   11|-VCC 
         XTAL2->|08   13|--VCC              GND--|05   10|<-XTAL2
       !REDLED<-|09   12|->IR2            P1_D0<-|06   09|<-XTAL1
           GND--|10   11|->IR1            P2_D0  |07   08|
                `-------`                        `-------`
     		FLY046 (transmitter)          FLY047 (receiver)
    

They can be found in _Micro Genius IQ1000_ console and _Micro Genius TIJ-309_ joypad. 

  * More info: [https://forums.nesdev.org/viewtopic.php?f=9&t=18984](https://forums.nesdev.org/viewtopic.php?f=9&t=18984)



## FLY-826A + FLY-827A
    
    
               ,---v---.
     !REDLED <-|01   20|-- +5V                                   ,---v---.
     !SLOW   ->|02   19|-> !IR                         IRLED   ->|01   18|-> ?
     !LEFT   ->|03   18|-- XTAL2                           ?   --|02   17|-- ?
     !UP     ->|04   17|-- XTAL1 (455 kHZ)   XTAL1 455k (sine) ->|03   16|<- !RESET
     !RIGHT  ->|05   16|<- !SELECT           XTAL2 455k (sq)   <-|04   15|-- ?
     !DOWN   ->|06   15|<- !START                power strobe  <-|05   14|-- +5V
     TURBOB  <-|07   14|<- !A                          P2 D0   <-|06   13|<- P1 CLK
     TURBOA  <-|08   13|<- !B                          P2 CLK  ->|07   12|<- OUT0
     !2P/1P  ->|09   12|-- GND                   reset strobe  <-|08   11|-> P1 D0
     !FUN    ->|10   11|-- ?                               ?   --|09   10|-- GND
               +-------+                                         +-------+
            FLY-826A (transmitter)                             FLY-827A (receiver) 
    

They can be found in _Micro Genius IQ2000_ console and _Micro Genius TIJ-325_ joypad. 

  * More info: [https://forums.nesdev.org/viewtopic.php?f=9&t=18984](https://forums.nesdev.org/viewtopic.php?f=9&t=18984)



  


## RTS703 + RTS702 + RTS705
    
    
             ,---V---.                        ,---V---.                        ,---V---.          
     !P2/P1->|01   16|--VCC            RESET->|01   16|->B             P2_CLK->|01   16|<-P1_CLK
        !UP->|02   15|->LED           ? TEST--|02   15|->A              XTAL1->|02   15|->LED
      !DOWN->|03   14|->IR             XTAL1->|03   14|->SELECT         XTAL2->|03   14|          
        GND--|04   13|->TURBO          XTAL2->|04   13|->START                 |04   13|->P2_D0
     !RIGHT->|05   12|<-!B               GND--|05   12|->UP                    |05   12|->P1_D0
      !LEFT->|06   11|<-!A                IR->|06   11|->DOWN                  |06   11|--GND
      XTAL1->|07   10|<-!SELECT          VCC--|07   10|->P1/P2             IR->|07   10|          
      XTAL2->|08   09|<-!START          LEFT<-|08   09|->RIGHT          +4.3V->|08   09|<-OUT0
             `-------`                        `-------`                        `-------`          
         RTS703 (transmitter)          RTS702 (parallel receiver)        RTS705 (serial receiver)  
    

RTS705 - pins: 4,5,6,10,14 has no internal connection inside (multimeter diode test to VCC/GND) 

  * More info: [[1]](https://forums.nesdev.org/viewtopic.php?f=18&t=17105), [[2]](https://forums.nesdev.org/viewtopic.php?p=283868#p283868)



## ?? + 74126

Receiver is integrated with _Gameinis Ping Pong_ cartridge and transmitter is believed to be part of a ping pong rocked-shaped joypad. 

  * More info: 
    * <https://forums.nesdev.org/viewtopic.php?t=16657>
    * <http://zc-infinity.blogspot.com/2018/01/knockoff-console-corner-virtual-station.html>



Categories: [Pinouts](Category_Pinouts.xhtml)
