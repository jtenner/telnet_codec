use std::convert::From;

pub const BINARY_TRANSMISSION: u8                = 0x0 ;
pub const ECHO: u8                               = 0x1 ;
pub const RECONNECTION: u8                       = 0x2 ;
pub const SUPPRESS_GO_AHEAD: u8                  = 0x3 ;
pub const APPROX_MESSAGE_SIZE_NEGOTIATION: u8    = 0x4 ;
pub const STATUS: u8                             = 0x5 ;
pub const TIMING_MARK: u8                        = 0x6 ;
pub const REMOTE_CONTROLLED_TRANS_AND_ECHO: u8   = 0x7 ;
pub const OUTPUT_LINE_WIDTH: u8                  = 0x8 ;
pub const OUTPUT_PAGE_SIZE: u8                   = 0x9 ;
pub const OUTPUT_CARRIAGE_RETURN_DISPOSITION: u8 = 0xA ;
pub const OUTPUT_HORIZONTAL_TAB_STOPS: u8        = 0xB ;
pub const OUTPUT_HORIZONTAL_TAB_DISPOSITION: u8  = 0xC ;
pub const OUTPUT_FORM_FEED_DISPOSITION: u8       = 0xD ;
pub const OUTPUT_VERTICAL_TAB_STOPS: u8          = 0xE ;
pub const OUTPUT_VERTICAL_TAB_DISPOSITION: u8    = 0xF ;
pub const OUTPUT_LINE_FEED_DISPOSITION: u8       = 0x10;
pub const EXTENDED_ASCII: u8                     = 0x11;
pub const LOGOUT: u8                             = 0x12;
pub const BYTE_MACRO: u8                         = 0x13;
pub const DATAENTRYTERMINAL: u8                  = 0x14;
pub const SUPDUP: u8                             = 0x15;
pub const SUPDUP_OUTPUT: u8                      = 0x16;
pub const SEND_LOCATION: u8                      = 0x17;
pub const TERMINAL_TYPE: u8                      = 0x18;
pub const END_OF_RECORD: u8                      = 0x19;
pub const TACACS_USER_IDENTIFICATION: u8         = 0x1A;
pub const OUTPUT_MARKING: u8                     = 0x1B;
pub const TERMINAL_LOCATION_NUMBER: u8           = 0x1C;
pub const TELNET_3270_REGIME: u8                 = 0x1D;
pub const X3PAD: u8                              = 0x1E;
pub const NEGOTIATE_ABOUT_WINDOW_SIZE: u8        = 0x1F;
pub const TERMINAL_SPEED: u8                     = 0x20;
pub const REMOTE_FLOW_CONTROL: u8                = 0x21;
pub const LINEMODE: u8                           = 0x22;
pub const X_DISPLAY_LOCATION: u8                 = 0x23;
pub const EXTENDED_OPTIONS_LIST: u8              = 0xFF;

#[derive(PartialEq, Debug)]
pub enum TelnetOption {
    BinaryTransmission              , // switch to utf-8
    Echo                            , // echo back everything
    Reconnection                    , // support reconnection
    SuppressGoAhead                 ,
    ApproxMessageSizeNegotiation    ,
    Status                          ,
    TimingMark                      ,
    RemoteControlledTransAndEcho    ,
    OutputLineWidth                 ,
    OutputPageSize                  ,
    OutputCarriageReturnDisposition ,
    OutputHorizontalTabStops        ,
    OutputHorizontalTabDisposition  ,
    OutputFormfeedDisposition       ,
    OutputVerticalTabstops          ,
    OutputVerticalTabDisposition    ,
    OutputLinefeedDisposition       ,
    ExtendedASCII                   ,
    Logout                          ,
    ByteMacro                       ,
    DataEntryTerminal               ,
    SUPDUP                          ,
    SUPDUPOutput                    ,
    SendLocation                    ,
    TerminalType                    ,
    EndOfRecord                     ,
    TACACSUserIdentification        ,
    OutputMarking                   ,
    TerminalLocationNumber          ,
    Telnet3270Regime                ,
    X3Pad                           ,
    NegotiateAboutWindowSize        ,
    TerminalSpeed                   ,
    RemoteFlowControl               ,
    Linemode                        ,
    XDisplayLocation                ,
    ExtendedOptionsList             ,
    Other(u8)                       ,
}

impl Into<u8> for TelnetOption {
  fn into(self) -> u8 {
    match self {
      TelnetOption::BinaryTransmission              => BINARY_TRANSMISSION,
      TelnetOption::Echo                            => ECHO,
      TelnetOption::Reconnection                    => RECONNECTION,
      TelnetOption::SuppressGoAhead                 => SUPPRESS_GO_AHEAD,
      TelnetOption::ApproxMessageSizeNegotiation    => APPROX_MESSAGE_SIZE_NEGOTIATION,
      TelnetOption::Status                          => STATUS,
      TelnetOption::TimingMark                      => TIMING_MARK,
      TelnetOption::RemoteControlledTransAndEcho    => REMOTE_CONTROLLED_TRANS_AND_ECHO,
      TelnetOption::OutputLineWidth                 => OUTPUT_LINE_WIDTH,
      TelnetOption::OutputPageSize                  => OUTPUT_PAGE_SIZE,
      TelnetOption::OutputCarriageReturnDisposition => OUTPUT_CARRIAGE_RETURN_DISPOSITION,
      TelnetOption::OutputHorizontalTabStops        => OUTPUT_HORIZONTAL_TAB_STOPS,
      TelnetOption::OutputHorizontalTabDisposition  => OUTPUT_HORIZONTAL_TAB_DISPOSITION,
      TelnetOption::OutputFormfeedDisposition       => OUTPUT_FORM_FEED_DISPOSITION,
      TelnetOption::OutputVerticalTabstops          => OUTPUT_VERTICAL_TAB_STOPS,
      TelnetOption::OutputVerticalTabDisposition    => OUTPUT_VERTICAL_TAB_DISPOSITION,
      TelnetOption::OutputLinefeedDisposition       => OUTPUT_LINE_FEED_DISPOSITION,
      TelnetOption::ExtendedASCII                   => EXTENDED_ASCII,
      TelnetOption::Logout                          => LOGOUT,
      TelnetOption::ByteMacro                       => BYTE_MACRO,
      TelnetOption::DataEntryTerminal               => DATAENTRYTERMINAL,
      TelnetOption::SUPDUP                          => SUPDUP,
      TelnetOption::SUPDUPOutput                    => SUPDUP_OUTPUT,
      TelnetOption::SendLocation                    => SEND_LOCATION,
      TelnetOption::TerminalType                    => TERMINAL_TYPE,
      TelnetOption::EndOfRecord                     => END_OF_RECORD,
      TelnetOption::TACACSUserIdentification        => TACACS_USER_IDENTIFICATION,
      TelnetOption::OutputMarking                   => OUTPUT_MARKING,
      TelnetOption::TerminalLocationNumber          => TERMINAL_LOCATION_NUMBER,
      TelnetOption::Telnet3270Regime                => TELNET_3270_REGIME,
      TelnetOption::X3Pad                           => X3PAD,
      TelnetOption::NegotiateAboutWindowSize        => NEGOTIATE_ABOUT_WINDOW_SIZE,
      TelnetOption::TerminalSpeed                   => TERMINAL_SPEED,
      TelnetOption::RemoteFlowControl               => REMOTE_FLOW_CONTROL,
      TelnetOption::Linemode                        => LINEMODE,
      TelnetOption::XDisplayLocation                => X_DISPLAY_LOCATION,
      TelnetOption::ExtendedOptionsList             => EXTENDED_OPTIONS_LIST,
      TelnetOption::Other(byte)                     => byte,
    }
  }
}

impl From<u8> for TelnetOption {
  fn from(byte: u8) -> TelnetOption {
    match byte {
      BINARY_TRANSMISSION                => TelnetOption::BinaryTransmission,
      ECHO                               => TelnetOption::Echo,
      RECONNECTION                       => TelnetOption::Reconnection,
      SUPPRESS_GO_AHEAD                  => TelnetOption::SuppressGoAhead,
      APPROX_MESSAGE_SIZE_NEGOTIATION    => TelnetOption::ApproxMessageSizeNegotiation,
      STATUS                             => TelnetOption::Status,
      TIMING_MARK                        => TelnetOption::TimingMark,
      REMOTE_CONTROLLED_TRANS_AND_ECHO   => TelnetOption::RemoteControlledTransAndEcho,
      OUTPUT_LINE_WIDTH                  => TelnetOption::OutputLineWidth,
      OUTPUT_PAGE_SIZE                   => TelnetOption::OutputPageSize,
      OUTPUT_CARRIAGE_RETURN_DISPOSITION => TelnetOption::OutputCarriageReturnDisposition,
      OUTPUT_HORIZONTAL_TAB_STOPS        => TelnetOption::OutputHorizontalTabStops,
      OUTPUT_HORIZONTAL_TAB_DISPOSITION  => TelnetOption::OutputHorizontalTabDisposition,
      OUTPUT_FORM_FEED_DISPOSITION       => TelnetOption::OutputFormfeedDisposition,
      OUTPUT_VERTICAL_TAB_STOPS          => TelnetOption::OutputVerticalTabstops,
      OUTPUT_VERTICAL_TAB_DISPOSITION    => TelnetOption::OutputVerticalTabDisposition,
      OUTPUT_LINE_FEED_DISPOSITION       => TelnetOption::OutputLinefeedDisposition,
      EXTENDED_ASCII                     => TelnetOption::ExtendedASCII,
      LOGOUT                             => TelnetOption::Logout,
      BYTE_MACRO                         => TelnetOption::ByteMacro,
      DATAENTRYTERMINAL                  => TelnetOption::DataEntryTerminal,
      SUPDUP                             => TelnetOption::SUPDUP,
      SUPDUP_OUTPUT                      => TelnetOption::SUPDUPOutput,
      SEND_LOCATION                      => TelnetOption::SendLocation,
      TERMINAL_TYPE                      => TelnetOption::TerminalType,
      END_OF_RECORD                      => TelnetOption::EndOfRecord,
      TACACS_USER_IDENTIFICATION         => TelnetOption::TACACSUserIdentification,
      OUTPUT_MARKING                     => TelnetOption::OutputMarking,
      TERMINAL_LOCATION_NUMBER           => TelnetOption::TerminalLocationNumber,
      TELNET_3270_REGIME                 => TelnetOption::Telnet3270Regime,
      X3PAD                              => TelnetOption::X3Pad,
      NEGOTIATE_ABOUT_WINDOW_SIZE        => TelnetOption::NegotiateAboutWindowSize,
      TERMINAL_SPEED                     => TelnetOption::TerminalSpeed,
      REMOTE_FLOW_CONTROL                => TelnetOption::RemoteFlowControl,
      LINEMODE                           => TelnetOption::Linemode,
      X_DISPLAY_LOCATION                 => TelnetOption::XDisplayLocation,
      EXTENDED_OPTIONS_LIST              => TelnetOption::ExtendedOptionsList,
      _                                  => TelnetOption::Other(byte),
    }
  }
}
