use std::any::Any;
use std::fmt::{Formatter, Result as FmtResult};
use std::mem::size_of;

use crate::chunk::Chunk;
use crate::garbage_collection::collector::Collector as GarbageCollector;
use crate::garbage_collection::reference::Reference;
use crate::garbage_collection::trace::Trace;
use crate::instruction::Instruction;
use crate::value::Value;

pub mod upvalue;
use upvalue::Upvalue;

/// The `Function` type.
#[derive(Clone, Debug, Display)]
#[display(
  fmt = "name: {}, chunk: {}, arity: {}, upvalues: {:#?}",
  name,
  chunk,
  arity,
  upvalues
)]
pub struct Function {
  /// The name of the function.
  pub name: Reference<String>,
  /// A chunk of code.
  pub chunk: Chunk,
  /// The arity of the function.
  pub arity: usize,
  /// Upvalues.
  pub upvalues: Vec<Upvalue>,
}

impl Function {
  /// Constructor.
  pub fn new(name: Reference<String>) -> Self {
    let arity = 0;
    let chunk = Chunk::default();
    let upvalues = Vec::new();
    Self {
      arity,
      chunk,
      name,
      upvalues,
    }
  }
}

impl Trace for Function {
  fn format(&self, f: &mut Formatter, garbage_collector: &GarbageCollector) -> FmtResult {
    let name = garbage_collector.deref(self.name);
    if name.is_empty() {
      write!(f, "<script>")
    } else {
      write!(f, "<fn {}>", name)
    }
  }

  fn get_size(&self) -> usize {
    size_of::<Function>()
      + self.upvalues.capacity() * size_of::<Upvalue>()
      + self.chunk.instructions.instructions.capacity() * size_of::<Instruction>()
      + self.chunk.constants.constants.capacity() * size_of::<Value>()
      + self.chunk.constants.constants.capacity() * size_of::<usize>()
  }

  fn trace(&self, garbage_collector: &mut GarbageCollector) {
    garbage_collector.mark_object(self.name);
    for &constant in &self.chunk.constants.constants {
      garbage_collector.mark_value(constant);
    }
  }

  fn as_any(&self) -> &dyn Any {
    self as _
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self as _
  }
}
