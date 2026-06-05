use num_enum::TryFromPrimitive;

use crate::tournment::{Player, State};

#[derive(Debug)]
pub struct Error;

#[derive(Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum OpCode {
    PushNum,
    PushStr,
    Pop,

    AddPlayer,

    MakeGroups,

    Print,
    Eoc, // End Of Command
}

impl From<OpCode> for u8 {
    fn from(op: OpCode) -> Self {
        op as u8
    }
}

#[derive(Debug)]
pub struct StrRef(pub u8);

#[derive(Debug)]
enum Value {
    Number(u8),
    String(StrRef),
}

#[derive(Default, Debug)]
pub struct VM {
    stack: Vec<Value>,
    bytes: Vec<u8>,

    pub strings: Vec<String>, // TODO: remove public access
    state: State,
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
                OpCode::MakeGroups => self.make_groups(),
                OpCode::Eoc => return Ok(()),
            }?
        }
    }

    fn make_groups(&mut self) -> Result<(), Error> {
        self.state.make_groups();
        Ok(())
    }

    fn add_player(&mut self) -> Result<(), Error> {
        let byte = self.read_byte();
        self.state.add_player(StrRef(byte));
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

        self.stack.push(Value::String(StrRef(byte)));

        Ok(())
    }

    // TODO: Extract into a dedicated module to decouple the VM implementation from the interface
    fn print(&self) -> Result<(), Error> {
        let Some(value) = self.stack.last() else {
            return Err(Error);
        };

        let Value::Number(option) = value else {
            return Err(Error);
        };

        match *option {
            1 => self.print_players(),
            2 => self.print_groups(),
            _ => return Err(Error),
        }

        Ok(())
    }

    fn print_players(&self) {
        for Player { name } in self.state.iter_players() {
            let name = &self.strings[name.0 as usize];

            print!(" {name} | ");
        }
        println!();
    }

    fn print_groups(&self) {
        for (idx, group) in self.state.iter_groups().enumerate() {
            print!("Group {}: ", idx + 1);

            for player_id in group.iter_players() {
                let name_id = self.state.get_player(player_id).name.0;
                print!("{}, ", self.strings[name_id as usize]);
            }
            println!();
        }
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.bytes[self.ip];
        self.ip += 1;

        byte
    }
}
