use crate::{Address, TokenId, ADDRESS_SERIALIZED_SIZE};
use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use std::io;
use std::io::{Read, Write};

#[derive(Clone)]
pub struct OfferItem {
    pub address: Address,
    pub token_id: TokenId,
}

pub const OFFER_ITEM_SERIALIZED_SIZE: usize = ADDRESS_SERIALIZED_SIZE + 16;

impl AnchorSerialize for OfferItem {
    fn serialize<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.address.serialize(writer)?;
        match self.token_id {
            Some(id) => {
                writer.write(&id.to_be_bytes())?;
            }
            None => {
                // write 16 padding bytes
                writer.write(&[0; 16])?;
            }
        }
        Ok(())
    }
}

impl AnchorDeserialize for OfferItem {
    fn deserialize_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut data = [0; OFFER_ITEM_SERIALIZED_SIZE];
        let size = reader.read(&mut data)?;
        if size != OFFER_ITEM_SERIALIZED_SIZE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("wrong size: expected 50 bytes but size is {}", size),
            ));
        }
        let address = Address::try_from_slice(&data[0..ADDRESS_SERIALIZED_SIZE])?;
        let token_id = {
            let mut buffer = [0; 16];
            buffer.copy_from_slice(&data[ADDRESS_SERIALIZED_SIZE + 1..]);
            let id = u128::from_be_bytes(buffer);
            match id {
                0 => None,
                _ => Some(id),
            }
        };
        Ok(OfferItem { address, token_id })
    }
}
