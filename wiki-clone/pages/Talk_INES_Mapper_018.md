# Talk:INES Mapper 018

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_018) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_018)

\- My notes bring "Jaleco SS8806" for this mapper. \--[Zepper](User_Zepper.xhtml "User:Zepper") 04:41, 27 December 2010 (UTC) 

\- Counter counts **down** actually. There is hardware tested verilog code for it: 
    
    
       always @ (negedge m2)
       begin    
           if (mapper18_irq_control[0])
           begin
               reg carry;
               {carry, mapper18_irq_value[3:0]} = {1'b0, mapper18_irq_value[3:0]} - 1'b1;
               if (!mapper18_irq_control[3])
                  {carry, mapper18_irq_value[7:4]} = {1'b0, mapper18_irq_value[7:4]} - carry;
               if (!mapper18_irq_control[3] && !mapper18_irq_control[2])
                  {carry, mapper18_irq_value[11:8]} = {1'b0, mapper18_irq_value[11:8]} - carry;
               if (!mapper18_irq_control[3] && !mapper18_irq_control[2] && !mapper18_irq_control[1])
                  {carry, mapper18_irq_value[15:12]} = {1'b0, mapper18_irq_value[15:12]} - carry;
               mapper18_irq_out = mapper18_irq_out | carry;
           end
       end
    

Also fceux source code: 
    
    
       static void M18IRQHook(int a) {
           if (IRQa && IRQCount) {
               IRQCount -= a;
               if (IRQCount <= 0) {
                   X6502_IRQBegin(FCEU_IQEXT);
                   IRQCount = 0;
                   IRQa = 0;
               }
           }
       }
    

[Cluster](https://www.nesdev.org/w/index.php?title=User:Cluster&action=edit&redlink=1 "User:Cluster \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Cluster&action=edit&redlink=1 "User talk:Cluster \(page does not exist\)")) 08:39, 14 September 2020 (MDT) 
