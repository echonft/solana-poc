use crate::{Address, OfferId, OfferItems, OfferState};

pub struct Offer {
    pub id: OfferId,       // 32
    pub sender: Address,   // 44
    pub receiver: Address, // 44
    pub sender_items: OfferItems,
    pub receiver_items: OfferItems,
    pub expiration: i64,
    pub state: OfferState,
}
