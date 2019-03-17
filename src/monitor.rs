use isa::Instruction;
use super::TrapKind;

const DEFAULT_MAX: u64 = 100;

type GasForIndexFn = fn(usize) -> u64;

pub struct InterpreterMonitor {
    current_gas: u64,
    max_gas: u64,
    gas_for_index: Option<GasForIndexFn>,
}

impl InterpreterMonitor {

    pub fn new(
        start: u64,
        max: u64,
        gas_for_index_fn:
        Option<GasForIndexFn>
    ) -> InterpreterMonitor {
        InterpreterMonitor {
            current_gas: start,
            max_gas: max,
            gas_for_index: gas_for_index_fn
        }
    }

    pub fn default(gas_for_index_fn: Option<GasForIndexFn>) -> InterpreterMonitor {
        InterpreterMonitor::new(0, DEFAULT_MAX, gas_for_index_fn)
    }

    pub fn check_gas(&mut self, instruction: &Instruction) -> Result<(), TrapKind> {
        self.current_gas += self.gas_for_instruction(instruction);
        if self.current_gas >= self.max_gas {
            Err(TrapKind::OutOfGas)
        } else {
            Ok(())
        }
    }

    pub fn gas_for_instruction(&self, instruction: &Instruction) -> u64 {
        match instruction {
            Instruction::Call(index) => {
                if let Some(gas_for_index) = self.gas_for_index {
                    gas_for_index(*index as usize)
                } else {
                    1
                }
            },
            _ => 1
        }
    }
}
