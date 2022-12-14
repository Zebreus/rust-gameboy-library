use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::memory_device::MemoryDevice;

/// Instructions can be executed to modify cpu state and memory
pub mod instruction;

use self::instruction::decode;
use self::instruction::InstructionEnum;

/// The CpuState stores the internal state of the gameboy processor.
///
/// This is basically just a data container, the actual CPU functionality is handled by [Instruction](instruction::Instruction).
pub struct CpuState {
    program_counter: u16,
    stack_pointer: u16,
    registers: [u8; 8],
}

impl CpuState {
    /// Initialize a new CPU state.
    ///
    /// The program counter is set to the start of the ROM.
    /// The stack pointer is set to 0xFFFE.
    /// The registers are set to 0.
    ///
    /// ```
    /// use rust_gameboy_library::cpu::CpuState;
    ///
    /// let cpuState = CpuState::new();
    /// ```
    pub fn new() -> Self {
        Self {
            program_counter: 0,
            stack_pointer: 0xFFFE,
            registers: [0, 0, 0, 0, 0, 0, 0, 0],
        }
    }
    /// Load the next opcode
    ///
    /// Also increments the program counter
    pub fn load_opcode(&mut self, memory: &dyn MemoryDevice) -> u8 {
        let opcode = memory.read(self.read_program_counter());
        return opcode;
    }

    /// Load the next [Instruction]
    ///
    /// Also increments the program counter
    pub fn load_instruction(&mut self, memory: &dyn MemoryDevice) -> InstructionEnum {
        let opcode = self.load_opcode(memory);
        return decode(opcode);
    }
}

/// Trait for something that can be used as a gameboy cpu state.
pub trait Cpu {
    /// Get the address of the current instruction. Increment the program counter
    fn read_program_counter(&mut self) -> u16;
    /// Get the current stack pointer
    fn read_stack_pointer(&self) -> u16;
    /// Set the current stack pointer
    fn write_stack_pointer(&mut self, value: u16);
    /// Get the content of a register.
    fn read_register(&self, register: Register) -> u8;
    /// Write a value to a register
    ///
    /// ```
    /// # use rust_gameboy_library::cpu::{CpuState, Cpu, Register};
    /// # let mut cpu = CpuState::new();
    /// #
    /// cpu.write_register(Register::A, 5);
    /// ```
    fn write_register(&mut self, register: Register, value: u8) -> ();
    /// Read the value from two registers at once.
    ///
    /// The documentation for [DoubleRegister] contains more information on the values.
    fn read_double_register(&self, register: DoubleRegister) -> u16;
    /// Read the value from two registers at once.
    ///
    /// The documentation for [DoubleRegister] contains more information on the values.
    fn write_double_register(&mut self, register: DoubleRegister, value: u16) -> ();
}

impl Cpu for CpuState {
    fn read_program_counter(&mut self) -> u16 {
        let result = self.program_counter;
        self.program_counter += 1;
        return result;
    }
    fn read_stack_pointer(&self) -> u16 {
        let result = self.stack_pointer;
        return result;
    }
    fn write_stack_pointer(&mut self, value: u16) {
        self.stack_pointer = value;
    }

    fn read_register(&self, register: Register) -> u8 {
        let index = register as usize;
        return self.registers[index];
    }
    fn write_register(&mut self, register: Register, value: u8) -> () {
        let index = register as usize;

        if let Register::F = register {
            // You cannot write bit 0-3 on the flags register
            self.registers[index] = value & 0b11110000;
        }
        self.registers[index] = value;
    }
    fn read_double_register(&self, register: DoubleRegister) -> u16 {
        let registers = register.id();
        let low: u16 = self.read_register(registers.lsb).into();
        let high: u16 = self.read_register(registers.msb).into();
        let value: u16 = high << 8 | low;
        return value;
    }
    fn write_double_register(&mut self, register: DoubleRegister, value: u16) -> () {
        let registers = register.id();
        let [high, low] = u16::to_be_bytes(value);
        self.write_register(registers.msb, high);
        self.write_register(registers.lsb, low);
    }
}

/// Register of the gameboy cpu
///
/// The gameboy processor has 8 general purpose 8bit registers.
/// The registers are named A, B, C, D, E, F, H, and L.
/// The registers are accessed by the `read_register`
///
/// [Register::A] also acts as the accumulator.
#[derive(TryFromPrimitive, Debug, IntoPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum Register {
    /// A general purpose register. Acts as the accumulator for some instructions.
    A = 0,
    /// A general purpose register.
    B = 1,
    /// A general purpose register.
    C = 2,
    /// A general purpose register.
    D = 3,
    /// A general purpose register.
    E = 4,
    /// The flags register.
    ///
    /// Contains flags set and used by various instructions.
    ///
    /// |  Bit 0   |   Bit 1  |   Bit 2  |   Bit 3  | Bit 4 |   Bit 5    |  Bit 6   | Bit 7 |
    /// |----------|----------|----------|----------|-------|------------|----------|-------|
    /// | readonly | readonly | readonly | readonly | carry | half-carry | negative | zero  |
    F = 5,
    /// A general purpose register. The high part of the address DoubleRegister [HL](DoubleRegister::HL)
    H = 6,
    /// A general purpose register. The low part of the address DoubleRegister [HL](DoubleRegister::HL)
    L = 7,
}

impl Register {
    fn id(&self) -> u8 {
        *self as u8
    }
}

struct RegisterCombination {
    lsb: Register,
    msb: Register,
}

/// The gameboy has many instructions that combine two registers as a single 16bit value.
///
/// This enum represents the two registers that are combined.
#[derive(TryFromPrimitive, Debug, IntoPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum DoubleRegister {
    /// A double register consisting of [Register::A] and [Register::F].
    ///
    /// Does not allow writing the bits 0-3 of F. See [Register::F] for details.
    AF = 0,
    /// A general purpose double register consisting of [Register::B] and [Register::C].
    BC = 1,
    /// A general purpose double register consisting of [Register::D] and [Register::E].
    DE = 2,
    /// A double register consisting of [Register::H] and [Register::L].
    ///
    /// Contains memory addresses for some operations.
    HL = 3,
}

impl DoubleRegister {
    fn id(&self) -> RegisterCombination {
        match self {
            DoubleRegister::AF => RegisterCombination {
                msb: Register::A,
                lsb: Register::F,
            },
            DoubleRegister::BC => RegisterCombination {
                msb: Register::B,
                lsb: Register::C,
            },
            DoubleRegister::DE => RegisterCombination {
                msb: Register::D,
                lsb: Register::E,
            },
            DoubleRegister::HL => RegisterCombination {
                msb: Register::H,
                lsb: Register::L,
            },
        }
    }

    fn numerical_id(&self) -> u8 {
        *self as u8
    }
}

#[cfg(test)]
mod tests {
    use super::instruction::{InstructionEnum, LoadFromRegisterToRegister};
    use super::Cpu;
    use super::{CpuState, DoubleRegister};
    use crate::cpu::Register;
    use crate::debug_memory::DebugMemory;

    #[test]
    fn read_double_register() {
        let mut cpu = CpuState::new();
        cpu.write_register(Register::B, 1);
        cpu.write_register(Register::C, 3);
        let double_value = cpu.read_double_register(DoubleRegister::BC);
        assert_eq!(double_value, 259)
    }

    #[test]
    fn write_double_register() {
        let mut cpu = CpuState::new();
        cpu.write_double_register(DoubleRegister::BC, 259);
        assert_eq!(cpu.read_register(Register::B), 1);
        assert_eq!(cpu.read_register(Register::C), 3);
    }

    #[test]
    fn write_read_double_register() {
        let mut cpu = CpuState::new();
        cpu.write_double_register(DoubleRegister::BC, 9874);
        assert_eq!(cpu.read_double_register(DoubleRegister::BC), 9874);
    }

    #[test]
    fn cpu_read_program_counter_works() {
        let mut cpu = CpuState::new();

        let memory = DebugMemory::new_with_init(&[0b01000010u8, 8]);
        let opcode = cpu.load_opcode(&memory);

        assert_eq!(opcode, 0b01000010u8);

        let opcode = cpu.load_opcode(&memory);
        assert_eq!(opcode, 8);
    }

    #[test]
    fn cpu_can_fetch_and_decode_instructions() {
        let mut cpu = CpuState::new();
        cpu.write_register(Register::A, 100);

        let memory = DebugMemory::new_with_init(&[0b01000010u8]);
        let instruction = cpu.load_instruction(&memory);
        assert!(matches!(
            instruction,
            InstructionEnum::LoadFromRegisterToRegister(LoadFromRegisterToRegister {
                source: Register::A,
                destination: Register::C
            })
        ))
    }
}
