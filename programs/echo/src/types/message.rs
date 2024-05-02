use crate::{OfferAcceptedMessage, OfferCanceledMessage, OfferCreatedMessage, MESSAGE_MAX_LENGTH};
use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use std::io;
use std::io::{Cursor, Read};

const PAYLOAD_ID_OFFER_CREATED: u8 = 1;
const PAYLOAD_ID_OFFER_ACCEPTED: u8 = 2;
const PAYLOAD_ID_OFFER_CANCELED: u8 = 3;

// TODO change this to a struct with payload
#[derive(Clone, Debug)]
pub enum Message {
    OfferAccepted { message: OfferAcceptedMessage },
    OfferCanceled { message: OfferCanceledMessage },
    OfferCreated { message: OfferCreatedMessage },
}

// TODO Should optimize this code to remove duplications of message length checking
impl AnchorSerialize for Message {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        match self {
            Message::OfferAccepted { message } => {
                let serialized_message = message.try_to_vec()?;
                if serialized_message.len() + 3 > MESSAGE_MAX_LENGTH {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("message exceeds {MESSAGE_MAX_LENGTH} bytes"),
                    ))
                } else {
                    PAYLOAD_ID_OFFER_ACCEPTED.serialize(writer)?;
                    (serialized_message.len() as u16)
                        .to_be_bytes()
                        .serialize(writer)?;
                    writer.write(&serialized_message)?;
                    Ok(())
                }
            }
            Message::OfferCanceled { message } => {
                let serialized_message = message.try_to_vec()?;
                if serialized_message.len() + 3 > MESSAGE_MAX_LENGTH {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("message exceeds {MESSAGE_MAX_LENGTH} bytes"),
                    ))
                } else {
                    PAYLOAD_ID_OFFER_CANCELED.serialize(writer)?;
                    (serialized_message.len() as u16)
                        .to_be_bytes()
                        .serialize(writer)?;
                    writer.write(&serialized_message)?;
                    Ok(())
                }
            }
            Message::OfferCreated { message } => {
                let serialized_message = message.try_to_vec()?;
                if serialized_message.len() + 3 > MESSAGE_MAX_LENGTH {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("message exceeds {MESSAGE_MAX_LENGTH} bytes"),
                    ))
                } else {
                    PAYLOAD_ID_OFFER_CREATED.serialize(writer)?;
                    (serialized_message.len() as u16)
                        .to_be_bytes()
                        .serialize(writer)?;
                    writer.write(&serialized_message)?;
                    Ok(())
                }
            }
        }
    }
}

impl AnchorDeserialize for Message {
    fn deserialize_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut message = Vec::new();
        reader.read_to_end(&mut message)?;
        let payload = message[0];
        const MESSAGE_START_INDEX: usize = 3;
        let length = {
            let mut buffer = [0u8; 2];
            buffer.copy_from_slice(&message[1..MESSAGE_START_INDEX]);
            let received_length = u16::from_be_bytes(buffer) as usize;
            if received_length > MESSAGE_MAX_LENGTH {
                Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("message exceeds {MESSAGE_MAX_LENGTH} bytes"),
                ))?
            }
            received_length
        };

        let mut message_reader: Cursor<Vec<u8>> = {
            let mut buffer = Vec::with_capacity(length);
            buffer.copy_from_slice(&message[3..(3 + length)]);
            Cursor::new(buffer)
        };
        match payload {
            PAYLOAD_ID_OFFER_CREATED => {
                let message = OfferCreatedMessage::deserialize_reader(&mut message_reader)?;
                Ok(Message::OfferCreated { message })
            }
            PAYLOAD_ID_OFFER_ACCEPTED => {
                let message = OfferAcceptedMessage::deserialize_reader(&mut message_reader)?;
                Ok(Message::OfferAccepted { message })
            }
            PAYLOAD_ID_OFFER_CANCELED => {
                let message = OfferCanceledMessage::deserialize_reader(&mut message_reader)?;
                Ok(Message::OfferCanceled { message })
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid payload ID",
            )),
        }
    }
}
