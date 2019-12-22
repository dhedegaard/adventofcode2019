#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum IntcodeState {
  Halt,
  Running,
  NeedInput,
  Output(i64),
}

#[derive(Debug)]
pub struct Intcode {
  pub insts: Vec<i64>,
  pc: i64,
  pub state: IntcodeState,
  pub input: Vec<i64>,
  relative_base: i64,
}

impl Intcode {
  pub fn new(insts: &[i64], input: &[i64]) -> Intcode {
    Intcode {
      insts: insts.to_vec(),
      pc: 0,
      state: IntcodeState::Running,
      input: input.to_vec(),
      relative_base: 0,
    }
  }

  fn get_registry(&mut self, operation: i64, param: usize, addr: bool) -> i64 {
    let val = self.insts[self.pc as usize + param];
    let param_mode = (operation as u32) / (10_u32.pow(param as u32 + 1)) % 10;
    if addr {
      return match param_mode {
        0 => val,
        2 => val + self.relative_base,
        _ => panic!("Weird parameter in addr mode: {}", param_mode),
      };
    }
    assert!(!(addr && param_mode == 1));
    let pos = match param_mode {
      0 => val as usize,
      1 => return val,
      2 => (val + self.relative_base) as usize,
      _ => panic!("Weird parameter mode: {}", param_mode),
    };
    self.ensure_size(pos);
    self.insts[pos]
  }

  /// Runs the entire program to completion and returns the input.
  pub fn run(&mut self) -> Vec<i64> {
    let mut result = vec![];
    loop {
      match self.execute() {
        IntcodeState::Halt => break,
        IntcodeState::Output(e) => result.push(e),
        _ => {}
      }
    }
    result
  }

  pub fn execute(&mut self) -> IntcodeState {
    loop {
      let operation = self.insts[self.pc as usize];
      let opcode = operation % 100;
      match opcode {
        1 => {
          // Addition
          let a = self.get_registry(operation, 1, false);
          let b = self.get_registry(operation, 2, false);
          let res = self.get_registry(operation, 3, true) as usize;
          self.ensure_size(res);
          self.insts[res] = a + b;
          self.pc += 4;
        }
        2 => {
          // Multiplication
          let a = self.get_registry(operation, 1, false);
          let b = self.get_registry(operation, 2, false);
          let res = self.get_registry(operation, 3, true) as usize;
          self.ensure_size(res);
          self.insts[res] = a * b;
          self.pc += 4;
        }
        3 => {
          // Input
          let res = self.get_registry(operation, 1, true) as usize;
          if self.input.is_empty() {
            self.state = IntcodeState::NeedInput;
            return IntcodeState::NeedInput;
          }
          self.ensure_size(res);
          self.insts[res] = self.input.remove(0);
          self.pc += 2;
        }
        4 => {
          // Output
          let res = self.get_registry(operation, 1, false);
          self.pc += 2;
          self.state = IntcodeState::Output(res);
          return IntcodeState::Output(res);
        }
        5 => {
          // jump-if-true
          let res = self.get_registry(operation, 1, false);
          if res != 0 {
            self.pc = self.get_registry(operation, 2, false);
          } else {
            self.pc += 3;
          }
        }
        6 => {
          // jump-if-false
          let res = self.get_registry(operation, 1, false);
          if res == 0 {
            self.pc = self.get_registry(operation, 2, false);
          } else {
            self.pc += 3;
          }
        }
        7 => {
          // less-than
          let a = self.get_registry(operation, 1, false);
          let b = self.get_registry(operation, 2, false);
          let res = self.get_registry(operation, 3, true) as usize;
          self.ensure_size(res);
          self.insts[res] = if a < b { 1 } else { 0 };
          self.pc += 4;
        }
        8 => {
          // equals
          let a = self.get_registry(operation, 1, false);
          let b = self.get_registry(operation, 2, false);
          let res = self.get_registry(operation, 3, true) as usize;
          self.ensure_size(res);
          self.insts[res] = if a == b { 1 } else { 0 };
          self.pc += 4;
        }
        9 => {
          // adjust relative base
          self.relative_base += self.get_registry(operation, 1, false);
          self.pc += 2;
        }
        // Halt
        99 => {
          self.state = IntcodeState::Halt;
          return IntcodeState::Halt;
        }
        _ => panic!("Unknown opcode: {}", opcode),
      }
    }
  }

  fn ensure_size(&mut self, size: usize) {
    let bounds = if self.relative_base > 0 {
      size + (self.relative_base as usize) + 1
    } else {
      size + 1
    };
    if self.insts.len() < bounds {
      self.insts.resize(bounds + 1, 0);
    }
  }
}
