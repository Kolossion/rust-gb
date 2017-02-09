use std::env;

mod emu;

use emu::*;

fn main() {
  let rom_file_name = env::args().nth(1).unwrap(); 

  emu::run(rom_file_name);
}
