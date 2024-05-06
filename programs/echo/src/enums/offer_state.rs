use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default)]
pub enum OfferState {
    #[default]
    OPEN,
    ACCEPTED,
}
