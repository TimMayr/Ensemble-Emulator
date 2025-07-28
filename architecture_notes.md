# 6502

## Architecture

- 8-Bit
- 64kb of addressable memory -> 16-Bit address bus
	- More with cartridge mapper
- Little endian
  - Addresses least significant byte first
- First page (256 Bytes; \$0000-\$00FF) is called Zero Page
	- Relevant for special addressing modes
		- Shorter and quicker instructions
		- indirect access to memory
- Second page (\$0100-\$01FF) is reserved for system stack
- Last 6 bytes of memory (\$FFFA-\$FFFF) are reserved for:
	- Non-maskable interrupt handler (\$FFFA/B)
	- Power on reset location (\$FFFC/D)
	- BRK/interrupt request handler (\$FFFE/F)
- Hardware devices must be mapped to regions in memory

## Registers

### Program Counter

- 16-Bit
- Points to next instruction to be executed
- Modified by:
	- Automatically when instructions are executed
	- When jump is executed
	- Relative branch or subroutine call to another memory address
	- Returning from subroutine or interrupt

### Stack Pointer

- 8-Bit
- Holds the low 8 bits of next free location on stack
- Modified by:
	- Decremented by pushing bytes to stack
	- Incremented by pulling bytes from stack
- CPU does not detect overflow

### Accumulator

- 8-Bit
- Used for most arithmetic and logical operations
- Can be retrieved and stored from either memory or stack

### Index Register X

- 8-Bit
- Most commonly used to hold counters or memory offset
- Can be retrieved and stored from memory
- Can be compared with values from memory
- Can be incremented and decremented
- Can be used to get and set stack pointer

### Index Register Y

- 8-Bit
- Most commonly used to hold counters or memory offset
- Can be retrieved and stored from memory
- Can be compared with values from memory
- Can be incremented and decremented

### Processor Status

- Bitmask
- 8-Bit
- Bits are set as instructions are executed
- Can be tested by instructions
- Store information about result of instructions
- Flags:
	- Carry Flag:
		- Set if last operation caused an overflow from bit 7 or an underflow from bit 0
		- Set during arithmetic, logical, and shift operations
		- Can be cleared with `CLC`
		- Can be set with `SEC`
	- Zero Flag:
		- Set if result of last operation was zero
	- Interrupt disable
		- CPU will not respond to device interrupts while set
		- Can be set with `SEI`
		- Can be cleared with `CLI`
	- Decimal Mode
		- While set the processor obeys the rules of Binary Coded Decimal during addition and subtraction
		- Can be set with `SED`
		- Can be cleared with `CLD`
	- Break Command
		- Set when a `BRK` instruction has been executed and an interrupt has been generated
	- Overflow Flag
		- Is set during arithmetic operations if an overflow happened
		- Determined by looking at the carry between bits 6 and 7, and between bit 7 and the carry flag
	- Negative Flag
		- Set if result of last operation had bit 7 set to one