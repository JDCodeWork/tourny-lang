use std::ops::Index;

use num_enum::TryFromPrimitive;

use crate::{display::ShowTarget, state::State};

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

#[derive(Debug, Clone, Copy)]
pub struct StrId(u8);
impl StrId {
    pub fn new(value: u8) -> Self {
        Self(value)
    }
}

#[derive(Default, Debug)]
pub struct StrPool<'a> {
    strings: Vec<&'a str>,
}

impl<'a> Index<StrId> for StrPool<'a> {
    type Output = str;
    fn index(&self, index: StrId) -> &Self::Output {
        self.strings[index.0 as usize]
    }
}

#[derive(Default, Debug)]
pub struct VM<'a> {
    bytes: Vec<u8>,

    str_pool: StrPool<'a>,
    state: State,
    ip: usize,
}

impl<'a> VM<'a> {
    pub fn interpret(&mut self, mut code: Vec<u8>, strings: Vec<&'a str>) {
        self.bytes.append(&mut code);
        self.add_strings(strings);

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
        self.state.add_player(StrId(byte));
        Ok(())
    }

    fn show(&mut self) -> Result<(), Error> {
        let option = self.read_byte();

        let target = match option {
            1 => ShowTarget::Players,
            2 => ShowTarget::Groups,
            3 => ShowTarget::Matches,
            _ => return Err(Error),
        };

        let out = self.state.render(target, &self.str_pool);
        println!("{out}");

        Ok(())
    }

    fn add_strings(&mut self, strings: Vec<&'a str>) {
        self.str_pool.strings.extend(strings);
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.bytes[self.ip];
        self.ip += 1;

        byte
    }
}
