# Family Computer Network Adapter

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Family_Computer_Network_Adapter) | View [other pages](Special_AllPages.xhtml#Family_Computer_Network_Adapter)

The Family Computer Network Adapter (HVC-035) is an unreleased Famicom modem used with the [Family Computer Disk System](Family_Computer_Disk_System.xhtml "Family Computer Disk System") via the RAM adapter's expansion port. A controller similar to the [Famicom Network Controller](Famicom_Network_Controller.xhtml "Famicom Network Controller") attaches directly to the adapter, and like the later-released [Famicom Network System](Family_Computer_Network_System.xhtml "Famicom Network System"), the adapter provides kanji graphics for use by software. A speaker is also built in to the unit. 

## Contents

  * 1 Interface
    * 1.1 Kanji graphics
    * 1.2 Controller
      * 1.2.1 Layout
      * 1.2.2 Report
    * 1.3 Packets
      * 1.3.1 Send
      * 1.3.2 Receive
  * 2 References



## Interface

The adapter uses the FDS expansion port, which is mapped to the low 7 bits of $4026 for writes and $4033 for reads. These bits are bidirectional and each side can pull them low. As such, to be able to receive data on a bit, the console must have first set that bit. 
    
    
    6 bit  0
    --- ----
    PDC ARWK
    ||| ||||
    ||| |||+- Kanji/controller data
    ||| ||+-- Kanji index write clock, and
    ||| ||    Kanji/controller read selector (0 = kanji, 1 = controller)
    ||| |+--- Kanji/controller read clock
    ||| +---- Adapter ready for packet transfer (0 = ready), and
    |||       Adapter packet transfer clock
    ||+------ Console ready for packet transfer (0 = ready), and
    ||        Console packet transfer clock
    |+------- Packet data
    +-------- Packet available (0 = available)
    

Note below that all bits modified in the same step should be written in a single write. 

### Kanji graphics

The adapter provides 4096 16x16 1-bit-per-pixel characters, presumed to be mostly kanji. The console writes a 12-bit index to the adapter and then reads the corresponding 32-byte character back. The tile order of the returned graphics is top left, bottom left, top right, bottom right. 

To write the kanji index: 

  1. Set bit 2 and clear bit 1.
  2. Write index bit (low bit first) to bit 0.
  3. Set bit 1.
  4. Clear bit 1.
  5. Repeat from step 2 until 12-bit index is written.



To read the kanji data: 

  1. (Bit 2 should be set and bit 1 should be clear from writing the index.)
  2. Set bit 0.
  3. Clear bit 2.
  4. Read data bit (low bit first) from bit 0.
  5. Set bit 2.
  6. Repeat from step 3 until 32-byte data is read.



Available software converts Shift-JIS characters in the ranges $8000-9FFF and $E000-EBFF to adapter indices. Based on the resulting mapping, it appears the first half (128 KB) of the [Network System's kanji ROM](Family_Computer_Network_System.xhtml#LH5323M1_Kanji_Graphic_ROM "Famicom Network System") is compatible. 

### Controller

The adapter uses a 23-button controller with a D-pad and keypad. Though its appearance and layout are similar to the [Famicom Network Controller](Famicom_Network_Controller.xhtml "Famicom Network Controller"), this controller differs in that it connects directly to the adapter and has a unique report. 

To read the controller: 

  1. Set bit 2, clear bit 1, and set bit 0.
  2. Set bit 1.
  3. Clear bit 2.
  4. Read data bit from bit 0.
  5. Set bit 2.
  6. Repeat from step 3 until 24-bit report is read.



#### Layout
    
    
    .------------------------------------------.
    | [<]   [>]  [1] [2] [3] [*] [C]   [END]   |
    |PREV _ NEXT                               |----------/////
    |   _| |_    [4] [5] [6] [#] [.]   [TOC]   |
    |  |_   _|                                 |
    |    |_|     [7] [8] [9] [  0  ]  [EXEC ]  |
    '------------------------------------------'
    

The controller's layout approximately matches that of the [Famicom Network Controller](Famicom_Network_Controller.xhtml "Famicom Network Controller"), but it does not name A, B, select, or start buttons, and its equivalent of the B and A buttons are stacked vertically rather than horizontally. The button names are otherwise the same. The buttons themselves are square with raised round centers rather than simply being round. The cable enters the controller on the right side and is coiled, like that of a telephone handset. When not in use, the controller is stored face down in a recessed region on the front of the Network Adapter. The controller connects directly to the Network Adapter and can be detached, but nothing is known about its connector. 

#### Report
    
    
     0 - 前ページ (Previous page)
     1 - 次ページ (Next page)
     2 - 1
     3 - 2
     4 - 3
     5 - *
     6 - C
     7 - 通信終了 (End communication)
     8 - (D-pad axis 1)
     9 - (D-pad axis 2)
    10 - 4
    11 - 5
    12 - 6
    13 - #
    14 - .
    15 - 目次 (Table of contents)
    16 - (D-pad axis 2)
    17 - (D-pad axis 1)
    18 - 7
    19 - 8
    20 - 9
    21 - 0
    22 - 実行 (Execute)
    23 - (Always 0)
    

The bit order appears to reflect the physical layout of the controller and is not compatible with the [standard controller](Standard_controller.xhtml "Standard controller"). The D-pad directions have not yet been identified. It is not known what is returned after 24 reads. Available software nullifies the input if 2 or more buttons are down unless just D-pad directions are down. 

### Packets

A bidirectional interface is used to transfer packets between the console and adapter. These packets are terminated when byte value $1F is seen. The console software transfers in batches of up to 20 bytes at a time and picks up where it left off the next frame, as necessary. The format and functionality of these packets is not understood. 

#### Send

To send a packet to the adapter: 

  1. Verify bit 6 and bit 3 are set. If not, receive the pending packet, instead.
  2. Clear bit 4.
  3. Verify bit 6 and bit 3 are set. If not, set bit 4 and receive the pending packet, instead.
  4. Clear bit 6.
  5. Wait for bit 3 to toggle. 
     * If toggle does not occur soon enough, fail.
  6. Toggle bit 4 and write data bit (low bit first) to bit 5.
  7. Repeat from step 5 until the last byte sent had value $1F.
  8. Wait for bit 3 to become clear. 
     * If clear does not occur soon enough, fail.
  9. Set bit 6, set bit 5, and set bit 4.
  10. Wait for bit 3 to become set. 
     * If set does not occur soon enough, fail.



If the send fails: 

  1. Set bit 6, set bit 5, and set bit 4.



#### Receive

If bit 6 and bit 4 are both 0, the adapter is ready to send a packet. To receive a packet from the adapter: 

  1. Clear bit 4.
  2. Wait for bit 3 to toggle. 
     * If bit 6 becomes set while waiting, fail.
     * If toggle does not occur soon enough, fail.
  3. Read data bit (low bit first) from bit 5.
  4. Toggle bit 4.
  5. Repeat from step 2 until the last byte received had value $1F.
  6. Wait for bit 6 and bit 3 to be set. 
     * If both bits do not set soon enough, fail.
  7. Set bit 4.



If the receive fails: 

  1. Set bit 4.



## References

    

  * [BigAfroDogg's blog post with FCNA images](https://bigafrodogg.hatenablog.com/entry/2020/05/11/064428)


