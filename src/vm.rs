use num_enum::TryFromPrimitive;

use crate::tournment::{Player, State};

#[derive(Debug)]
pub struct Error;

#[derive(Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum OpCode {
    AddPlayer,
    MakeGroups,

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
                OpCode::Show => self.show(),
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

    // TODO: Extract into a dedicated module to decouple the VM implementation from the interface
    fn show(&mut self) -> Result<(), Error> {
        let option = self.read_byte();

        match option {
            1 => self.show_players(),
            2 => self.show_groups(),
            _ => return Err(Error),
        }

        Ok(())
    }

    fn show_players(&self) {
        for Player { name } in self.state.iter_players() {
            let name = &self.strings[name.0 as usize];

            print!(" {name} | ");
        }
        println!();
    }

    fn show_groups(&self) {
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
