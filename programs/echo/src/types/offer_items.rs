use crate::{OfferItem, OFFER_ITEM_SERIALIZED_SIZE};
use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use std::io;
use std::io::{Read, Write};

#[derive(Clone)]
pub struct OfferItems {
    pub count: u8,
    pub items: Vec<OfferItem>,
}

impl AnchorSerialize for OfferItems {
    fn serialize<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write(&self.count.to_be_bytes())?;
        for item in &self.items {
            item.serialize(writer)?;
        }
        Ok(())
    }
}

impl AnchorDeserialize for OfferItems {
    fn deserialize_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;

        // Ensure at least 1 byte (for count) is read
        if data.len() < 1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "insufficient data for count",
            ));
        }

        let count = data[0];
        if count == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "count cannot be 0",
            ));
        }

        // Calculate expected data size based on count and serialized size of each item
        let expected_data_size = 1 + count as usize * OFFER_ITEM_SERIALIZED_SIZE;
        if data.len() != expected_data_size {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "wrong size: expected {} bytes but size is {}",
                    expected_data_size,
                    data.len()
                ),
            ));
        }
        // Deserialize each offer item
        let mut items = Vec::with_capacity(count as usize);
        for item_index in 0..count {
            let start = 1 + item_index as usize * OFFER_ITEM_SERIALIZED_SIZE;
            let end = start + OFFER_ITEM_SERIALIZED_SIZE;
            let item = OfferItem::try_from_slice(&data[start..end])?;
            items.push(item);
        }

        Ok(OfferItems { count, items })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::*;
    use std::io::Cursor;
    use ethereum_types::H160;
    use crate::{Address};

    #[test]
    fn serialize_deserialize_offer_items() {
        // Create some offer items
        let offer_item_1 = OfferItem {
            address: Address {
                chain_id: 1,
                solana_address: Some(Pubkey::new_unique()),
                eth_address: None,
            },
            token_id: None,
        };
        let eth_address = H160::random();
        let offer_item_2 = OfferItem {
            address: Address {
                chain_id: 2,
                solana_address: None,
                eth_address: Some(eth_address),
            },
            token_id: Some(9876543210),
        };
        let offer_items = OfferItems {
            count: 2,
            items: vec![offer_item_1, offer_item_2],
        };

        // Serialize the offer items
        let mut buffer: Vec<u8> = Vec::new();
        offer_items.serialize(&mut buffer).unwrap();

        // Deserialize the offer items from the buffer
        let mut cursor = Cursor::new(buffer);
        let deserialized_offer_items = OfferItems::deserialize_reader(&mut cursor).unwrap();

        // Ensure the deserialized offer items match the original ones
        assert_eq!(offer_items.count, deserialized_offer_items.count);
        assert_eq!(offer_items.items.len(), deserialized_offer_items.items.len());
        for i in 0..offer_items.items.len() {
            assert_eq!(offer_items.items[i].address, deserialized_offer_items.items[i].address);
            assert_eq!(offer_items.items[i].token_id, deserialized_offer_items.items[i].token_id);
        }
    }

    #[test]
    fn deserialize_invalid_offer_items_size() {
        let mut buffer = vec![0; OFFER_ITEM_SERIALIZED_SIZE * 2 - 1];
        buffer[0] = 4; // Invalid size
        let mut cursor = Cursor::new(buffer);

        // Attempt to deserialize the offer items
        let result = OfferItems::deserialize_reader(&mut cursor);

        // Ensure an error is returned due to invalid size
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_offer_items_with_empty_array() {
        // Create a buffer with zero count
        let buffer = [0; 0];
        let mut cursor = Cursor::new(&buffer[..]);

        // Attempt to deserialize the offer items
        let result = OfferItems::deserialize_reader(&mut cursor);

        // Ensure an error is returned due to zero count
        assert!(result.is_err());
    }


    #[test]
    fn deserialize_offer_items_with_zero_count() {
        let mut buffer = vec![0; 1];
        buffer[0] = 0; // Zero count
        let mut cursor = Cursor::new(buffer);

        // Attempt to deserialize the offer items
        let result = OfferItems::deserialize_reader(&mut cursor);

        // Ensure an error is returned due to zero count
        assert!(result.is_err());
    }
}
