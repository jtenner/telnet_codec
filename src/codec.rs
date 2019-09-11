use tokio::codec::{ Decoder, Encoder };
use bytes::{ BufMut, BytesMut };
use crate::event::*;
use crate::subnegotiation::*;
use crate::error::*;
use crate::consts::*;
use std::mem;

pub struct TelnetCodec {
  pub sga: bool,
  max_buffer_length: usize,
  buffer: Vec<u8>,
}

impl TelnetCodec {
  pub fn new(max_buffer_length: usize) -> TelnetCodec {
    TelnetCodec {
      sga: false,
      max_buffer_length,
      buffer: Vec::new(),
    }
  }
}

impl Encoder for TelnetCodec {
  type Item = TelnetEvent;
  type Error = TelnetError;

  fn encode(&mut self, event: TelnetEvent, buf: &mut BytesMut) -> Result<(), Self::Error> {
      match event {
        // basic commands are IAC (COMMAND) (OPT)
        TelnetEvent::Do(opt) => {
          buf.reserve(3);
          buf.put(IAC);
          buf.put(DO);
          buf.put::<u8>(opt.into());
        },
        TelnetEvent::Dont(opt) => {
          buf.reserve(3);
          buf.put(IAC);
          buf.put(DONT);
          buf.put::<u8>(opt.into());
        },
        TelnetEvent::Will(opt) => {
          buf.reserve(3);
          buf.put(IAC);
          buf.put(WILL);
          buf.put::<u8>(opt.into());
        },
        TelnetEvent::Wont(opt) => {
          buf.reserve(3);
          buf.put(IAC);
          buf.put(WONT);
          buf.put::<u8>(opt.into());
        },
        TelnetEvent::Subnegotiation(subnegotation) => {
          match subnegotation {
            SubnegotiationType::NegotiateAboutWindowSize(width, height) => {
              buf.reserve(9);
              buf.put(IAC);
              buf.put(SUBNEGOTIATION);
              buf.put(NEGOTIATE_ABOUT_WINDOW_SIZE);
              buf.put_u16_be(width);
              buf.put_u16_be(height);
              buf.put(IAC);
              buf.put(SUBNEGOTIATION_END);
            },
            SubnegotiationType::Other(opt, bytes) => {
              let mut len = 5_usize + bytes.len();

              // scan the bytes and add one for each escape sequence
              for x in &bytes {
                if *x == IAC { len += 1; }
              }

              // reserve the exact amount of space for the subnegotiation
              buf.reserve(len);

              // IAC SUB Opt
              buf.put(IAC);
              buf.put(SUBNEGOTIATION);
              buf.put::<u8>(opt.into());

              // write the bytes...
              for x in &bytes {
                if *x == IAC {
                  // if 255, escape it
                  buf.put(IAC);
                  buf.put(IAC);
                } else {
                  buf.put(*x);
                }
              }

              // IAC SUBEND
              buf.put(IAC);
              buf.put(SUBNEGOTIATION_END);
            }
          };
        },
        TelnetEvent::Message(msg) => {
          // message is utf8
          let bytes = msg.into_bytes();

          // reserve the proper bytelength
          let mut len = bytes.len();
          for x in &bytes {
            if *x == IAC { len += 1; }
          }

          // now we know the length of the message
          buf.reserve(len);

          // write the bytes to the buffer
          for x in &bytes {
            if *x == IAC {
              // escape 255
              buf.put(IAC);
              buf.put(IAC);
            } else {
              buf.put(*x);
            }
          }

          if !buf.ends_with(b"\r\n") {
            if buf.ends_with(b"\r") {
              buf.reserve(1);
              buf.put(b'\n');
            } else {
              buf.reserve(2);
              buf.put(b'\r');
              buf.put(b'\n');
            }
          }
        }
        _ => {
          // Nops can happen, ignore them
        },
      }

      Ok(())
  }
}

impl Decoder for TelnetCodec {
  type Item = TelnetEvent;
  type Error = TelnetError;

  fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
    let mut x = 0;
    let len = src.len();
    let mut buffer_len = self.buffer.len();
    let max_buffer_length = self.max_buffer_length;

    if self.sga {
      if self.buffer.len() > 0 {
        // truncate the buffer into a message and emit it
        let buffer = mem::replace(&mut self.buffer, Vec::new());
        let result = String::from_utf8_lossy(&buffer[..]);
        return Ok(Some(TelnetEvent::Message(result.to_string())));
      }
    }

    if len == 0 { return Ok(None); }

    if self.sga {
      let mut byte = src[0];

      match byte {
        IAC => {
          // check the length first
          if 1 >= len {
            return Ok(None);
          }

          // get the next byte
          byte = src[x + 1];

          match byte {
            IAC => {
              src.split_to(2);
              return Ok(Some(TelnetEvent::Character(IAC)));
            },
            ERASE_CHARACTER => {
              src.split_to(2);
              return Ok(Some(TelnetEvent::EraseCharacter));
            },
            ERASE_LINE => {
              src.split_to(2);
              return Ok(Some(TelnetEvent::EraseLine));
            },
            _ => {
              src.split_to(2);
              return Err(TelnetError::InvalidIACSequence);
            }
          }
        },
        _ => {
          src.split_to(1);
          return Ok(Some(TelnetEvent::Character(byte)));
        }
      }
    }

    loop {
      if x >= len { return Ok(None); }
      let mut byte = src[x];
      match byte {
        // parse the IAC
        IAC => {
          // check the length first
          if x + 1 >= len {
            return Ok(None);
          }

          // get the next byte
          byte = src[x + 1];
          match byte {
            ERASE_LINE => {
              mem::replace(&mut self.buffer, Vec::new());
              x += 1;
            }
            ERASE_CHARACTER => {
              self.buffer.pop();
              if buffer_len != 0 { // prevent underflow
                buffer_len -= 1;
              }
              x += 1;
            },
            IAC => {
              if buffer_len < max_buffer_length {
                self.buffer.push(IAC);
                buffer_len += 1;
              }
              x += 1;
            },
            DO => {
              if x + 2 >= len {
                return Ok(None);
              }
              // split the buffer at the current index.
              let option = src[x + 2];
              src.split_to(x + 3);
              return Ok(Some(TelnetEvent::Do(option.into())))
            },
            DONT => {
              if x + 2 >= len {
                return Ok(None);
              }
              // split the buffer at the current index.
              let option = src[x + 2];
              src.split_to(x + 3);
              return Ok(Some(TelnetEvent::Dont(option.into())))
            },
            WILL => {
              if x + 2 >= len {
                return Ok(None);
              }
              // split the buffer at the current index.
              let option = src[x + 2];
              src.split_to(x + 3);
              return Ok(Some(TelnetEvent::Will(option.into())))
            },
            WONT => {
              if x + 2 >= len {
                return Ok(None);
              }
              // split the buffer at the current index.
              let option = src[x + 2];
              src.split_to(x + 3);
              return Ok(Some(TelnetEvent::Wont(option.into())))
            },
            SUBNEGOTIATION => {
              // parse a subnegotiation
              if x + 2 >= len {
                // io bytes may have been consumed at this point, so truncate them
                src.split_to(x + 2);
                return Ok(None);
              }
              let start_index = x;
              let option = src[x + 2];
              x += 3;
              let mut subvec: Vec<u8> = Vec::new();

              let mut invalid = false;
              // loop until IAC ENDSUB
              loop {
                // we need more data to parse the subnegotation
                if x > len {
                  // if the subnegotiation parse is incomplete, trucate to the start of the sub
                  src.split_to(start_index);
                  return Ok(None);
                }
                byte = src[x];
                match byte {
                  IAC => {
                    if x + 1 > len {
                      return Ok(None); // need to read the next byte
                    }

                    // check the byte for subnegotiation end
                    byte = src[x + 1];
                    match byte {
                      SUBNEGOTIATION_END => {
                        src.split_to(x + 2);
                        if invalid {
                          // continue parsing even though the subnegotiation failed
                          return Err(TelnetError::InvalidSubnegotiationSequence);
                        } else {
                          match option {
                            NEGOTIATE_ABOUT_WINDOW_SIZE => {
                              // Assert that there are 4 bytes in the vec
                              match subvec.len() {
                                4 => {
                                  let result = SubnegotiationType::NegotiateAboutWindowSize(
                                    ((subvec[0] as u16) << 8) | (subvec[1] as u16), // big endian width
                                    ((subvec[2] as u16) << 8) | (subvec[3] as u16), // big endian height
                                  );
                                  return Ok(Some(TelnetEvent::Subnegotiation(result)));
                                },
                                _ => return Err(TelnetError::InvalidSubnegotiationSequence) // Invalid
                              }
                            },
                            _ => {
                              let result = SubnegotiationType::Other(option.into(), subvec);
                              return Ok(Some(TelnetEvent::Subnegotiation(result)));
                            }
                          }
                        }
                      },
                      IAC => {
                        subvec.push(IAC);
                      },
                      _ => {
                        // invalid sequence
                        invalid = true;
                      },
                    }
                    x += 1;
                  },
                  _ => {
                    subvec.push(byte);
                  },
                }
                x += 1;
              }
            },
            NOP => {
              x += 1;
            },
            _ => {
              // TODO: What happens here?
            },
          }
        },
        b'\n' => {
          // newline hit, truncate the buffer and emit the bytes as a utf8 string
          let mut buffer = mem::replace(&mut self.buffer, Vec::new());
          // it could have ended with crlf
          if buffer.ends_with(&[b'\r']) {
            buffer.pop();
            src.split_to(x + 1);

            let result = String::from_utf8_lossy(&buffer[..]);
            return Ok(Some(TelnetEvent::Message(result.to_string())));
          }

          // default byte action:
          // if the buffer has reached max buffer length, drop the byte
          if buffer_len < max_buffer_length {
            self.buffer.push(byte);
            buffer_len += 1;
          }
        },
        _ => {
          // default byte action:
          // if the buffer has reached max buffer length, drop the byte
          if buffer_len < max_buffer_length {
            self.buffer.push(byte);
            buffer_len += 1;
          }
        }
      }
      x += 1;
    }
  }
}
