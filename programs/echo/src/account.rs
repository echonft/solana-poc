use crate::contexts::OfferState;
use anchor_lang::prelude::*;

#[account]
pub struct EscrowAuthority {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub offer: Pubkey,
    pub bump: u8,
}

#[account]
pub struct Offer {
    pub sender: Pubkey,
    pub receiver: Pubkey,
    pub sender_token_mint: Pubkey,
    pub receiver_token_mint: Pubkey,
    pub state: OfferState,
    // TODO add expiration date
    pub bump: u8,
}

#[account]
pub struct CrossChainOffer {
    pub offer: Offer,
    pub id: String,
}
