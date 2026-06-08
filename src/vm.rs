use num_enum::TryFromPrimitive;

use crate::state::State;

#[derive(Debug)]
pub struct Error;

#[derive(Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum OpCode {
    AddPlayer,
    MakeGroups,
    MakeMatches,

    Show,
    Eoc, // End Of Command
}

impl From<OpCode> for u8 {
    fn from(op: OpCode) -> Self {
        op as u8
    }
}

#[derive(Debug)]
pub struct StrRef(pub u8);

#[derive(Default, Debug)]
pub struct VM {
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
                OpCode::AddPlayer => self.add_player(),
                OpCode::MakeGroups => self.make_groups(),
                OpCode::MakeMatches => self.make_matches(),
                OpCode::Show => self.show(),
                OpCode::Eoc => return Ok(()),
            }?
        }
    }

    fn make_matches(&mut self) -> Result<(), Error> {
        self.state.make_matches();
        Ok(())
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

    fn show(&mut self) -> Result<(), Error> {
        let option = self.read_byte();

        let str = match option {
            1 => self.state.display_players(&self.strings),
            2 => self.state.display_groups(&self.strings),
            _ => return Err(Error),
        };

        println!("{str}");

        Ok(())
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.bytes[self.ip];
        self.ip += 1;

        byte
    }
}
