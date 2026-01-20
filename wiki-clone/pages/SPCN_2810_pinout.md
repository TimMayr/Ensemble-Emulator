# SPCN 2810 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/SPCN_2810_pinout) | View [other pages](Special_AllPages.xhtml#SPCN_2810_pinout)
    
    
    SPCN 2810 (from 8752, DIL28-600)
              .---v--.
     !RESET-> |01  28| +5V
    CPU A12-> |02  27| <- CPU A14
    CPU A7 -> |03  26| <- CPU A13
    CPU A6 -> |04  25| <- CPU A8
    CPU A5 -> |05  24| <- CPU R/!W
    CPU A4 -> |06  23| -> !IRQ
    CPU A3 -> |07  22| <- !ROMSEL
    CPU A2 -> |08  21| <- MODE
    CPU A1 -> |09  20| -> ROM !CE
    CPU A0 -> |10  19| <- M2
    CPU D0 -> |11  18| -> PRG A16
    CPU D1 -> |12  17| -> PRG A15
    CPU D2 -> |13  16| -> PRG A14
       GND    |14  15| -> PRG A13
              '------'
    		  
    Registers:
                                                   mask   power-up
    $4022 [.... .BBB] - bank select (for mode 0)  $71ff   3 (when 4120.0 = 0) / 1 (when 4120.0 = 1)
    $4120 [.... ...S] - bank swapping             $71ff   0
    $4122 [.... ...I] - IRQ off/on                $f1ff   0
    $8000 [.... .BBB] - bank select (for mode 1)  $8000   0
    		  
    MODE - sets the chip to two different modes:
    
    WHen MODE = 0:
    | $6000 | $8000 | $a000 | $c000 | $e000 |
        $2      $1     $0    {$4022}    $a      (when $4120.0 = 0)
        $0      $0     $0    {$4022}    $8      (when $4120.0 = 1)
    Bank at $c000 is switchable and can be changed by writing to $4022
    written value                           : 0 1 2 3 4 5 6 7
    bank will be switched to when $4120.0=0 : 4 3 5 3 6 3 7 3
    bank will be switched to when $4120.0=1 : 1 1 5 1 4 1 5 1
    
    $4122.0: when 0, irqs are disabled (any pending irq is acknowledged), when 1 - irqs are enabled
    When IRQs are enabled, after 4096 rising edges of M2, IRQ is triggered
    (if it is not acknowledged, after another 4096 rising edges of M2,
    it will be automatically acknowledged)
    
    ---------------
    When MODE = 1
           
    | $6000 |     $8000     |     $c000     |
       OPEN      {$8000}            $7       
    
    Value written at $8000 is directly stored in the register (without any scrabling)
    

References: [[1]](http://forums.nesdev.org/viewtopic.php?f=9&t=17399)

Categories: [Pinouts](Category_Pinouts.xhtml)
