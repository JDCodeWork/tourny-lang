mod macros;
mod vm;

use vm::{OpCode as Op, VM};

fn main() {
    let strs = vec!["Juan".to_string(), "Carlos".to_string()];
    let code: Vec<u8> = bytecode![
        Op::AddPlayer,
        0,
        Op::AddPlayer,
        1,
        Op::PushNum,
        1,
        Op::Print,
        Op::Eoc
    ];

    let mut vm = VM::default();
    vm.strings = strs;

    vm.interpret(code);
}
