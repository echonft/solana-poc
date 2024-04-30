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
        // TODO Should fail if token ID with solana chain ID
        let token_id = {
            let mut buffer = [0; 16];
            buffer.copy_from_slice(&data[ADDRESS_SERIALIZED_SIZE..]);
            let id = u128::from_be_bytes(buffer);
            match id {
                0 => None,
                _ => Some(id),
            }
        };
        Ok(OfferItem { address, token_id })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::*;
    use std::io::{Cursor};
    use ethereum_types::H160;

    #[test]
    fn test_offer_item_serialization_deserialization_solana() {
        // Create test data
        let address = Address {
            chain_id: 1,
            solana_address: Some(Pubkey::new_unique()),
            eth_address: None,
        };
        let token_id = Some(1234567890u128);
        let offer_item = OfferItem { address, token_id };

        // Serialize the offer item
        let mut buffer: Vec<u8> = Vec::new();
        offer_item.serialize(&mut buffer).unwrap();

        // Ensure the serialized buffer has the correct size
        assert_eq!(buffer.len(), OFFER_ITEM_SERIALIZED_SIZE);

        // Deserialize the offer item from the buffer
        let mut cursor = Cursor::new(buffer);
        let deserialized_offer_item = OfferItem::deserialize_reader(&mut cursor).unwrap();

        // Ensure the deserialized offer item matches the original one
        assert_eq!(deserialized_offer_item.address.chain_id, offer_item.address.chain_id);
        assert_eq!(
            deserialized_offer_item.address.solana_address,
            offer_item.address.solana_address
        );
        assert_eq!(deserialized_offer_item.address.eth_address, offer_item.address.eth_address);
        assert_eq!(deserialized_offer_item.token_id, offer_item.token_id);
    }

    #[test]
    fn test_offer_item_serialization_deserialization_solana_no_token_id() {
        // Create test data
        let address = Address {
            chain_id: 1,
            solana_address: Some(Pubkey::new_unique()),
            eth_address: None,
        };
        let token_id = None;
        let offer_item = OfferItem { address, token_id };

        // Serialize the offer item
        let mut buffer: Vec<u8> = Vec::new();
        offer_item.serialize(&mut buffer).unwrap();

        // Ensure the serialized buffer has the correct size
        assert_eq!(buffer.len(), OFFER_ITEM_SERIALIZED_SIZE);

        // Deserialize the offer item from the buffer
        let mut cursor = Cursor::new(buffer);
        let deserialized_offer_item = OfferItem::deserialize_reader(&mut cursor).unwrap();

        // Ensure the deserialized offer item matches the original one
        assert_eq!(deserialized_offer_item.address.chain_id, offer_item.address.chain_id);
        assert_eq!(
            deserialized_offer_item.address.solana_address,
            offer_item.address.solana_address
        );
        assert_eq!(deserialized_offer_item.address.eth_address, offer_item.address.eth_address);
        assert_eq!(deserialized_offer_item.token_id, offer_item.token_id);
    }

    #[test]
    fn test_offer_item_serialization_deserialization_ethereum() {
        // Create an Address instance with an Ethereum address
        let eth_address = H160::random();
        let address = Address {
            chain_id: 2,
            solana_address: None,
            eth_address: Some(eth_address),
        };

        let token_id = Some(1234567890u128);
        let offer_item = OfferItem { address, token_id };

        // Serialize the offer item
        let mut buffer: Vec<u8> = Vec::new();
        offer_item.serialize(&mut buffer).unwrap();

        // Ensure the serialized buffer has the correct size
        assert_eq!(buffer.len(), OFFER_ITEM_SERIALIZED_SIZE);

        // Deserialize the offer item from the buffer
        let mut cursor = Cursor::new(buffer);
        let deserialized_offer_item = OfferItem::deserialize_reader(&mut cursor).unwrap();

        // Ensure the deserialized offer item matches the original one
        assert_eq!(deserialized_offer_item.address.chain_id, offer_item.address.chain_id);
        assert_eq!(
            deserialized_offer_item.address.solana_address,
            offer_item.address.solana_address
        );
        assert_eq!(deserialized_offer_item.address.eth_address, offer_item.address.eth_address);
        assert_eq!(deserialized_offer_item.token_id, offer_item.token_id);
    }

    #[test]
    fn test_offer_item_deserialization_invalid_size() {
        // Create a buffer with an invalid size
        let buffer = vec![0; OFFER_ITEM_SERIALIZED_SIZE - 1];
        let mut cursor = Cursor::new(buffer);

        // Attempt to deserialize the offer item from the buffer
        let result = OfferItem::deserialize_reader(&mut cursor);

        // Ensure an error is returned due to the invalid size
        assert!(result.is_err());
    }

    #[test]
    fn test_offer_item_deserialization_invalid_address() {
        // Create a buffer with an invalid address
        let mut buffer = vec![0; OFFER_ITEM_SERIALIZED_SIZE];
        buffer[0] = 99; // Invalid chain ID
        let mut cursor = Cursor::new(buffer);

        // Attempt to deserialize the offer item from the buffer
        let result = OfferItem::deserialize_reader(&mut cursor);

        // Ensure an error is returned due to the invalid address
        assert!(result.is_err());
    }
}
