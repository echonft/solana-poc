use crate::OfferId;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use std::io;

#[derive(Clone)]
pub struct OfferAcceptedMessage {
    pub id: OfferId,
}

impl AnchorSerialize for OfferAcceptedMessage {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        self.id.serialize(writer)
    }
}

impl AnchorDeserialize for OfferAcceptedMessage {
    fn deserialize_reader<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let id = OfferId::deserialize_reader(reader)?;
        Ok(OfferAcceptedMessage { id })
    }
}
