use core::fmt;
use isa::Instruction;
use Error;

#[derive(Clone, Debug)]
pub enum MiddlewareEvent<'a> {
    Instruction(&'a Instruction<'a>),
}

pub trait Middleware: fmt::Debug {
    fn handle(self: &mut Self, event: MiddlewareEvent) -> Result<(), Error>;
}
