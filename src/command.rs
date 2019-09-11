use crate::consts::*;

#[derive(PartialEq, Debug)]
pub enum TelnetCommand {
    SubnegotiationEnd, // End of subnegotiation parameters.
    NOP              , // No operation.
    DataMark         , // The data stream portion of a Synch. This should always be accompanied by a TCP Urgent notification.
    Break            , // NVT character BRK.
    InterruptProcess , // The function IP.
    Abort            , // The function AO.
    AreYouThere      , // The function AYT.
    EraseCharacter   , // The function EC.
    EraseLine        , // The function EL.
    GoAhead          , // The GA signal.
    Subnegotiation   , // Indicates that what follows is subnegotiation of the indicated option.
    WILL             , // Indicates the want to begin performing, or confirmation that you are now performing, the indicated option.
    WONT             , // Indicates the refusal to perform, or continue performing, the indicated option.
    DO               , // Indicates the request that the other party perform, or confirmation that you are expecting the other party to perform, the indicated option.
    DONT             , // Indicates the demand that the other party stop performing, or confirmation that you are no longer expecting the other party to perform, the indicated option.
    IAC              , // IAC
    Other(u8)        ,
}

impl From<u8> for TelnetCommand {
  fn from(byte: u8) -> TelnetCommand {
    match byte {
      SUBNEGOTIATION_END => TelnetCommand::SubnegotiationEnd,
      NOP                => TelnetCommand::NOP              ,
      DATA_MARK          => TelnetCommand::DataMark         ,
      BREAK              => TelnetCommand::Break            ,
      INTERRUPT_PROCESS  => TelnetCommand::InterruptProcess ,
      ABORT              => TelnetCommand::Abort            ,
      ARE_YOU_THERE      => TelnetCommand::AreYouThere      ,
      ERASE_CHARACTER    => TelnetCommand::EraseCharacter   ,
      ERASE_LINE         => TelnetCommand::EraseLine        ,
      GO_AHEAD           => TelnetCommand::GoAhead          ,
      SUBNEGOTIATION     => TelnetCommand::Subnegotiation   ,
      WILL               => TelnetCommand::WILL             ,
      WONT               => TelnetCommand::WONT             ,
      DO                 => TelnetCommand::DO               ,
      DONT               => TelnetCommand::DONT             ,
      IAC                => TelnetCommand::IAC              ,
      _                  => TelnetCommand::Other(byte)      ,
    }
  }
}

impl Into<u8> for TelnetCommand {
  fn into(self) -> u8 {
    match self {
      TelnetCommand::SubnegotiationEnd => SUBNEGOTIATION_END,
      TelnetCommand::NOP               => NOP               ,
      TelnetCommand::DataMark          => DATA_MARK         ,
      TelnetCommand::Break             => BREAK             ,
      TelnetCommand::InterruptProcess  => INTERRUPT_PROCESS ,
      TelnetCommand::Abort             => ABORT             ,
      TelnetCommand::AreYouThere       => ARE_YOU_THERE     ,
      TelnetCommand::EraseCharacter    => ERASE_CHARACTER   ,
      TelnetCommand::EraseLine         => ERASE_LINE        ,
      TelnetCommand::GoAhead           => GO_AHEAD          ,
      TelnetCommand::Subnegotiation    => SUBNEGOTIATION    ,
      TelnetCommand::WILL              => WILL              ,
      TelnetCommand::WONT              => WONT              ,
      TelnetCommand::DO                => DO                ,
      TelnetCommand::DONT              => DONT              ,
      TelnetCommand::IAC               => IAC               ,
      TelnetCommand::Other(byte)       => byte              ,
    }
  }
}
