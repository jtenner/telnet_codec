#[derive(Debug)]
pub enum TelnetError {
  InvalidSubnegotiationSequence,
  InvalidIACSequence,
  IO(std::io::Error),
}

impl From<std::io::Error> for TelnetError {
  fn from(err: std::io::Error) -> TelnetError {
    TelnetError::IO(err)
  }
}

impl PartialEq for TelnetError {
  fn eq(&self, other: &Self) -> bool {
    match (&self, &other) {
      (TelnetError::IO(a), TelnetError::IO(b)) => a.kind() == b.kind(),
      (TelnetError::InvalidSubnegotiationSequence, TelnetError::InvalidSubnegotiationSequence) => true,
      _ => false
    }
  }
}
