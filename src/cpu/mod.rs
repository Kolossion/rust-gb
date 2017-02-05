
#[cfg(test)]
mod test;
pub mod alu;

#[derive(Debug)]
struct CPUClock {
  m: u8,
  t: u8,
}

#[derive(Debug)]
struct CPURegs {
  a  : u8,
  b  : u8,
  c  : u8,
  d  : u8,
  e  : u8,
  h  : u8,
  l  : u8,
  f  : u8,
  pc : u16,
  sp : u16,
  m  : u8,
  t  : u8,
}

#[derive(Debug)]
pub struct CPU {
  clock: CPUClock,
  reg: CPURegs,
}

impl CPU {

  // Given an opcode, run the appropriate things.

  pub fn run_op(&mut self, opcode: u16) {
    println!("UNIMPLEMENTED");

  }

  pub fn get_reg_8(&mut self, reg: &str) -> u8 {
    match reg.as_ref() {
      "a" => return self.reg.a,
      "b" => return self.reg.b,
      "c" => return self.reg.c,
      "d" => return self.reg.d,
      "e" => return self.reg.e,
      "h" => return self.reg.h,
      "l" => return self.reg.l,
      "f" => return self.reg.f,
      "m" => return self.reg.m,
      "t" => return self.reg.t,
      _   => panic!("GB-Z80: Cannot get value of unknown register"),
    }
  }

  pub fn set_reg_8(&mut self, reg: &str, val: u8) {
    // println!("VAL: {:08b}", val);
    match reg.as_ref() {
      "a" => self.reg.a = val as u8,
      "b" => self.reg.b = val as u8,
      "c" => self.reg.c = val as u8,
      "d" => self.reg.d = val as u8,
      "e" => self.reg.e = val as u8,
      "h" => self.reg.e = val as u8,
      "l" => self.reg.e = val as u8,
      "f" => self.reg.e = val as u8,
      "m" => self.reg.e = val as u8,
      "t" => self.reg.e = val as u8,
      _   => panic!("[GB-Z80]: Cannot set value of unknown register"),
    }
    
  }

  pub fn add_e(&mut self) {
    let add_res = alu::add_8(self.reg.a, self.reg.e);
    let (overflow, zero, sum) = add_res;
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

  // Constructor //

  pub fn new() -> CPU {

    return CPU { 
      clock: CPUClock {
        m : 0,
        t : 0,
      } ,
      reg: CPURegs {
        a  : 0,
        b  : 0,
        c  : 0,
        d  : 0,
        e  : 0,
        h  : 0,
        l  : 0,
        f  : 0,
        pc : 0,
        sp : 0,
        m  : 0,
        t  : 0,
      },
    };

  }

}
