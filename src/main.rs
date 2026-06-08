use tmt_lang::vm::{OpCode as Op, VM};

fn main() {
    let strs = vec![
        "Juan".to_string(),
        "Carlos".to_string(),
        "Juliana".to_string(),
        "Camilo".to_string(),
        "Rodriguez".to_string(),
        "Jaime".to_string(),
    ];
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
        Op::Show as u8,
        2,
        Op::Eoc as u8,
    ];

    let mut vm = VM::default();
    vm.strings = strs;

    vm.interpret(code);
}
