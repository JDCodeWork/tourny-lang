use num_enum::TryFromPrimitive;

#[derive(Debug)]
pub struct Error;

#[derive(Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum OpCode {
    PushNum,
    PushStr,
    Pop,
    Print,
    Eoc, // End Of Command
}

#[derive(Debug)]
pub enum MarshalError {
    InvalidBytecode,
}

impl From<OpCode> for u8 {
    fn from(op: OpCode) -> Self {
        op as u8
    }
}

#[derive(Debug)]
enum Value {
    Number(u8),
    String(u8),
}

#[derive(Default, Debug)]
pub struct VM {
    stack: Vec<Value>,
    bytes: Vec<u8>,

    pub strings: Vec<String>, // TODO: remove public access
    ip: usize,
}

impl VM {
    pub fn interpret(&mut self, mut code: Vec<u8>) {
        self.bytes.append(&mut code);

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
                OpCode::PushNum => self.push_num(),
                OpCode::PushStr => self.push_str(),
                OpCode::Pop => self.pop().map(|_| ()),
                OpCode::Print => self.print(),
                OpCode::Eoc => return Ok(()),
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

    fn push_num(&mut self) -> Result<(), Error> {
        let byte = self.read_byte();

        self.stack.push(Value::Number(byte));

        Ok(())
    }

    fn push_str(&mut self) -> Result<(), Error> {
        let byte = self.read_byte();

        self.stack.push(Value::String(byte));

        Ok(())
    }

    fn print(&self) -> Result<(), Error> {
        let Some(value) = self.stack.last() else {
            println!();
            return Err(Error);
        };

        match *value {
            Value::Number(num) => println!("{num}"),
            Value::String(idx) => println!("{}", self.strings[idx as usize]),
        };

        Ok(())
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.bytes[self.ip];
        self.ip += 1;

        byte
    }
}
