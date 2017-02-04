extern crate rand;

use rand::distributions::{IndependentSample, Range};
use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

mod cpu;

use cpu::CPU;
use cpu::alu;


fn main() {
  let bootfile_name = env::args().nth(1).unwrap();
  let rom_file_name = env::args().nth(2).unwrap(); 

  let boot_data = read_bin(bootfile_name);
  let rom_data = read_bin(rom_file_name);

  let between = Range::new(0,255);
  let mut rng = rand::thread_rng();
  let mut cpu = CPU::new();

  let mut bytecount = 0;

  println!("{:?}", cpu);

  for val in boot_data {
    bytecount += 1;
    cpu.set_a(between.ind_sample(&mut rng));
    cpu.set_e(between.ind_sample(&mut rng));
    cpu.ADDr_e();
    println!("CPU STATUS: {:?}", cpu);
  }

  let addresult = alu::add_8( 25, 10 );

  println!("");

  println!("ALU RESULT: {:?}", addresult);

  println!("");

  println!("BYTECOUNT: {}", bytecount);
}

fn read_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
  let mut file = fs::File::open(&path).unwrap();
  let mut file_buf = Vec::new();
  file.read_to_end(&mut file_buf).unwrap();
  file_buf
}
