mod vm;
use vm::{OpCode as OP, VM};

macro_rules! bytecode {
    ($($x:expr),* $(,)?) => {
        vec![$($x as u8),*]
    };
}

fn main() {
    let code: Vec<u8> = bytecode![OP::Push, 1, OP::Push, 4, OP::EOC];

    let mut vm = VM::default();
    vm.interpret(code);

    dbg!(vm);
}
