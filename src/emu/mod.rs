
use std::fs;
use std::io::Read;
use std::path::Path;

mod cpu;
mod mmu;

use self::cpu::CPU;
use self::mmu::MMU;


pub fn run(rom_file_name: String) {
  let rom_data = read_bin(rom_file_name);

  let mut cpu = CPU::new();
  let mut mmu = MMU::new();

  for i in 0..(mmu.get_bios_size() - 1) {
    println!("[BOOT] ${:02X} : {:02X}", i, mmu.get_bios_byte(i));
  }
}

fn read_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
  let mut file = fs::File::open(&path).unwrap();
  let mut file_buf = Vec::new();
  file.read_to_end(&mut file_buf).unwrap();
  file_buf
}
