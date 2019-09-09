use crate::option::{ TelnetOption };
use crate::subnegotiation::{ SubnegotiationType };

#[derive(Debug,PartialEq)]
pub enum TelnetEvent {
  Do(TelnetOption),
  Dont(TelnetOption),
  Will(TelnetOption),
  Wont(TelnetOption),
  Subnegotiation(SubnegotiationType),
  Message(String),
  Nop,
}