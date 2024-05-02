use crate::OfferId;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use std::fmt::Debug;
use std::io;

#[derive(Clone, Debug)]
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

// impl fmt::Display for OfferAcceptedMessage {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "(offer_id: {})", &self.id.fmt(f))
//     }
// }
