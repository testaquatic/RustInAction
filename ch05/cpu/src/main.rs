use cpu::CPU;

fn main() {
    let mut cpu = CPU::default();

    cpu.registers_mut()[0] = 5;
    cpu.registers_mut()[1] = 10;

    let memory = cpu.memory_mut();
    let data1 = [0x21, 0x00, 0x21, 0x00, 0x00, 0x00];
    memory[0..data1.len()].copy_from_slice(&data1);
    let data2 = [0x80, 0x14, 0x80, 0x14, 0x00, 0xEE];
    memory[0x100..0x100 + data2.len()].copy_from_slice(&data2);

    cpu.run();

    assert_eq!(cpu.registers_ref()[0], 45);

    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers_ref()[0]);
}
