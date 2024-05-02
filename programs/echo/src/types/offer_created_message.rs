use crate::{Address, OfferId, OfferItems, ADDRESS_SERIALIZED_SIZE, OFFER_ITEM_SERIALIZED_SIZE};
use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use std::io;
use std::io::{Read, Write};

#[derive(Clone)]
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
        // Need to add 1 because count is in the data
        end_index += data[start_index] as usize * OFFER_ITEM_SERIALIZED_SIZE + 1;
        let sender_items = OfferItems::try_from_slice(&data[start_index..end_index])?;

        start_index = end_index;
        // Need to add 1 because count is in the data
        end_index += data[start_index] as usize * OFFER_ITEM_SERIALIZED_SIZE + 1;
        let receiver_items = OfferItems::try_from_slice(&data[start_index..end_index])?;

        start_index = end_index;
        let expiration_end = start_index + 8;
        let expiration_buffer = &data[start_index..expiration_end];
        let expiration = i64::from_be_bytes(expiration_buffer.try_into().unwrap());

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use anchor_lang::prelude::*;
    use ethereum_types::H160;
    use crate::{OfferItem};

    #[test]
    fn serialize_deserialize_offer_created_message() {
        // Create test data
        let id: OfferId = [1; 32];
        let sender = Address {
            chain_id: 1,
            solana_address: Some(Pubkey::new_unique()),
            eth_address: None,
        };
        let receiver = Address {
            chain_id: 2,
            solana_address: None,
            eth_address: Some(H160::random()),
        };
        let sender_items = OfferItems {
            count: 2,
            items: vec![
                OfferItem {
                    address: sender.clone(),
                    token_id: Some(1),
                },
                OfferItem {
                    address: sender.clone(),
                    token_id: Some(2),
                },
            ],
        };
        let receiver_items = OfferItems {
            count: 1,
            items: vec![OfferItem {
                address: receiver.clone(),
                token_id: Some(3),
            }],
        };
        let expiration = 1619798400; // May 1, 2021, 12:00:00 AM UTC

        let offer_created_message = OfferCreatedMessage {
            id,
            sender,
            receiver,
            sender_items,
            receiver_items,
            expiration,
        };

        // Serialize the offer created message
        let mut serialized_data = Vec::new();
        offer_created_message.serialize(&mut serialized_data).unwrap();

        // Deserialize the offer created message
        let mut cursor = Cursor::new(&serialized_data);
        let deserialized_offer_created_message =
            OfferCreatedMessage::deserialize_reader(&mut cursor).unwrap();

        // Check if the deserialized offer created message matches the original one
        assert_eq!(offer_created_message.id, deserialized_offer_created_message.id);
        assert_eq!(offer_created_message.sender, deserialized_offer_created_message.sender);
        assert_eq!(offer_created_message.receiver, deserialized_offer_created_message.receiver);

        // Sender items
        for i in 0..offer_created_message.sender_items.items.len() {
            assert_eq!(offer_created_message.sender_items.items[i].address, deserialized_offer_created_message.sender_items.items[i].address);
            assert_eq!(offer_created_message.sender_items.items[i].token_id, deserialized_offer_created_message.sender_items.items[i].token_id);
        }

        // Receiver items
        for i in 0..offer_created_message.receiver_items.items.len() {
            assert_eq!(offer_created_message.receiver_items.items[i].address, deserialized_offer_created_message.receiver_items.items[i].address);
            assert_eq!(offer_created_message.receiver_items.items[i].token_id, deserialized_offer_created_message.receiver_items.items[i].token_id);
        }

        assert_eq!(offer_created_message.expiration, deserialized_offer_created_message.expiration);
    }
}
