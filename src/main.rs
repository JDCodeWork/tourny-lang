mod macros;
mod tournment;
mod vm;

use vm::{OpCode as Op, VM};

fn main() {
    let strs = vec![
        "Juan".to_string(),
        "Carlos".to_string(),
        "Juliana".to_string(),
        "Camilo".to_string(),
        "Rodriguez".to_string(),
        "Jaime".to_string(),
    ];
    let code: Vec<u8> = bytecode![
        Op::AddPlayer,
        0,
        Op::AddPlayer,
        1,
        Op::AddPlayer,
        2,
        Op::AddPlayer,
        3,
        Op::AddPlayer,
        4,
        Op::AddPlayer,
        5,
        Op::MakeGroups,
        Op::PushNum,
        2,
        Op::Print,
        Op::Eoc
    ];

    let mut vm = VM::default();
    vm.strings = strs;

    vm.interpret(code);
}
