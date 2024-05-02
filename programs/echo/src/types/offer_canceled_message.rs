use crate::OfferId;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use std::io;

#[derive(Clone)]
pub struct OfferCanceledMessage {
    pub id: OfferId,
}

impl AnchorSerialize for OfferCanceledMessage {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        self.id.serialize(writer)
    }
}

impl AnchorDeserialize for OfferCanceledMessage {
    fn deserialize_reader<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let id = OfferId::deserialize_reader(reader)?;
        Ok(OfferCanceledMessage { id })
    }
}
