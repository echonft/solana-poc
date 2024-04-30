use crate::{Address, OfferId, OfferItems, ADDRESS_SERIALIZED_SIZE, OFFER_ITEM_SERIALIZED_SIZE};
use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use std::io;
use std::io::{Read, Write};

pub struct OfferCreatedMessage {
    pub id: OfferId,
    pub sender: Address,
    pub receiver: Address,
    pub sender_items: OfferItems,
    pub receiver_items: OfferItems,
    pub expiration: i64,
}

impl AnchorSerialize for OfferCreatedMessage {
    fn serialize<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write(&self.id)?;
        self.sender.serialize(writer)?;
        self.receiver.serialize(writer)?;
        self.sender_items.serialize(writer)?;
        self.receiver_items.serialize(writer)?;
        writer.write(&self.expiration.to_be_bytes())?;
        Ok(())
    }
}

// TODO better code
impl AnchorDeserialize for OfferCreatedMessage {
    fn deserialize_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        let mut start_index = 0;
        let mut end_index = 32;
        let mut id = [0; 32];
        id.copy_from_slice(&data[start_index..end_index]);
        start_index = end_index;
        end_index += ADDRESS_SERIALIZED_SIZE;
        let sender = Address::try_from_slice(&data[start_index..end_index])?;
        start_index = end_index;
        end_index += ADDRESS_SERIALIZED_SIZE;
        let receiver = Address::try_from_slice(&data[start_index..end_index])?;
        start_index = end_index;
        end_index += data[start_index] as usize * OFFER_ITEM_SERIALIZED_SIZE;
        let sender_items = OfferItems::try_from_slice(&data[start_index..end_index])?;
        start_index = end_index;
        end_index += data[start_index] as usize * OFFER_ITEM_SERIALIZED_SIZE;
        let receiver_items = OfferItems::try_from_slice(&data[start_index..end_index])?;
        start_index = end_index;
        let mut expiration_buffer = [0; 8];
        expiration_buffer.copy_from_slice(&data[start_index..]);
        let expiration = i64::from_be_bytes(expiration_buffer);
        Ok(OfferCreatedMessage {
            id,
            sender,
            receiver,
            sender_items,
            receiver_items,
            expiration,
        })
    }
}
