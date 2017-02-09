
pub fn add_8(a: u8, b :u8) -> (bool, bool, u8) {
  let mut carrybool: bool = false;
  let mut zerobool: bool = false;
  let mut sum: u16 = 0;

  sum = (a as u16) + (b as u16);
  let mut overflow = sum;

  overflow >>= 8;    

  if overflow > 0 {
    carrybool = true;
  }

  if sum as u8 == 0 {
    zerobool = true;
  }

  return (carrybool, zerobool, sum as u8);
}

pub fn add_16(a: u16, b :u16) -> (bool, bool, u16) {
  let mut carrybool: bool = false;
  let mut zerobool: bool = false;
  let mut sum: u32 = 0;

  sum = (a as u32) + (b as u32);
  let mut overflow = sum;

  overflow >>= 16;    

  if overflow > 0 {
    carrybool = true;
  }

  if sum as u16 == 0 {
    zerobool = true;
  }

  return (carrybool, zerobool, sum as u16);
}
