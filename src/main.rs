use tmt_lang::vm::{OpCode as Op, VM};

fn main() {
    let strs = vec!["Juan", "Carlos", "Juliana", "Camilo", "Rodriguez", "Jaime"];

    let code: Vec<u8> = vec![
        Op::AddPlayer as u8,
        0,
        Op::AddPlayer as u8,
        1,
        Op::AddPlayer as u8,
        2,
        Op::AddPlayer as u8,
        3,
        Op::AddPlayer as u8,
        4,
        Op::AddPlayer as u8,
        5,
        Op::MakeGroups as u8,
        Op::MakeMatches as u8,
        Op::Show as u8,
        3,
        Op::Eoc as u8,
    ];

    let mut vm = VM::default();

    vm.interpret(code, strs);
}
