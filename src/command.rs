pub const SUBNEGOTIATION_END: u8 = 0xF0;
pub const NOP: u8                = 0xF1;
pub const DATA_MARK: u8          = 0xF2;
pub const BREAK: u8              = 0xF3;
pub const INTERRUPT_PROCESS: u8  = 0xF4;
pub const ABORT: u8              = 0xF5;
pub const ARE_YOU_THERE: u8      = 0xF6;
pub const ERASE_CHARACTER: u8    = 0xF7;
pub const ERASE_LINE: u8         = 0xF8;
pub const GO_AHEAD: u8           = 0xF9;
pub const SUBNEGOTIATION: u8     = 0xFA;
pub const WILL: u8               = 0xFB;
pub const WONT: u8               = 0xFC;
pub const DO: u8                 = 0xFD;
pub const DONT: u8               = 0xFE;
pub const IAC: u8                = 0xFF;

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
