use crate::cpu::CPU;

#[test]
fn test_0xa9_lda_immediate_load_data() {
    let mut cpu = CPU::new();
    cpu.load_run(vec![0xa9, 0x05, 0x00]);
    assert_eq!(cpu.ra, 5);
    assert!(cpu.status & 0b0000_0010 == 0);
    assert!(cpu.status & 0b1000_0000 == 0);
}

#[test]
fn test_0xa9_lda_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_run(vec![0xa9, 0x00, 0x00]);
    assert!(cpu.status & 0b0000_0010 == 0b10);
}

#[test]
fn test_0xaa_tax_move_a_to_x() {
    let mut cpu = CPU::new();
    cpu.load_run(vec![0xa9, 0x0A,0xaa, 0x00]);

    assert_eq!(cpu.rx, 10)
}

#[test]
fn test_5_ops_working_together() {
    let mut cpu = CPU::new();
    cpu.load_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

    assert_eq!(cpu.rx, 0xc1)
}

#[test]
fn test_inx_overflow() {
    let mut cpu = CPU::new();
    cpu.load_run(vec![0xa9, 0xff, 0xaa,0xe8, 0xe8, 0x00]);

    assert_eq!(cpu.rx, 1)
}

#[test]
fn test_lda_from_memory() {
    let mut cpu = CPU::new();
    cpu.write(0x10, 0x55);

    cpu.load_run(vec![0xa5, 0x10, 0x00]);

    assert_eq!(cpu.ra, 0x55);
}