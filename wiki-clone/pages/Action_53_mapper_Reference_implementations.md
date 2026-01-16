# Action 53 mapper/Reference implementations

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Action_53_mapper/Reference_implementations) | View [other pages](Special_AllPages.xhtml#Action_53_mapper_Reference_implementations)

These reference implementations of the PRG ROM bank formula in the [Action 53 mapper](Action_53_mapper.xhtml "Action 53 mapper") ([mapper 28](Action_53_mapper.xhtml "INES Mapper 028")) may be used to verify emulator or hardware implementations of this mapper. 

## Python
    
    
    #!/usr/bin/env python3
    from __future__ import print_function
    
    bank_size_masks = [(2<<i)-1 for i in range(4)]  # 0x01, 0x03, 0x07, 0x0F
    
    def calc_prg_bank(address, bank_mode, outer_bank, current_bank):
        """Calculate the 16K bank on A20-A14 for a given address.
    
    address -- current address being accessed by the CPU,
        in range 0x8000-0xFFFF
    bank_mode -- last value written to $80
    outer_bank -- last value written to $81
    current_bank -- last value written to $01
    
    """
        cpu_a14 = (address >> 14) & 0x01
        outer_bank = outer_bank << 1
        bank_mode >>= 2  # discard mirroring bits
        if (bank_mode ^ cpu_a14) & 0x03 == 0x02:  # in UNROM fixed bank?
            bank_mode = 0  # if so, treat as NROM
        if (bank_mode & 0x02) == 0:  # in 32K bank mode?
            current_bank = (current_bank << 1) | cpu_a14
        bank_size_mask = bank_size_masks[(bank_mode >> 2) & 3]
        return (current_bank & bank_size_mask) | (outer_bank & ~bank_size_mask)
    
    def test_with_bank_mode_size(bank_mode, outer_bank):
        print("mode $%02x, outer bank $%02x" % (bank_mode, outer_bank))
        out80 = [calc_prg_bank(0x8000, bank_mode, outer_bank, current_bank)
             for current_bank in range(16)]
        print("$8000 banks:", " ".join("%02x" % i for i in out80))
        outC0 = [calc_prg_bank(0xC000, bank_mode, outer_bank, current_bank)
                 for current_bank in range(16)]
        print("$C000 banks:", " ".join("%02x" % i for i in outC0))
    
    for outer_bank in (0x00, 0x3C, 0x3F):
        test_with_bank_mode_size(0x28, outer_bank)
    for outer_bank in (0x00, 0x03, 0x3F):
        test_with_bank_mode_size(0x2C, outer_bank)
    for bank_mode in (0x00, 0x08, 0x0C, 0x10, 0x18, 0x1C,
                      0x20, 0x28, 0x2C, 0x30, 0x38, 0x3C):
        test_with_bank_mode_size(bank_mode, 0x2A)
    

## 6502 assembly

This routine appears in the test ROMs "test28" and "Holy Mapperel". 
    
    
    ;;
    ; Determines what 16 KiB PRG ROM bank ought to be mapped into a given
    ; CPU address with a given set of $80, $81, $00 values.
    ; For use in a test ROM that verifies mapper 28.
    ; @param A $80 value
    ; @param X $01 value
    ; @param Y $81 value
    ; @param C CPU A14 value: clear for $8000-$BFFF, set for $C000-$FFFF
    ; @return PRG bank number in A
    .proc calc_prg_bank
    bank_mode = 0
    outer_bank = 1
    current_bank = 2
      stx current_bank
      sty outer_bank
      rol outer_bank
      lsr a  ; discard mirroring bits
      lsr a
      sta bank_mode
    
      ; If the mode is UxROM (10 = mapper 180, 11 = mapper 2), and bit 0
      ; of the mode matches CPU A14, then the read is within the fixed
      ; bank.  For such reads, the mapper acts in 32K (NROM) mode.
      and #$02
      beq not_unrom
      lda outer_bank
      eor bank_mode
      ; If bit 0 of the eor result is false, there is a match, so
      ; fall through to the not-UNROM code.
      and #$01
      bne have_current_bank
      sta bank_mode
    
    not_unrom:
      ; In 32K switched modes (NROM, CNROM, BNROM, AOROM),
      ; shift CPU A14 into the current bank
      lda outer_bank
      lsr a
      rol current_bank
      
    have_current_bank:
      lda bank_mode
      lsr a
      lsr a
      and #$03
      tax
      lda current_bank
      eor outer_bank
      and bank_size_masks,x
      eor outer_bank
      rts
    .endproc
    
    .segment "RODATA"
    bank_size_masks: .byt $01, $03, $07, $0F
    
