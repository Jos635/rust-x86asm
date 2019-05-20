use ::*;
use ::test::*;

#[test]
fn extended_regs() {
    run_test(&Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), Operand::Direct(Reg::R8)), &[0x4C, 0x01, 0xC0], OperandSize::Qword);
    run_test(&Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::AL), Operand::Direct(Reg::SPL)), &[0x40, 0x00, 0xE0], OperandSize::Qword);
    run_test(&Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::AL), Operand::Direct(Reg::BPL)), &[0x40, 0x00, 0xE8], OperandSize::Qword);
    run_test(&Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::AL), Operand::Direct(Reg::SIL)), &[0x40, 0x00, 0xF0], OperandSize::Qword);
    run_test(&Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::AL), Operand::Direct(Reg::DIL)), &[0x40, 0x00, 0xF8], OperandSize::Qword);
}

#[test]
fn push_long() {
    run_test(&Instruction::new1(Mnemonic::PUSH, Operand::Direct(Reg::R8)), &[0x41, 0x50], OperandSize::Qword);
    run_test(&Instruction::new1(Mnemonic::PUSH, Operand::Direct(Reg::R9)), &[0x41, 0x51], OperandSize::Qword);
    run_test(&Instruction::new1(Mnemonic::PUSH, Operand::Direct(Reg::R10)), &[0x41, 0x52], OperandSize::Qword);
    run_test(&Instruction::new1(Mnemonic::PUSH, Operand::Direct(Reg::R11)), &[0x41, 0x53], OperandSize::Qword);
    run_test(&Instruction::new1(Mnemonic::PUSH, Operand::Direct(Reg::R12)), &[0x41, 0x54], OperandSize::Qword);
    run_test(&Instruction::new1(Mnemonic::PUSH, Operand::Direct(Reg::R13)), &[0x41, 0x55], OperandSize::Qword);
    run_test(&Instruction::new1(Mnemonic::PUSH, Operand::Direct(Reg::R14)), &[0x41, 0x56], OperandSize::Qword);
    run_test(&Instruction::new1(Mnemonic::PUSH, Operand::Direct(Reg::R15)), &[0x41, 0x57], OperandSize::Qword);
}