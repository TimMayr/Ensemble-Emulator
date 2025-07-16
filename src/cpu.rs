const MEMORY_SIZE: u16 = 0xFFFF;
const STACK_START: u8 = 0xFF;

#[derive(Debug, Eq, PartialEq)]
pub enum AddressingMode {
    Implied,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
}

#[derive(Debug, Copy, Clone)]
pub struct Cpu {
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub accumulator: u8,
    pub index_register_x: u8,
    pub index_register_y: u8,
    pub processor_status: u8,
    memory: [u8; MEMORY_SIZE as usize],
}

impl Default for Cpu {
    fn default() -> Self {
        let memory: [u8; MEMORY_SIZE as usize] = [0; MEMORY_SIZE as usize];
        Self {
            program_counter: 0,
            processor_status: 0,
            accumulator: 0,
            index_register_x: 0,
            index_register_y: 0,
            memory,
            stack_pointer: STACK_START,
        }
    }
}

impl Cpu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub fn mem_read_u16(&self, addr: u16) -> u16 {
        let least_significant_bits = self.mem_read(addr) as u16;
        let highest_significant_bits = self.mem_read(addr + 1) as u16;

        (highest_significant_bits << 8) | (least_significant_bits)
    }

    pub fn mem_write_u16(&mut self, addr: u16, data: u16) {
        let least_significant_bits = (data & 0xFF) as u8;
        let highest_significant_bits = (data >> 8) as u8;
        self.mem_write(addr, least_significant_bits);
        self.mem_write(addr + 1, highest_significant_bits)
    }

    fn set_zero_flag(&mut self) {
        self.processor_status |= 0b0000_0010;
    }

    fn clear_zero_flag(&mut self) {
        self.processor_status &= 0b1111_1101;
    }

    fn update_zero_flag(&mut self, result: u8) {
        if result == 0 {
            self.set_zero_flag()
        } else {
            self.clear_zero_flag()
        }
    }

    fn update_negative_flag(&mut self, result: u8) {
        if result & 0b1000_0000 != 0 {
            self.processor_status |= 0b1000_0000;
        } else {
            self.processor_status &= 0b0111_1111;
        }
    }

    fn update_negative_and_zero_flags(&mut self, result: u8) {
        self.update_negative_flag(result);
        self.update_zero_flag(result);
    }

    fn set_carry_flag(&mut self) {
        self.processor_status |= 0b0000_0001;
    }

    fn clear_carry_flag(&mut self) {
        self.processor_status &= 0b1111_1110;
    }

    fn set_overflow_flag(&mut self) {
        self.processor_status |= 0b0100_0000;
    }

    fn clear_overflow_flag(&mut self) {
        self.processor_status &= 0b1011_1111;
    }

    fn update_carry_and_overflow_flags(&mut self) -> ! {
        unimplemented!()
    }
}
