#[derive(Debug)]
pub struct Error;

#[derive(Debug)]
pub enum OpCode {
    Push,
    Pop,
    EOC, // End Of Command
    _COUNT,
}

#[derive(Debug)]
pub enum MarshalError {
    InvalidBytecode,
}

impl TryFrom<u8> for OpCode {
    type Error = MarshalError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > (OpCode::_COUNT as u8) - 1 {
            return Err(MarshalError::InvalidBytecode);
        }

        let opcode = match value {
            x if x == (OpCode::Push as u8) => OpCode::Push,
            x if x == (OpCode::Pop as u8) => OpCode::Pop,
            x if x == (OpCode::EOC as u8) => OpCode::EOC,
            _ => return Err(MarshalError::InvalidBytecode),
        };

        Ok(opcode)
    }
}

impl From<OpCode> for u8 {
    fn from(op: OpCode) -> Self {
        op as u8
    }
}

#[derive(Debug)]
enum Value {
    Number(u8),
}

#[derive(Default, Debug)]
pub struct VM {
    stack: Vec<Value>,
    code: Vec<u8>,
    ip: usize,
}

impl VM {
    pub fn interpret(&mut self, mut code: Vec<u8>) {
        self.code.append(&mut code);

        if self.run().is_err() {
            eprintln!("Hubo un Error.");
        }
    }

    fn run(&mut self) -> Result<(), Error> {
        loop {
            let instr = self.read_byte();

            let Ok(op) = OpCode::try_from(instr) else {
                return Err(Error);
            };

            match op {
                OpCode::Push => self.push(),
                OpCode::Pop => self.pop().map(|_| ()),
                OpCode::EOC => return Ok(()),
                OpCode::_COUNT => return Err(Error),
            }?
        }
    }

    fn pop(&mut self) -> Result<Value, Error> {
        if let Some(value) = self.stack.pop() {
            Ok(value)
        } else {
            Err(Error)
        }
    }

    fn push(&mut self) -> Result<(), Error> {
        let byte = self.read_byte();

        self.stack.push(Value::Number(byte));

        Ok(())
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.code[self.ip];
        self.ip += 1;

        byte
    }
}
