use super::*;

#[test]
fn set_8bit_reg() {
  let mut cpu = CPU::new();

  cpu.set_reg_8("a", 8);
  cpu.set_reg_8("b", 8);
  cpu.set_reg_8("c", 8);
  cpu.set_reg_8("d", 8);
  cpu.set_reg_8("e", 8);
  assert_eq!(cpu.get_reg_8("a"), 8);
  assert_eq!(cpu.get_reg_8("b"), 8);
  assert_eq!(cpu.get_reg_8("c"), 8);
  assert_eq!(cpu.get_reg_8("d"), 8);
  assert_eq!(cpu.get_reg_8("e"), 8);
}
