# User:Lidnariq/Unofficial opcode matrix

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3ALidnariq/Unofficial_opcode_matrix) | View [other pages](Special_AllPages.xhtml#User_Lidnariq_Unofficial_opcode_matrix)

I posted this [to the forum](https://forums.nesdev.org/viewtopic.php?p=83966#p83966) forever ago. Here's a slightly different format from [CPU unofficial opcodes](CPU_unofficial_opcodes.xhtml "CPU unofficial opcodes"). 
    
    
           00       04       08       0C       10        14        18        1C
    00 BRK immed|nop zpg|PHP impl |nop abs|BPL rel  |nop zpg,X|CLC impl |nop abs,X  \
    20 JSR abs  |BIT zpg|PLP impl |BIT abs|BMI rel  |nop zpg,X|SEC impl |nop abs,X   \ program flow
    40 RTI impl |nop zpg|PHA impl |JMP abs|BVC rel  |nop zpg,X|CLI impl |nop abs,X   /
    60 RTS impl |nop zpg|PLA impl |JMP ind|BVS rel  |nop zpx,X|SEI impl |nop abs,X  /
    80 nop immed|STY zpg|DEY impl |STY abs|BCC rel  |STY zpg,X|TYA impl |**sya abs,X**  store Y
    A0 LDY immed|LDY zpg|TAY impl |LDY abs|BCS rel  |LDY zpg,X|CLV impl |LDY abs,X  load Y
    C0 CPY immed|CPY zpg|INY impl |CPY abs|BNE rel  |nop zpg,X|CLD impl |nop abs,X  compare Y
    E0 CPX immed|CPX zpg|INX impl |CPX abs|BEQ rel  |nop zpg,X|SED impl |nop abs,X  compare X
    02 kil      |ASL zpg|ASL A    |ASL abs|kil      |ASL zpg,X|nop impl |ASL abs,X  arithmetic shift left
    22 kil      |ROL zpg|ROL A    |ROL abs|kil      |ROL zpg,X|nop impl |ROL abs,X  rotate left through carry
    42 kil      |LSR zpg|LSR A    |LSR abs|kil      |LSR zpg,X|nop impl |LSR abs,X  logical shift right
    62 kil      |ROR zpg|ROR A    |ROR abs|kil      |ROR zpg,X|nop impl |ROR abs,X  rotate right through carry
    82 nop imm  |STX zpg|TXA impl |STX abs|kil      |STX zpg,Y|TXS impl |**sxa abs,Y**  store X
    A2 LDX imm  |LDX zpg|TAX impl |LDX abs|kil      |LDX zpg,Y|TSX impl |LDX abs,Y  load X
    C2 nop imm  |DEC zpg|DEX impl |DEC abs|kil      |DEC zpg,X|nop impl |DEC abs,X  decrement
    E2 nop imm  |INC zpg|NOP impl |INC abs|kil      |INC zpg,X|nop impl |INC abs,X  increment
    01 ORA X,ind|ORA zpg|ORA immed|ORA abs|ORA ind,Y|ORA zpg,X|ORA abs,Y|ORA abs,X  bitwise inclusive OR
    21 AND X,ind|AND zpg|AND immed|AND abs|AND ind,Y|AND zpg,X|AND abs,Y|AND abs,X  bitwise and
    41 EOR X,ind|EOR zpg|EOR immed|EOR abs|EOR ind,Y|EOR zpg,X|EOR abs,Y|EOR abs,X  bitwise exclusive or
    61 ADC X,ind|ADC zpg|ADC immed|ADC abs|ADC ind,Y|ADC zpg,X|ADC abs,Y|ADC abs,X  add with carry
    81 STA X,ind|STA zpg|nop immed|STA abs|STA ind,Y|STA zpg,X|STA abs,Y|STA abs,X  store A
    A1 LDA X,ind|LDA zpg|LDA immed|LDA abs|LDA ind,Y|LDA zpg,X|LDA abs,Y|LDA abs,X  load A
    C1 CMP X,ind|CMP zpg|CMP immed|CMP abs|CMP ind,Y|CMP zpg,X|CMP abs,Y|CMP abs,X  compare
    E1 SBC X,ind|SBC zpg|SBC immed|SBC abs|SBC ind,Y|SBC zpg,X|SBC abs,Y|SBC abs,X  subtract with borrow
    03 slo X,ind|slo zpg|**anc immed** |slo abs|slo ind,Y|slo zpg,X|slo abs,Y|slo abs,X  01+02 = ORA+ASL = 'slo'
    23 rla X,ind|rla zpg|**anc immed** |rla abs|rla ind,Y|rla zpg,X|rla abs,Y|rla abs,X  21+22 = AND+ROL = 'rla'
    43 sre X,ind|sre zpg|**alr immed** |sre abs|sre ind,Y|sre zpg,X|sre abs,Y|sre abs,X  41+42 = EOR+LSR = 'sre'
    63 rra X,ind|rra zpg|**arr immed** |rra abs|rra ind,Y|rra zpg,X|rra abs,Y|rra abs,X  61+62 = ADC+ROR = 'rra'
    83 sax X,ind|sax zpg|**xaa immed** |sax abs|**axa ind,Y** |sax zpg,Y|**tas abs,Y** |**axa abs,Y**  81+82 = STA+STX = 'sax'
    A3 lax X,ind|lax zpg|**atx immed** |lax abs|lax ind,Y|lax zpg,Y|**lar abs,Y** |lax abs,Y  A1+A2 = LDA+LDX = 'lax'
    C3 dcp X,ind|dcp zpg|**axs immed** |dcp abs|dcp ind,Y|dcp zpg,X|dcp abs,Y|dcp abs,X  C1+C2 = CMP+DEC = 'dcp'
    E3 isc X,ind|isc zpg|**sbc immed** |isc abs|isc ind,Y|isc zpg,X|isc abs,Y|isc abs,X  E1+E2 = SBC+INC = 'isc'
    

Unofficial instructions in **bold** break the pattern of their row. Some are described at [Programming with unofficial opcodes](Programming_with_unofficial_opcodes.xhtml "Programming with unofficial opcodes"). 
