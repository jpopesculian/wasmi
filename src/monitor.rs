use super::TrapKind;
use isa::Instruction;

const DEFAULT_MAX: u64 = 100;

type GasForIndexFn = fn(usize) -> Option<u64>;

#[derive(Debug, Clone)]
pub struct InterpreterMonitor {
    current_gas: u64,
    default_instruction_gas: u64,
    max_gas: u64,
    gas_for_index: Option<GasForIndexFn>,
}

impl InterpreterMonitor {
    pub fn new(
        default_instruction_gas: u64,
        max_gas: u64,
        gas_for_index_fn: Option<GasForIndexFn>,
    ) -> InterpreterMonitor {
        InterpreterMonitor {
            default_instruction_gas,
            max_gas,
            gas_for_index: gas_for_index_fn,
            ..InterpreterMonitor::default()
        }
    }

    pub fn default() -> InterpreterMonitor {
        InterpreterMonitor {
            current_gas: 0,
            default_instruction_gas: 1,
            max_gas: DEFAULT_MAX,
            gas_for_index: None,
        }
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
                    if let Some(gas) = gas_for_index(*index as usize) {
                        gas
                    } else {
                        self.default_instruction_gas
                    }
                } else {
                    self.default_instruction_gas
                }
            }
            _ => self.default_instruction_gas,
        }
    }
}

pub trait MonitoredExternals {
    /// Get gas price of invoking a function at a specific index.
    fn gas_for_index(_index: usize) -> Option<u64> {
        Some(1)
    }
}
