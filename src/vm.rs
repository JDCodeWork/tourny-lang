use num_enum::TryFromPrimitive;

#[derive(Debug)]
pub struct Error;

#[derive(Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum OpCode {
    PushNum,
    PushStr,
    AddPlayer,
    Pop,
    Print,
    Eoc, // End Of Command
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
struct TmtState {
    players: Vec<u8>,
}

#[derive(Default, Debug)]
pub struct VM {
    stack: Vec<Value>,
    bytes: Vec<u8>,

    pub strings: Vec<String>, // TODO: remove public access
    state: TmtState,
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
                OpCode::AddPlayer => self.add_player(),
                OpCode::Pop => self.pop().map(|_| ()),
                OpCode::Print => self.print(),
                OpCode::Eoc => return Ok(()),
            }?
        }
    }

    fn add_player(&mut self) -> Result<(), Error> {
        let byte = self.read_byte();
        self.state.players.push(byte);
        Ok(())
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
        let byte = self.read_byte(); // Read string index from bytecode

        self.stack.push(Value::String(byte));

        Ok(())
    }

    fn print(&self) -> Result<(), Error> {
        let Some(value) = self.stack.last() else {
            return Err(Error);
        };

        let Value::Number(option) = value else {
            return Err(Error);
        };

        match *option {
            1 => self.print_players(),
            _ => return Err(Error),
        }

        Ok(())
    }

    fn print_players(&self) {
        for player in self.state.players.iter() {
            let name = &self.strings[*player as usize];

            print!(" {name} | ");
        }
        println!();
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.bytes[self.ip];
        self.ip += 1;

        byte
    }
}
