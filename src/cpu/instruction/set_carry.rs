use super::Instruction;
use crate::{
    cpu::{Cpu, Flag},
    memory_device::MemoryDevice,
};

#[doc(alias = "SCF")]
/// Set the [Flag::Carry] flag to `true`.
///
/// ```
/// # use rust_gameboy_library::cpu::{CpuState, Cpu, Register, Flag};
/// # use rust_gameboy_library::cpu::instruction::SetCarry;
/// # use rust_gameboy_library::cpu::instruction::Instruction;
/// # use rust_gameboy_library::debug_memory::DebugMemory;
/// #
/// # let mut cpu = CpuState::new();
/// # let mut memory = DebugMemory::new();
/// #
/// let instruction = SetCarry {};
/// instruction.execute(&mut cpu, &mut memory);
///
/// assert_eq!(cpu.read_flag(Flag::Subtract), false);
/// assert_eq!(cpu.read_flag(Flag::HalfCarry), false);
/// assert_eq!(cpu.read_flag(Flag::Carry), true);
/// ```
pub struct SetCarry {}

impl Instruction for SetCarry {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        cpu.write_flag(Flag::Subtract, false);
        cpu.write_flag(Flag::HalfCarry, false);
        cpu.write_flag(Flag::Carry, true);

        return cpu.load_instruction(memory);
    }
    fn encode(&self) -> Vec<u8> {
        Vec::from([0b00110111])
    }
}

#[cfg(test)]
mod tests {
    use super::SetCarry;
    use crate::cpu::instruction::Instruction;
    use crate::cpu::{Cpu, CpuState, Flag};
    use crate::debug_memory::DebugMemory;

    #[test]
    fn invert_carry_works() {
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new();

        let instruction = SetCarry {};
        instruction.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.read_flag(Flag::Subtract), false);
        assert_eq!(cpu.read_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.read_flag(Flag::Carry), true);
    }
}
