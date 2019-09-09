pub mod command;
pub mod option;
pub mod event;
pub mod codec;
pub mod subnegotiation;
pub mod error;

#[cfg(test)]
mod tests {
    use tokio::codec::*;
    use bytes::{ BytesMut };
    use crate::codec::*;
    use crate::event::*;
    use crate::command::*;
    use crate::option::*;
    use crate::subnegotiation::*;
    use crate::error::*;

    fn consume(codec: &mut TelnetCodec, bytes: &mut BytesMut) -> Vec<Result<Option<TelnetEvent>, TelnetError>> {
        let mut result = Vec::new();
        loop {
            match codec.decode(bytes) {
                Ok(None) => { break; }
                output => result.push(output)
            }
        }
        return result;
    }

    #[test]
    fn unfinished_message() {
        let mut codec = TelnetCodec::new(4096);
        let mut bytes = BytesMut::from(b"Hello world".to_vec());
        let result = consume(&mut codec, &mut bytes);

        assert_eq!(
            result,
            vec![],
        );
    }

    #[test]
    fn finished_message() {
        let mut codec = TelnetCodec::new(4096);
        let mut bytes = BytesMut::from(b"Hello world\n".to_vec());
        let result = consume(&mut codec, &mut bytes);

        assert_eq!(
            result,
            vec![
                Ok(Some(TelnetEvent::Message(String::from("Hello world"))))
            ],
        );
    }

    #[test]
    fn iac_escape() {
        let mut codec = TelnetCodec::new(4096);
        let mut bytes = BytesMut::from(vec![
            IAC, IAC, b'a', b'b', b'c',
            b'\n',
        ]);

        let result = consume(&mut codec, &mut bytes);
        assert_eq!(
            result,
            vec![
                Ok(Some(TelnetEvent::Message(String::from("�abc")))),
            ],
        );
    }

    #[test]
    fn crlf_message() {
        let mut codec = TelnetCodec::new(4096);
        let mut bytes = BytesMut::from(b"Hello world\r\n".to_vec());
        let result = consume(&mut codec, &mut bytes);

        assert_eq!(
            result,
            vec![
                Ok(Some(TelnetEvent::Message(String::from("Hello world"))))
            ],
        );
    }

    #[test]
    fn do_message() {
        let mut codec = TelnetCodec::new(4096);
        for x in 0..=255 {
            let mut bytes = BytesMut::from(vec![IAC, DO, x]);
            let result = consume(&mut codec, &mut bytes);

            assert_eq!(
                result,
                vec![
                    Ok(Some(TelnetEvent::Do(TelnetOption::from(x))))
                ],
            );
        }
    }

    #[test]
    fn dont_message() {
        let mut codec = TelnetCodec::new(4096);

        for x in 0..=255 {
            let mut bytes = BytesMut::from(vec![IAC, DONT, x]);

            let result = consume(&mut codec, &mut bytes);
            assert_eq!(
                result,
                vec![
                    Ok(Some(TelnetEvent::Dont(TelnetOption::from(x)))),
                ],
            );
        }
    }

    #[test]
    fn will_message() {
        let mut codec = TelnetCodec::new(4096);

        for x in 0..=255 {
            let mut bytes = BytesMut::from(vec![IAC, WILL, x]);
            let result = consume(&mut codec, &mut bytes);

            assert_eq!(
                result,
                vec![
                    Ok(Some(TelnetEvent::Will(TelnetOption::from(x))))
                ],
            );
        }
    }

    #[test]
    fn wont_message() {
        let mut codec = TelnetCodec::new(4096);

        for x in 0..=255 {
            let mut bytes = BytesMut::from(vec![IAC, WONT, x]);
            let result = consume(&mut codec, &mut bytes);

            assert_eq!(
                result,
                vec![
                    Ok(Some(TelnetEvent::Wont(TelnetOption::from(x))))
                ],
            );
        }
    }

    #[test]
    fn peppered_messages() {
        let mut codec = TelnetCodec::new(4096);
        let mut bytes = BytesMut::from(vec![
            b'a',
            IAC, DO, BINARY_TRANSMISSION,
            b'b',
            IAC, WILL, NEGOTIATE_ABOUT_WINDOW_SIZE,
            b'c',
            IAC, WONT, BYTE_MACRO,
            b'\n',
        ]);
        let result = consume(&mut codec, &mut bytes);

        assert_eq!(
            result,
            vec![
                Ok(Some(TelnetEvent::Do(TelnetOption::BinaryTransmission))),
                Ok(Some(TelnetEvent::Will(TelnetOption::NegotiateAboutWindowSize))),
                Ok(Some(TelnetEvent::Wont(TelnetOption::ByteMacro))),
                Ok(Some(TelnetEvent::Message(String::from("abc")))),
            ],
        );
    }

    #[test]
    fn subnegotiation_naws() {
        let mut codec = TelnetCodec::new(4096);
        let mut bytes = BytesMut::from(vec![
            IAC, SUBNEGOTIATION, NEGOTIATE_ABOUT_WINDOW_SIZE,
            0, 100, 0, 120,
            IAC, SUBNEGOTIATION_END,
        ]);

        let result = consume(&mut codec, &mut bytes);
        assert_eq!(
            result,
            vec![
                Ok(Some(TelnetEvent::Subnegotiation(SubnegotiationType::NegotiateAboutWindowSize(100, 120)))),
            ],
        );
    }

    #[test]
    fn subnegotiation_naws_failure() {
        let mut codec = TelnetCodec::new(4096);
        let mut bytes = BytesMut::from(vec![
            IAC, SUBNEGOTIATION, NEGOTIATE_ABOUT_WINDOW_SIZE,
            0, 100, 0, 120, 0,
            IAC, SUBNEGOTIATION_END,
        ]);

        let result = consume(&mut codec, &mut bytes);
        assert_eq!(
            result,
            vec![
                Err(TelnetError::InvalidSubnegotiationSequence),
            ],
        );
    }

    #[test]
    fn subnegotiation() {
        let mut codec = TelnetCodec::new(4096);
        let mut bytes = BytesMut::from(vec![
            IAC, SUBNEGOTIATION, BINARY_TRANSMISSION,
            1, 2, 3, 4, 5,
            IAC, SUBNEGOTIATION_END,
        ]);

        let result = consume(&mut codec, &mut bytes);
        assert_eq!(
            result,
            vec![
                Ok(Some(TelnetEvent::Subnegotiation(SubnegotiationType::Other(
                    TelnetOption::BinaryTransmission,
                    vec![1, 2, 3, 4, 5],
                )))),
            ],
        );
    }

    #[test]
    fn subnegotiation_invalid_iac() {
        let mut codec = TelnetCodec::new(4096);
        let mut bytes = BytesMut::from(vec![
            IAC, SUBNEGOTIATION, BINARY_TRANSMISSION,
            1, 2, 3, 4, 5, IAC, NEGOTIATE_ABOUT_WINDOW_SIZE,
            IAC, SUBNEGOTIATION_END,
        ]);

        let result = consume(&mut codec, &mut bytes);
        assert_eq!(
            result,
            vec![
                Err(TelnetError::InvalidSubnegotiationSequence),
            ],
        );
    }

    #[test]
    fn do_encode() {
        let mut codec = TelnetCodec::new(4096);

        for x in 0..=255 {
            let mut output = BytesMut::new();
            match codec.encode(TelnetEvent::Do(TelnetOption::from(x)), &mut output) {
                Ok(()) => {
                    assert_eq!(
                        output,
                        BytesMut::from(vec![IAC, DO, x]),
                    );
                },
                Err(_) => {
                    panic!("Invalid encoding sequence");
                }
            }
        }
    }

    #[test]
    fn dont_encode() {
        let mut codec = TelnetCodec::new(4096);

        for x in 0..=255 {
            let mut output = BytesMut::new();
            match codec.encode(TelnetEvent::Dont(TelnetOption::from(x)), &mut output) {
                Ok(()) => {
                    assert_eq!(
                        output,
                        BytesMut::from(vec![IAC, DONT, x]),
                    );
                },
                Err(_) => {
                    panic!("Invalid encoding sequence");
                }
            }
        }
    }

    #[test]
    fn will_encode() {
        let mut codec = TelnetCodec::new(4096);

        for x in 0..=255 {
            let mut output = BytesMut::new();
            match codec.encode(TelnetEvent::Will(TelnetOption::from(x)), &mut output) {
                Ok(()) => {
                    assert_eq!(
                        output,
                        BytesMut::from(vec![IAC, WILL, x]),
                    );
                },
                Err(_) => {
                    panic!("Invalid encoding sequence");
                }
            }
        }
    }

    #[test]
    fn wont_encode() {
        let mut codec = TelnetCodec::new(4096);

        for x in 0..=255 {
            let mut output = BytesMut::new();
            match codec.encode(TelnetEvent::Wont(TelnetOption::from(x)), &mut output) {
                Ok(()) => {
                    assert_eq!(
                        output,
                        BytesMut::from(vec![IAC, WONT, x]),
                    );
                },
                Err(_) => {
                    panic!("Invalid encoding sequence");
                }
            }
        }
    }

    #[test]
    fn subnegotiation_naws_encode() {
        let mut codec = TelnetCodec::new(4096);

        let mut output = BytesMut::new();
        match codec.encode(
            TelnetEvent::Subnegotiation(SubnegotiationType::NegotiateAboutWindowSize(200, 200)),
            &mut output) {
            Ok(()) => {
                assert_eq!(
                    output,
                    BytesMut::from(vec![
                        IAC, SUBNEGOTIATION, NEGOTIATE_ABOUT_WINDOW_SIZE,
                        0, 200, 0, 200,
                        IAC, SUBNEGOTIATION_END,
                    ]),
                );
            },
            Err(_) => {
                panic!("Invalid encoding sequence");
            }
        }
    }

    #[test]
    fn subnegotiation_encode() {
        let mut codec = TelnetCodec::new(4096);

        let mut output = BytesMut::new();
        match codec.encode(
            TelnetEvent::Subnegotiation(SubnegotiationType::Other(
                TelnetOption::BinaryTransmission,
                vec![1, 2, 3, 4, 5, 6],
            )),
            &mut output) {
            Ok(()) => {
                assert_eq!(
                    output,
                    BytesMut::from(vec![
                        IAC, SUBNEGOTIATION, BINARY_TRANSMISSION,
                        1, 2, 3, 4, 5, 6,
                        IAC, SUBNEGOTIATION_END,
                    ]),
                );
            },
            Err(_) => {
                panic!("Invalid encoding sequence");
            }
        }
    }

    #[test]
    fn message_encode() {
        let mut codec = TelnetCodec::new(4096);

        let mut output = BytesMut::new();
        match codec.encode(
            TelnetEvent::Message(String::from("Hello world!")),
            &mut output) {
            Ok(()) => {
                assert_eq!(
                    output,
                    BytesMut::from(vec![
                        0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0x0a,
                    ]),
                );
            },
            Err(_) => {
                panic!("Invalid encoding sequence");
            }
        }
    }
}
