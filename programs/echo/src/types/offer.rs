use crate::{Address, OfferId, OfferItems, OfferState};

pub struct Offer {
    pub id: OfferId,
    pub sender: Address,
    pub receiver: Address,
    pub sender_items: OfferItems,
    pub receiver_items: OfferItems,
    pub expiration: i64,
    pub state: OfferState,
}
