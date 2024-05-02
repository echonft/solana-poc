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
        let count = data[0];
        if count == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "count cannot be 0",
            ));
        }
        let mut items: Vec<OfferItem> = Vec::with_capacity(count as usize);
        let data_size = 1 + count * OFFER_ITEM_SERIALIZED_SIZE as u8;
        if data.len() != data_size as usize {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "wrong size: exepected {} bytes but size is {}",
                    data_size,
                    data.len()
                ),
            ));
        }
        for item_index in 0..count {
            let item = OfferItem::try_from_slice(
                &data[1 + item_index as usize..OFFER_ITEM_SERIALIZED_SIZE * item_index as usize],
            )?;
            items.push(item)
        }
        Ok(OfferItems { count, items })
    }
}
