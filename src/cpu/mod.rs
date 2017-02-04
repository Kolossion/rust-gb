
pub mod alu;

#[derive(Debug)]
struct CPUClock {
  m: u8,
  t: u8,
}

#[derive(Debug)]
struct CPURegs {
  a: u8,
  b: u8,
  c: u8,
  d: u8,
  e: u8,
  h: u8,
  l: u8,
  f: u8,
  pc: u16,
  sp: u16,
  m: u8,
  t: u8,
}

#[derive(Debug)]
pub struct CPU {
  clock: CPUClock,
  reg: CPURegs,
}

impl CPU {

  pub fn run_op(&mut self, opcode: u16) {

  }

  pub fn set_a(&mut self, val: u8) {
    self.reg.a = val;
  }

  pub fn set_e(&mut self, val: u8) {
    self.reg.e = val;
  }

  pub fn ADDr_e(&mut self) {
    let add_res = alu::add_8(self.reg.a, self.reg.e);
    let (overflow, zero, sum) = add_res;
    println!("ADDR: {:?}", add_res);
    self.reg.a = sum;
    self.reg.f |= if overflow { 0x80 } else { 0x00 };
    self.reg.f |= if zero { 0x10 } else { 0x00 };
    self.reg.m = 1;
    self.reg.t = 4;
  }

  pub fn noop(&mut self) {
    self.reg.m = 1;
    self.reg.t = 4;
  }

  pub fn new() -> CPU {

    return CPU { 
      clock: CPUClock {
        m: 0,
        t: 0,
      } ,
      reg: CPURegs {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        h: 0,
        l: 0,
        f: 0,
        pc: 0,
        sp: 0,
        m: 0,
        t: 0,
      },
    };

  }

}
