use crate::MESSAGE_MAX_LENGTH;
use anchor_lang::{prelude::Pubkey, AnchorDeserialize, AnchorSerialize};
use std::io;
use std::io::Read;

const PAYLOAD_ID_ALIVE: u8 = 0;
const PAYLOAD_ID_OFFER_CREATED: u8 = 1;
const PAYLOAD_ID_OFFER_ACCEPTED: u8 = 2;
const PAYLOAD_ID_OFFER_CANCELED: u8 = 3;

#[derive(Clone)]
/// * `Alive`: Payload ID == 0. Emitted when [`initialize`](crate::initialize)
///  is called).
pub enum Message {
    Alive { program_id: Pubkey },
    Hello { message: Vec<u8> },
}

impl AnchorSerialize for Message {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        match self {
            Message::Alive { program_id } => {
                PAYLOAD_ID_ALIVE.serialize(writer)?;
                program_id.serialize(writer)
            }
            Message::Hello { message } => {
                if message.len() > MESSAGE_MAX_LENGTH {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("message exceeds {MESSAGE_MAX_LENGTH} bytes"),
                    ))
                } else {
                    PAYLOAD_ID_OFFER_CREATED.serialize(writer)?;
                    (message.len() as u16).to_be_bytes().serialize(writer)?;
                    for item in message {
                        item.serialize(writer)?;
                    }
                    Ok(())
                }
            }
        }
    }
}

impl AnchorDeserialize for Message {
    fn deserialize_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        let message: Vec<u8> = AnchorDeserialize::deserialize_reader(reader)?;
        match message[0] {
            PAYLOAD_ID_ALIVE => Ok(Message::Alive {
                program_id: Pubkey::try_from(&message[1..33]).unwrap(),
            }),
            PAYLOAD_ID_OFFER_CREATED => {
                let length = {
                    let mut out = [0u8; 2];
                    out.copy_from_slice(&message[1..3]);
                    u16::from_be_bytes(out) as usize
                };
                if length > MESSAGE_MAX_LENGTH {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("message exceeds {MESSAGE_MAX_LENGTH} bytes"),
                    ))
                } else {
                    Ok(Message::Hello {
                        message: message[3..(3 + length)].to_vec(),
                    })
                }
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid payload ID",
            )),
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use anchor_lang::prelude::Result;
    use std::{mem::size_of, str, string::String};

    #[test]
    fn test_message_alive() -> Result<()> {
        let my_program_id = Pubkey::new_unique();
        let msg = Message::Alive {
            program_id: my_program_id,
        };

        // Serialize program ID above.
        let mut encoded = Vec::new();
        msg.serialize(&mut encoded)?;

        assert_eq!(encoded.len(), size_of::<u8>() + size_of::<Pubkey>());

        // Verify Payload ID.
        assert_eq!(encoded[0], PAYLOAD_ID_ALIVE);

        // Verify Program ID.
        let mut program_id_bytes = [0u8; 32];
        program_id_bytes.copy_from_slice(&encoded[1..33]);
        assert_eq!(program_id_bytes, my_program_id.to_bytes());

        // Now deserialize the encoded message.
        match Message::deserialize(&mut encoded.as_slice())? {
            Message::Alive { program_id } => {
                assert_eq!(program_id, my_program_id)
            }
            _ => assert!(false, "incorrect deserialization"),
        }

        Ok(())
    }

    #[test]
    fn test_message_hello() -> Result<()> {
        let raw_message = String::from("All your base are belong to us");
        let msg = Message::Hello {
            message: raw_message.as_bytes().to_vec(),
        };

        // Serialize message above.
        let mut encoded = Vec::new();
        msg.serialize(&mut encoded)?;

        assert_eq!(
            encoded.len(),
            size_of::<u8>() + size_of::<u16>() + raw_message.len()
        );

        // Verify Payload ID.
        assert_eq!(encoded[0], PAYLOAD_ID_OFFER_CREATED);

        // Verify message length.
        let mut message_len_bytes = [0u8; 2];
        message_len_bytes.copy_from_slice(&encoded[1..3]);
        assert_eq!(
            u16::from_be_bytes(message_len_bytes) as usize,
            raw_message.len()
        );

        // Verify message.
        let from_utf8_result = str::from_utf8(&encoded[3..]);
        assert!(from_utf8_result.is_ok(), "from_utf8 resulted in an error");
        assert_eq!(from_utf8_result.unwrap(), raw_message);

        // Now deserialize the encoded message.
        match Message::deserialize(&mut encoded.as_slice())? {
            Message::Hello { message } => {
                assert_eq!(message, raw_message.as_bytes())
            }
            _ => assert!(false, "incorrect deserialization"),
        }

        Ok(())
    }

    #[test]
    fn test_message_hello_too_large() -> Result<()> {
        let n: usize = 513;
        let raw_message = {
            let mut out = Vec::with_capacity(n);
            for _ in 0..n {
                out.push(33u8)
            }
            String::from_utf8(out).unwrap()
        };
        let msg = Message::Hello {
            message: raw_message.as_bytes().to_vec(),
        };

        // Attempt to serialize message above.
        let mut encoded = Vec::new();
        match msg.serialize(&mut encoded) {
            Err(e) => assert_eq!(e.kind(), io::ErrorKind::InvalidInput),
            _ => assert!(false, "not supposed to serialize"),
        };

        // Serialize manually and then attempt to deserialize.
        encoded.push(PAYLOAD_ID_OFFER_CREATED);
        encoded.extend_from_slice(&(raw_message.len() as u16).to_be_bytes());
        encoded.extend_from_slice(raw_message.as_bytes());

        assert_eq!(
            encoded.len(),
            size_of::<u8>() + size_of::<u16>() + raw_message.len()
        );

        // Verify Payload ID.
        assert_eq!(encoded[0], PAYLOAD_ID_OFFER_CREATED);

        // Verify message length.
        let mut message_len_bytes = [0u8; 2];
        message_len_bytes.copy_from_slice(&encoded[1..3]);
        assert_eq!(
            u16::from_be_bytes(message_len_bytes) as usize,
            raw_message.len()
        );

        match Message::deserialize(&mut encoded.as_slice()) {
            Err(e) => assert_eq!(e.kind(), io::ErrorKind::InvalidInput),
            _ => assert!(false, "not supposed to deserialize"),
        };

        Ok(())
    }
}
