use crate::option::{ TelnetOption };

#[derive(Debug,PartialEq)]
pub enum SubnegotiationType {
  NegotiateAboutWindowSize(u16, u16),
  // TODO: Implement more
  Other(TelnetOption, Vec<u8>),
}
