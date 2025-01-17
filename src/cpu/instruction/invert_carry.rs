use super::Instruction;
use crate::{
    cpu::{Cpu, Flag},
    memory::MemoryDevice,
};

/// Invert the current value of the [Flag::Carry] flag.
///
/// ```
/// # use rust_gameboy_library::cpu::{CpuState, Cpu, Register, Flag};
/// # use rust_gameboy_library::cpu::instruction::InvertCarry;
/// # use rust_gameboy_library::cpu::instruction::Instruction;
/// # use rust_gameboy_library::memory::Memory;
/// #
/// # let mut cpu = CpuState::new();
/// # let mut memory = Memory::new();
/// #
/// cpu.write_flag(Flag::Carry, false);
///
/// let instruction = InvertCarry {};
/// instruction.execute(&mut cpu, &mut memory);
///
/// assert_eq!(cpu.read_flag(Flag::Subtract), false);
/// assert_eq!(cpu.read_flag(Flag::HalfCarry), false);
/// assert_eq!(cpu.read_flag(Flag::Carry), true);
/// ```
///
/// | [Zero](Flag::Zero)  | [Subtract](Flag::Subtract) | [HalfCarry](Flag::HalfCarry) | [Carry](Flag::Carry)       |
/// |---------------------|----------------------------|------------------------------|----------------------------|
/// | unchanged           | false                      | false                        | true if carry was set      |
#[doc(alias = "CCF")]
#[derive(Debug)]
pub struct InvertCarry {}

impl Instruction for InvertCarry {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        cpu.write_flag(Flag::Subtract, false);
        cpu.write_flag(Flag::HalfCarry, false);
        cpu.write_flag(Flag::Carry, !cpu.read_flag(Flag::Carry));

        return cpu.load_instruction(memory);
    }
    fn encode(&self) -> Vec<u8> {
        Vec::from([0b00111111])
    }
}

#[cfg(test)]
mod tests {
    use super::InvertCarry;
    use crate::cpu::instruction::Instruction;
    use crate::cpu::{Cpu, CpuState, Flag};
    use crate::memory::MemoryController;

    #[test]
    fn invert_carry_works() {
        let mut cpu = CpuState::new();
        let mut memory = MemoryController::new_for_tests();

        cpu.write_flag(Flag::Carry, false);
        let instruction = InvertCarry {};
        instruction.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.read_flag(Flag::Subtract), false);
        assert_eq!(cpu.read_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.read_flag(Flag::Carry), true);
    }
}
