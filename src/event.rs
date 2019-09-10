use crate::option::*;
use crate::command::*;
use crate::subnegotiation::*;

#[derive(Debug,PartialEq)]
pub enum TelnetEvent {
  Do(TelnetOption),
  Dont(TelnetOption),
  Will(TelnetOption),
  Wont(TelnetOption),
  Subnegotiation(SubnegotiationType),
  Message(String),
  Character(u8),
  EraseCharacter,
  EraseLine,
  Nop,
}

impl Into<u8> for TelnetEvent {
  fn into(self) -> u8 {
    match self {
      TelnetEvent::Do(_) => DO,
      TelnetEvent::Dont(_) => DONT,
      TelnetEvent::Will(_) => WILL,
      TelnetEvent::Wont(_) => WONT,
      TelnetEvent::Subnegotiation(_) => SUBNEGOTIATION,
      TelnetEvent::Message(_) => 0x00,
      TelnetEvent::Character(val) => val,
      TelnetEvent::EraseCharacter => ERASE_CHARACTER,
      TelnetEvent::EraseLine => ERASE_LINE,
      TelnetEvent::Nop => NOP,
    }
  }
}
